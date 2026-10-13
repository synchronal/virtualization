use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use virtualization::{VirtioSocketConnection, VirtioSocketDevice, VirtioSocketListener, VirtioSocketListenerDelegate};
use virtualization_test::{
  add_socket_device, booting_machine, initial_ramdisk_with_programs, on_queue, socket_device, start, stream,
  vsock_connecting,
};

/// The port the guest connects to.
const PORT: u32 = 1234;

struct Delegate;

impl VirtioSocketListenerDelegate for Delegate {}

#[test]
fn has_the_delegate_it_was_given() {
  let listener = VirtioSocketListener::new().unwrap();
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Delegate);

  assert!(listener.delegate().is_none());

  listener.set_delegate(Some(&delegate));

  assert!(Arc::ptr_eq(&listener.delegate().unwrap(), &delegate));

  listener.set_delegate(None);

  assert!(listener.delegate().is_none());
}

#[test]
fn holds_its_delegate_weakly() {
  let listener = VirtioSocketListener::new().unwrap();
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Delegate);

  listener.set_delegate(Some(&delegate));
  drop(delegate);

  assert!(listener.delegate().is_none());
}

#[test]
fn a_clone_is_the_same_object() {
  let listener = VirtioSocketListener::new().unwrap();
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Delegate);

  listener.clone().set_delegate(Some(&delegate));

  assert!(Arc::ptr_eq(&listener.delegate().unwrap(), &delegate));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", VirtioSocketListener::new().unwrap()).starts_with("<VZVirtioSocketListener: 0x"));
}

/// Reports whether the guest connects to `PORT`: 0 if it does, having said
/// hello.
const CONNECT: &str = "vsock-connect /bin/echo hello\necho \"connect:$?\"";

/// What a guest reports with `CONNECT`, to a listener on `PORT` with
/// `delegate`, then that listener and its device, each as its description.
fn connects(name: &str, delegate: Option<&Arc<dyn VirtioSocketListenerDelegate>>) -> (String, String, String) {
  let ramdisk = initial_ramdisk_with_programs(name, CONNECT, &[("vsock-connect", &vsock_connecting(PORT))]);
  let (queue, machine, mut console) = booting_machine(name, Some(&ramdisk), add_socket_device);

  let (listener, device) = on_queue(&queue, || {
    let listener = VirtioSocketListener::new().unwrap();
    let device = socket_device(&machine);

    listener.set_delegate(delegate);
    device.set_socket_listener(&listener, PORT);

    (format!("{listener:?}"), format!("{device:?}"))
  });
  start(&queue, &machine);

  (console.read_until("connect:").trim_end().to_string(), listener, device)
}

/// What its delegate is asked about a connection: the listener and the
/// device, each as its description, and the connection.
struct Asked {
  listener: String,
  socket_device: String,
  destination_port: u32,
  stream: File,
}

/// Accepts every connection, and tells its sender what it was asked.
struct Accepts(Mutex<Sender<Asked>>);

impl VirtioSocketListenerDelegate for Accepts {
  fn listener_should_accept_new_connection_from(
    &self,
    listener: &VirtioSocketListener,
    connection: &VirtioSocketConnection,
    socket_device: &VirtioSocketDevice,
  ) -> bool {
    let _ = self.0.lock().unwrap().send(Asked {
      listener: format!("{listener:?}"),
      socket_device: format!("{socket_device:?}"),
      destination_port: connection.destination_port(),
      stream: stream(connection),
    });

    true
  }
}

#[test]
fn asks_its_delegate_about_a_connection_from_the_guest_and_accepts_it() {
  let (sender, receiver) = mpsc::channel();
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Accepts(Mutex::new(sender)));
  let (connected, listener, device) = connects("asks_its_delegate_about_a_connection_from_the_guest", Some(&delegate));
  let asked = receiver.recv().unwrap();
  let mut said = String::new();

  BufReader::new(asked.stream).read_line(&mut said).unwrap();

  assert_eq!(connected, "connect:0");
  assert_eq!(asked.listener, listener);
  assert_eq!(asked.socket_device, device);
  assert_eq!(asked.destination_port, PORT);
  assert_eq!(said, "hello\n");
}

/// Refuses every connection.
struct Refuses;

impl VirtioSocketListenerDelegate for Refuses {
  fn listener_should_accept_new_connection_from(
    &self,
    _listener: &VirtioSocketListener,
    _connection: &VirtioSocketConnection,
    _socket_device: &VirtioSocketDevice,
  ) -> bool {
    false
  }
}

#[test]
fn a_guest_does_not_connect_when_its_delegate_refuses() {
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Refuses);

  assert_eq!(
    connects("a_guest_does_not_connect_when_its_delegate_refuses", Some(&delegate)).0,
    "connect:1"
  );
}

#[test]
fn a_guest_does_not_connect_when_its_delegate_does_not_say() {
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Delegate);

  assert_eq!(
    connects(
      "a_guest_does_not_connect_when_its_delegate_does_not_say",
      Some(&delegate)
    )
    .0,
    "connect:1"
  );
}

#[test]
fn a_guest_does_not_connect_to_a_listener_with_no_delegate() {
  assert_eq!(
    connects("a_guest_does_not_connect_to_a_listener_with_no_delegate", None).0,
    "connect:1"
  );
}
