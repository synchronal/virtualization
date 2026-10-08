use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use virtualization::{
  VirtioConsoleDevice, VirtioConsoleDeviceConfiguration, VirtioConsoleDeviceDelegate, VirtioConsolePort,
  VirtioConsolePortConfiguration, VirtualMachine, VirtualMachineConfiguration,
};
use virtualization_test::{booting_machine, initial_ramdisk, linux_configuration, on_queue, serial_queue, start};

/// Gives `configuration` a console device whose port 0 is named `named`.
fn add_device(configuration: &VirtualMachineConfiguration) {
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some("named"));
  device.ports().set(0, Some(&port));
  configuration.set_console_devices(&[&device]);
}

/// Runs `check` on the console device of a machine made with `add_device`,
/// on its queue and not started.
fn with_device(name: &str, check: impl FnOnce(&VirtioConsoleDevice) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    add_device(&configuration);

    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.console_devices::<VirtioConsoleDevice>();

    assert_eq!(devices.len(), 1);
    check(devices[0].as_ref().unwrap());
  });
}

#[test]
fn has_the_ports_of_its_configuration() {
  with_device("has_the_ports_of_its_configuration", |device| {
    assert_eq!(device.ports().get(0).unwrap().name().as_deref(), Some("named"));
  });
}

struct Delegate;

impl VirtioConsoleDeviceDelegate for Delegate {}

#[test]
fn has_the_delegate_it_was_given() {
  with_device("console_device_has_the_delegate_it_was_given", |device| {
    let delegate: Arc<dyn VirtioConsoleDeviceDelegate> = Arc::new(Delegate);

    assert!(device.delegate().is_none());

    device.set_delegate(Some(&delegate));

    assert!(Arc::ptr_eq(&device.delegate().unwrap(), &delegate));

    device.set_delegate(None);

    assert!(device.delegate().is_none());
  });
}

#[test]
fn holds_its_delegate_weakly() {
  with_device("console_device_holds_its_delegate_weakly", |device| {
    let delegate: Arc<dyn VirtioConsoleDeviceDelegate> = Arc::new(Delegate);

    device.set_delegate(Some(&delegate));
    drop(delegate);

    assert!(device.delegate().is_none());
  });
}

/// Tells its sender each port opened and closed, by name.
struct Opens(Mutex<Sender<String>>);

impl VirtioConsoleDeviceDelegate for Opens {
  fn console_device_did_open(&self, _console_device: &VirtioConsoleDevice, console_port: &VirtioConsolePort) {
    let _ = self
      .0
      .lock()
      .unwrap()
      .send(format!("open:{}", console_port.name().unwrap()));
  }

  fn console_device_did_close(&self, _console_device: &VirtioConsoleDevice, console_port: &VirtioConsolePort) {
    let _ = self
      .0
      .lock()
      .unwrap()
      .send(format!("close:{}", console_port.name().unwrap()));
  }
}

/// Opens the port named `named`, writes to it, and closes it.
const OPEN: &str = "for port in /sys/class/virtio-ports/*; do [ \"$(cat $port/name)\" = named ] && \
                    device=/dev/$(basename $port); done\necho opened > $device";

#[test]
fn tells_its_delegate_when_the_guest_opens_and_closes_a_port() {
  let name = "tells_its_delegate_when_the_guest_opens_and_closes_a_port";
  let (queue, machine, _console) = booting_machine(name, Some(&initial_ramdisk(name, OPEN)), add_device);
  let (sender, receiver) = mpsc::channel();
  let delegate: Arc<dyn VirtioConsoleDeviceDelegate> = Arc::new(Opens(Mutex::new(sender)));

  on_queue(&queue, || {
    machine.console_devices::<VirtioConsoleDevice>()[0]
      .as_ref()
      .unwrap()
      .set_delegate(Some(&delegate));
  });
  start(&queue, &machine);

  assert_eq!(receiver.recv().unwrap(), "open:named");
  assert_eq!(receiver.recv().unwrap(), "close:named");
}

#[test]
fn a_clone_is_the_same_object() {
  with_device("console_device_a_clone_is_the_same_object", |device| {
    let delegate: Arc<dyn VirtioConsoleDeviceDelegate> = Arc::new(Delegate);

    device.clone().set_delegate(Some(&delegate));

    assert!(Arc::ptr_eq(&device.delegate().unwrap(), &delegate));
  });
}

#[test]
fn debugs_as_its_description() {
  with_device("console_device_debugs_as_its_description", |device| {
    assert!(format!("{device:?}").starts_with("<VZVirtioConsoleDevice: 0x"));
  });
}
