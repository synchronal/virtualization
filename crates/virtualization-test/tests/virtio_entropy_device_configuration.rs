use virtualization::{VirtioEntropyDeviceConfiguration, VirtualMachineConfiguration};
use virtualization_test::{booting_machine, initial_ramdisk, linux_configuration, start};

#[test]
fn is_an_entropy_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_an_entropy_device_of_the_configuration_it_was_given_to");

  configuration.set_entropy_devices(&[&VirtioEntropyDeviceConfiguration::new().unwrap()]);

  let given = configuration.entropy_devices::<VirtioEntropyDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert!(given[0].is_some());
  configuration.validate().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioEntropyDeviceConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioEntropyDeviceConfiguration::new().unwrap())
      .starts_with("<VZVirtioEntropyDeviceConfiguration: 0x")
  );
}

/// Reports the guest's hardware random number generator, and how many bytes
/// of 16 it reads from it.
const RNG: &str = "echo \"rng:$(cat /sys/class/misc/hw_random/rng_current 2>&1)\"\necho \"read:$(head -c 16 /dev/hwrng \
                   2>/dev/null | wc -c)\"";

/// What a guest reports with `RNG`, after `configure`.
fn rng(name: &str, configure: impl FnOnce(&VirtualMachineConfiguration) + Send) -> (String, String) {
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, RNG)), configure);

  start(&queue, &machine);

  (
    console.read_until("rng:").trim_end().to_string(),
    console.read_until("read:").trim_end().to_string(),
  )
}

#[test]
fn a_guest_reads_its_hardware_random_number_generator() {
  let reported = rng("a_guest_reads_its_hardware_random_number_generator", |configuration| {
    configuration.set_entropy_devices(&[&VirtioEntropyDeviceConfiguration::new().unwrap()]);
  });

  assert_eq!(reported, ("rng:virtio_rng.0".to_string(), "read:16".to_string()));
}

#[test]
fn a_guest_without_one_has_no_hardware_random_number_generator() {
  let reported = rng("a_guest_without_one_has_no_hardware_random_number_generator", |_| {});

  assert_eq!(reported, ("rng:none".to_string(), "read:0".to_string()));
}
