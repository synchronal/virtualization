use virtualization::VirtioSocketConnection;
use virtualization_test::{
  add_socket_device, booting_machine, connect_once_listening, initial_ramdisk_with_programs, start, vsock_listening,
};

/// The port the guest listens on.
const PORT: u32 = 1234;

/// A guest that listens on `PORT` once it says it will, and sends back what
/// it is sent.
const ECHO: &str = "echo listening\nvsock-listen /bin/cat";

/// What `inspect` makes of a connection to a guest with `ECHO`, on its
/// machine's queue.
fn connected<T: Send + 'static>(name: &str, inspect: fn(&VirtioSocketConnection) -> T) -> T {
  let ramdisk = initial_ramdisk_with_programs(name, ECHO, &[("vsock-listen", &vsock_listening(PORT))]);
  let (queue, machine, mut console) = booting_machine(name, Some(&ramdisk), add_socket_device);

  start(&queue, &machine);
  console.read_until("listening");

  connect_once_listening(&queue, &machine, PORT, inspect)
}

#[test]
fn has_the_port_it_connected_to_as_its_destination() {
  let (destination_port, source_port) = connected("has_the_port_it_connected_to_as_its_destination", |connection| {
    (connection.destination_port(), connection.source_port())
  });

  assert_eq!(destination_port, PORT);
  assert_ne!(source_port, PORT);
}

#[test]
fn has_no_file_descriptor_once_closed() {
  let (open, closed) = connected("has_no_file_descriptor_once_closed", |connection| {
    let open = connection.file_descriptor();

    connection.close();

    (open, connection.file_descriptor())
  });

  assert!(open >= 0, "{open}");
  assert_eq!(closed, -1);
}

#[test]
fn a_clone_is_the_same_object() {
  let closed = connected("socket_connection_a_clone_is_the_same_object", |connection| {
    connection.clone().close();

    connection.file_descriptor()
  });

  assert_eq!(closed, -1);
}

#[test]
fn debugs_as_its_description() {
  let description = connected("socket_connection_debugs_as_its_description", |connection| {
    format!("{connection:?}")
  });

  assert!(
    description.starts_with("<VZVirtioSocketConnection: 0x"),
    "{description}"
  );
}
