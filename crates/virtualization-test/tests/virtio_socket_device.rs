use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;

use virtualization::{
  VirtioSocketConnection, VirtioSocketDevice, VirtioSocketListener, VirtioSocketListenerDelegate, VirtualMachine,
};
use virtualization_test::{
  add_socket_device, booting_machine, connect, connect_once_listening, initial_ramdisk, initial_ramdisk_with_programs,
  linux_configuration, ns_error, on_queue, serial_queue, socket_device, start, stream, vsock_connecting,
  vsock_listening,
};

/// The port the guest listens on, or connects to.
const PORT: u32 = 1234;

/// Runs `check` on the socket device of a machine with one, made on its queue
/// and not started.
fn with_device(name: &str, check: impl FnOnce(&VirtioSocketDevice) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    add_socket_device(&configuration);

    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.socket_devices::<VirtioSocketDevice>();

    assert_eq!(devices.len(), 1);
    check(devices[0].as_ref().unwrap());
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_device("socket_device_a_clone_is_the_same_object", |device| {
    assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_device("socket_device_debugs_as_its_description", |device| {
    assert!(format!("{device:?}").starts_with("<VZVirtioSocketDevice: 0x"));
  });
}

/// A guest that listens on `PORT` once it says it will, and sends back what
/// it is sent.
const ECHO: &str = "echo listening\nvsock-listen /bin/cat";

#[test]
fn connects_to_a_port_the_guest_listens_on() {
  let name = "connects_to_a_port_the_guest_listens_on";
  let ramdisk = initial_ramdisk_with_programs(name, ECHO, &[("vsock-listen", &vsock_listening(PORT))]);
  let (queue, machine, mut console) = booting_machine(name, Some(&ramdisk), add_socket_device);

  start(&queue, &machine);
  console.read_until("listening");

  let mut stream = connect_once_listening(&queue, &machine, PORT, stream);
  let mut echoed = String::new();

  writeln!(stream, "ping").unwrap();
  BufReader::new(stream).read_line(&mut echoed).unwrap();

  assert_eq!(echoed, "ping\n");
}

/// A guest that says it is ready, then waits.
const READY: &str = "echo ready\nsleep 60";

#[test]
fn fails_to_connect_as_virtualization_does_to_a_port_the_guest_does_not_listen_on() {
  let name = "fails_to_connect_as_virtualization_does_to_a_port_the_guest_does_not_listen_on";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, READY)), add_socket_device);

  start(&queue, &machine);
  console.read_until("ready");

  let error = ns_error(connect(&queue, &machine, PORT, |_| ()));

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 54, "{error:#?}");
}

/// Accepts every connection.
struct Accepts;

impl VirtioSocketListenerDelegate for Accepts {
  fn listener_should_accept_new_connection_from(
    &self,
    _listener: &VirtioSocketListener,
    _connection: &VirtioSocketConnection,
    _socket_device: &VirtioSocketDevice,
  ) -> bool {
    true
  }
}

/// Reports whether the guest connects to `PORT`: 0 if it does.
const CONNECT: &str = "vsock-connect /bin/true\necho \"connect:$?\"";

/// What a guest reports with `CONNECT`, after `listen` on its machine's socket
/// device, with a listener that accepts every connection.
fn connects(name: &str, listen: impl FnOnce(&VirtioSocketDevice, &VirtioSocketListener) + Send) -> String {
  let ramdisk = initial_ramdisk_with_programs(name, CONNECT, &[("vsock-connect", &vsock_connecting(PORT))]);
  let (queue, machine, mut console) = booting_machine(name, Some(&ramdisk), add_socket_device);
  let delegate: Arc<dyn VirtioSocketListenerDelegate> = Arc::new(Accepts);

  on_queue(&queue, || {
    let listener = VirtioSocketListener::new().unwrap();

    listener.set_delegate(Some(&delegate));
    listen(&socket_device(&machine), &listener);
  });
  start(&queue, &machine);

  console.read_until("connect:").trim_end().to_string()
}

#[test]
fn a_guest_connects_to_a_port_with_a_listener_which_the_device_keeps() {
  let connected = connects("a_guest_connects_to_a_port_with_a_listener", |device, listener| {
    device.set_socket_listener(listener, PORT);
  });

  assert_eq!(connected, "connect:0");
}

#[test]
fn a_guest_does_not_connect_to_a_port_with_no_listener() {
  let connected = connects(
    "a_guest_does_not_connect_to_a_port_with_no_listener",
    |device, listener| {
      device.set_socket_listener(listener, PORT + 1);
    },
  );

  assert_eq!(connected, "connect:1");
}

#[test]
fn a_guest_does_not_connect_to_a_port_whose_listener_was_removed() {
  let name = "a_guest_does_not_connect_to_a_port_whose_listener_was_removed";
  let connected = connects(name, |device, listener| {
    device.set_socket_listener(listener, PORT);
    device.remove_socket_listener(PORT);
  });

  assert_eq!(connected, "connect:1");
}

#[test]
fn removing_a_listener_from_a_port_without_one_does_nothing() {
  let name = "removing_a_listener_from_a_port_without_one_does_nothing";
  let connected = connects(name, |device, listener| {
    device.set_socket_listener(listener, PORT);
    device.remove_socket_listener(PORT + 1);
  });

  assert_eq!(connected, "connect:0");
}
