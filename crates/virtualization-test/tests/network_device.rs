use std::io::Write;
use std::time::Duration;

use virtualization::{
  NATNetworkDeviceAttachment, NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration, VirtualMachine,
};
use virtualization_test::{
  Console, booting_machine, initial_ramdisk, linux_configuration, on_queue, serial_queue, start,
};

/// A guest that reports `eth0`'s carrier for each line it reads, once it
/// says it's ready.
const CARRIER: &str = "ip link set eth0 up\necho ready\nwhile read line; do echo \"carrier:$(cat \
                       /sys/class/net/eth0/carrier)\"; done";

#[test]
fn is_one_per_network_device_configuration_with_its_attachment() {
  let name = "is_one_per_network_device_configuration_with_its_attachment";
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);
    let attached = VirtioNetworkDeviceConfiguration::new().unwrap();

    attached.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));
    configuration.set_network_devices(&[&attached, &VirtioNetworkDeviceConfiguration::new().unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.network_devices();

    assert_eq!(devices.len(), 2);
    assert!(
      devices[0]
        .attachment::<NATNetworkDeviceAttachment>()
        .is_some()
    );
    assert!(
      devices[1]
        .attachment::<NATNetworkDeviceAttachment>()
        .is_none()
    );
  });
}

/// Asks the guest for `eth0`'s carrier until it is `expected`: the guest sees
/// an attachment change a moment after it is made.
fn wait_for_carrier(console: &mut Console, expected: &str) {
  for _ in 0..50 {
    writeln!(console.input).unwrap();
    if console.read_until("carrier:").trim_end() == format!("carrier:{expected}") {
      return;
    }
    std::thread::sleep(Duration::from_millis(100));
  }

  panic!("the guest's carrier never became {expected}");
}

#[test]
fn detaches_and_reattaches_while_running() {
  let name = "detaches_and_reattaches_while_running";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, CARRIER)), |configuration| {
    let device = VirtioNetworkDeviceConfiguration::new().unwrap();

    device.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));
    configuration.set_network_devices(&[&device]);
  });

  start(&queue, &machine);
  console.read_until("ready");
  wait_for_carrier(&mut console, "1");

  on_queue(&queue, || {
    let device = &machine.network_devices()[0];

    device.set_attachment(None);

    assert!(device.attachment::<NATNetworkDeviceAttachment>().is_none());
  });
  wait_for_carrier(&mut console, "0");

  on_queue(&queue, || {
    let device = &machine.network_devices()[0];

    device.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));

    assert!(device.attachment::<NATNetworkDeviceAttachment>().is_some());
  });
  wait_for_carrier(&mut console, "1");
}

#[test]
fn a_clone_is_the_same_object() {
  let name = "network_device_a_clone_is_the_same_object";
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    configuration.set_network_devices(&[&VirtioNetworkDeviceConfiguration::new().unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let device = &machine.network_devices()[0];

    device
      .clone()
      .set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));

    assert!(device.attachment::<NATNetworkDeviceAttachment>().is_some());
  });
}

#[test]
fn debugs_as_its_description() {
  let name = "network_device_debugs_as_its_description";
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    configuration.set_network_devices(&[&VirtioNetworkDeviceConfiguration::new().unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();

    assert!(format!("{:?}", machine.network_devices()[0]).starts_with("<VZNetworkDevice: 0x"));
  });
}
