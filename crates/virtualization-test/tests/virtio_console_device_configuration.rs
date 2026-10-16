use std::io::Write;

use virtualization::{ConsolePortConfiguration, VirtioConsoleDeviceConfiguration, VirtioConsolePortConfiguration};
use virtualization_test::{booting_machine, console_attachment, initial_ramdisk, linux_configuration, ns_error, start};

#[test]
fn is_a_console_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_console_device_of_the_configuration_it_was_given_to");

  configuration.set_console_devices(&[&VirtioConsoleDeviceConfiguration::new().unwrap()]);

  let given = configuration.console_devices::<VirtioConsoleDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert!(given[0].is_some());
}

#[test]
fn validates_with_a_port() {
  let configuration = linux_configuration("console_device_validates_with_a_port");
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();

  device
    .ports()
    .set(0, Some(&VirtioConsolePortConfiguration::new().unwrap()));
  configuration.set_console_devices(&[&device]);

  configuration.validate().unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_with_no_ports() {
  let configuration = linux_configuration("console_device_fails_validation_with_no_ports");

  configuration.set_console_devices(&[&VirtioConsoleDeviceConfiguration::new().unwrap()]);

  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();

  device
    .clone()
    .ports()
    .set(0, Some(&VirtioConsolePortConfiguration::new().unwrap()));

  assert!(device.ports().get(0).is_some());
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioConsoleDeviceConfiguration::new().unwrap())
      .starts_with("<VZVirtioConsoleDeviceConfiguration: 0x")
  );
}

/// Reports each virtio port the guest has, by its device and name.
const PORTS: &str = "for port in /sys/class/virtio-ports/*; do echo \"port:$(basename $port):$(cat $port/name)\"; done";

#[test]
fn a_guest_has_a_port_by_its_name() {
  let name = "a_guest_has_a_port_by_its_name";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, PORTS)), |configuration| {
    let device = VirtioConsoleDeviceConfiguration::new().unwrap();
    let port = VirtioConsolePortConfiguration::new().unwrap();

    port.set_name(Some("named"));
    device.ports().set(0, Some(&port));
    configuration.set_console_devices(&[&device]);
  });

  start(&queue, &machine);

  assert_eq!(console.read_until(":named"), "port:vport1p0:named");
}

/// Writes to the guest's second terminal, after `hvc0`, and reports whether
/// it could.
const TERMINAL: &str = "echo \"from hvc1\" > /dev/hvc1 && echo wrote:yes || echo wrote:no";

/// What a guest reports with `TERMINAL`, its console device's port 0 a
/// console as `is_console` says, and the line the host reads from the port
/// if it wrote.
fn terminal(name: &str, is_console: bool) -> (String, Option<String>) {
  let mut port_console = None;
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, TERMINAL)), |configuration| {
    let device = VirtioConsoleDeviceConfiguration::new().unwrap();
    let port = VirtioConsolePortConfiguration::new().unwrap();
    let (attachment, host) = console_attachment();

    port.set_is_console(is_console);
    port.set_attachment(Some(&attachment));
    device.ports().set(0, Some(&port));
    configuration.set_console_devices(&[&device]);
    port_console = Some(host);
  });

  start(&queue, &machine);

  let wrote = console.read_until("wrote:");
  let read = (wrote == "wrote:yes").then(|| port_console.unwrap().read_until("from"));

  (wrote, read)
}

#[test]
fn a_guest_has_a_console_port_as_a_terminal() {
  assert_eq!(
    terminal("a_guest_has_a_console_port_as_a_terminal", true),
    ("wrote:yes".to_string(), Some("from hvc1".to_string()))
  );
}

#[test]
fn a_guest_has_no_terminal_for_another_port() {
  assert_eq!(
    terminal("a_guest_has_no_terminal_for_another_port", false),
    ("wrote:no".to_string(), None)
  );
}

/// Reads a line from the port named `named`, and writes back what it read.
const ECHO: &str = "for port in /sys/class/virtio-ports/*; do [ \"$(cat $port/name)\" = named ] && \
                    device=/dev/$(basename $port); done\necho \"device:$device\"\nread line < $device\necho \
                    \"echo:$line\" > $device\necho done";

#[test]
fn carries_data_both_ways_through_a_port_s_attachment() {
  let name = "carries_data_both_ways_through_a_port_s_attachment";
  let mut port_console = None;
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, ECHO)), |configuration| {
    let device = VirtioConsoleDeviceConfiguration::new().unwrap();
    let port = VirtioConsolePortConfiguration::new().unwrap();
    let (attachment, host) = console_attachment();

    port.set_name(Some("named"));
    port.set_attachment(Some(&attachment));
    device.ports().set(0, Some(&port));
    configuration.set_console_devices(&[&device]);
    port_console = Some(host);
  });
  let mut port_console = port_console.unwrap();

  start(&queue, &machine);
  console.read_until("device:/dev/vport");
  writeln!(port_console.input, "from the host").unwrap();

  assert_eq!(port_console.read_until("echo:"), "echo:from the host");
  console.read_until("done");
}
