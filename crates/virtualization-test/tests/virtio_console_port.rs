use std::io::Write;

use virtualization::{
  ConsolePortConfiguration, FileHandleSerialPortAttachment, VirtioConsoleDevice, VirtioConsoleDeviceConfiguration,
  VirtioConsolePort, VirtioConsolePortConfiguration, VirtualMachine, VirtualMachineConfiguration,
};
use virtualization_test::{
  Console, booting_machine, console_attachment, initial_ramdisk, linux_configuration, on_queue, serial_queue, start,
};

/// Gives `configuration` a console device whose port 0 is named `named`,
/// with `attachment`.
fn add_port(configuration: &VirtualMachineConfiguration, attachment: &FileHandleSerialPortAttachment) {
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some("named"));
  port.set_attachment(Some(attachment));
  device.ports().set(0, Some(&port));
  configuration.set_console_devices(&[&device]);
}

/// The port `add_port` added.
fn port(machine: &VirtualMachine) -> VirtioConsolePort {
  machine.console_devices::<VirtioConsoleDevice>()[0]
    .as_ref()
    .unwrap()
    .ports()
    .get(0)
    .unwrap()
}

/// Runs `check` on the port of a machine made with `add_port` and
/// `attachment`, on its queue and not started.
fn with_port(name: &str, check: impl FnOnce(&VirtioConsolePort, &FileHandleSerialPortAttachment) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);
    let attachment = FileHandleSerialPortAttachment::new(None, None).unwrap();

    add_port(&configuration, &attachment);
    check(
      &port(&VirtualMachine::with_queue(&configuration, &queue).unwrap()),
      &attachment,
    );
  });
}

#[test]
fn has_the_name_of_its_configuration() {
  with_port("console_port_has_the_name_of_its_configuration", |port, _| {
    assert_eq!(port.name().as_deref(), Some("named"));
  });
}

#[test]
fn has_the_attachment_of_its_configuration() {
  with_port(
    "console_port_has_the_attachment_of_its_configuration",
    |port, attachment| {
      assert_eq!(
        format!("{:?}", port.attachment::<FileHandleSerialPortAttachment>().unwrap()),
        format!("{attachment:?}")
      );
    },
  );
}

#[test]
fn keeps_its_attachment() {
  with_port("console_port_keeps_its_attachment", |port, _| {
    let attachment = FileHandleSerialPortAttachment::new(None, None).unwrap();

    port.set_attachment(Some(&attachment));

    assert_eq!(
      format!("{:?}", port.attachment::<FileHandleSerialPortAttachment>().unwrap()),
      format!("{attachment:?}")
    );
  });
}

#[test]
fn loses_its_attachment() {
  with_port("console_port_loses_its_attachment", |port, _| {
    port.set_attachment(None);

    assert!(
      port
        .attachment::<FileHandleSerialPortAttachment>()
        .is_none()
    );
  });
}

/// Writes each line the console reads to the port named `named`.
const FORWARD: &str = "for port in /sys/class/virtio-ports/*; do [ \"$(cat $port/name)\" = named ] && \
                       device=/dev/$(basename $port); done\necho ready\nwhile read line; do echo \"port:$line\" > \
                       $device; done";

/// Has the guest write `line` to the port, and reads it from `port_console`.
fn forwarded(console: &mut Console, port_console: &mut Console, line: &str) -> String {
  writeln!(console.input, "{line}").unwrap();
  port_console.read_until("port:")
}

#[test]
fn changes_its_attachment_while_running() {
  let name = "console_port_changes_its_attachment_while_running";
  let mut first = None;
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, FORWARD)), |configuration| {
    let (attachment, host) = console_attachment();

    add_port(configuration, &attachment);
    first = Some(host);
  });
  let mut first = first.unwrap();

  start(&queue, &machine);
  console.read_until("ready");

  assert_eq!(forwarded(&mut console, &mut first, "one"), "port:one");

  let mut second = on_queue(&queue, || {
    let (attachment, host) = console_attachment();

    port(&machine).set_attachment(Some(&attachment));
    host
  });

  assert_eq!(forwarded(&mut console, &mut second, "two"), "port:two");
}

#[test]
fn a_clone_is_the_same_object() {
  with_port("console_port_a_clone_is_the_same_object", |port, _| {
    port.clone().set_attachment(None);

    assert!(
      port
        .attachment::<FileHandleSerialPortAttachment>()
        .is_none()
    );
  });
}

#[test]
fn debugs_as_its_description() {
  with_port("console_port_debugs_as_its_description", |port, _| {
    assert!(format!("{port:?}").starts_with("<VZVirtioConsolePort: 0x"));
  });
}
