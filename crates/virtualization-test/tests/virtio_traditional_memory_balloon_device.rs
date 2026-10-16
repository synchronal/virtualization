use virtualization::{
  VirtioTraditionalMemoryBalloonDevice, VirtioTraditionalMemoryBalloonDeviceConfiguration, VirtualMachine,
  VirtualMachineConfiguration,
};
use virtualization_test::{
  Console, booting_machine, initial_ramdisk, linux_configuration, on_queue, serial_queue, start,
};

const MIB: u64 = 1024 * 1024;

/// Runs `check` on the balloon device of a machine with one, made on its
/// queue and not started.
fn with_device(name: &str, check: impl FnOnce(&VirtioTraditionalMemoryBalloonDevice) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    configuration.set_memory_balloon_devices(&[&VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.memory_balloon_devices::<VirtioTraditionalMemoryBalloonDevice>();

    assert_eq!(devices.len(), 1);
    check(devices[0].as_ref().unwrap());
  });
}

#[test]
fn targets_the_memory_size_at_first() {
  with_device("targets_the_memory_size_at_first", |device| {
    assert_eq!(device.target_virtual_machine_memory_size(), 512 * MIB);
  });
}

#[test]
fn rounds_a_target_down_to_a_mib() {
  with_device("rounds_a_target_down_to_a_mib", |device| {
    device.set_target_virtual_machine_memory_size(300 * MIB + 1);

    assert_eq!(device.target_virtual_machine_memory_size(), 300 * MIB);
  });
}

#[test]
fn clamps_a_target_to_the_memory_size() {
  with_device("clamps_a_target_to_the_memory_size", |device| {
    device.set_target_virtual_machine_memory_size(1024 * MIB);

    assert_eq!(device.target_virtual_machine_memory_size(), 512 * MIB);
  });
}

#[test]
fn clamps_a_target_to_the_minimum_allowed_memory_size() {
  with_device("clamps_a_target_to_the_minimum_allowed_memory_size", |device| {
    device.set_target_virtual_machine_memory_size(0);

    assert_eq!(
      device.target_virtual_machine_memory_size(),
      VirtualMachineConfiguration::minimum_allowed_memory_size().unwrap()
    );
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_device("memory_balloon_device_a_clone_is_the_same_object", |device| {
    device
      .clone()
      .set_target_virtual_machine_memory_size(256 * MIB);

    assert_eq!(device.target_virtual_machine_memory_size(), 256 * MIB);
  });
}

#[test]
fn debugs_as_its_description() {
  with_device("memory_balloon_device_debugs_as_its_description", |device| {
    assert!(format!("{device:?}").starts_with("<VZVirtioTraditionalMemoryBalloonDevice: 0x"));
  });
}

/// A guest that reports its free memory in KiB for each line it reads, once
/// it says it's ready.
const FREE: &str = "echo ready\nwhile read line; do echo \"free:$(grep MemFree /proc/meminfo | tr -s ' ' | cut -d ' ' -f \
                    2)\"; done";

/// Asks the guest for its free memory until `expected` holds of it, in MiB:
/// the guest handles a new target a moment after it is set.
fn wait_for_free(console: &mut Console, expected: impl Fn(u64) -> bool) {
  console.ask_until("free:", |free| expected(free.parse::<u64>().unwrap() / 1024));
}

#[test]
fn takes_memory_from_the_guest_and_gives_it_back() {
  let name = "takes_memory_from_the_guest_and_gives_it_back";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, FREE)), |configuration| {
    configuration.set_memory_balloon_devices(&[&VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap()]);
  });
  let set_target = |target| {
    on_queue(&queue, || {
      machine.memory_balloon_devices::<VirtioTraditionalMemoryBalloonDevice>()[0]
        .as_ref()
        .unwrap()
        .set_target_virtual_machine_memory_size(target);
    });
  };

  start(&queue, &machine);
  console.read_until("ready");
  wait_for_free(&mut console, |free| free > 400);

  set_target(256 * MIB);
  wait_for_free(&mut console, |free| free < 256);

  set_target(512 * MIB);
  wait_for_free(&mut console, |free| free > 400);
}
