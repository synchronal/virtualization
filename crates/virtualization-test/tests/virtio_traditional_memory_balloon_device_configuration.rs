use virtualization::{ErrorCode, VirtioTraditionalMemoryBalloonDeviceConfiguration};
use virtualization_test::{linux_configuration, ns_error};

#[test]
fn is_a_memory_balloon_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_memory_balloon_device_of_the_configuration_it_was_given_to");

  configuration.set_memory_balloon_devices(&[&VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap()]);

  let given = configuration.memory_balloon_devices::<VirtioTraditionalMemoryBalloonDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert!(given[0].is_some());
  configuration.validate().unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_with_two() {
  let configuration = linux_configuration("memory_balloon_fails_validation_as_virtualization_does_with_two");

  configuration.set_memory_balloon_devices(&[
    &VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap(),
    &VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap(),
  ]);

  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!(
      "{:?}",
      VirtioTraditionalMemoryBalloonDeviceConfiguration::new().unwrap()
    )
    .starts_with("<VZVirtioTraditionalMemoryBalloonDeviceConfiguration: 0x")
  );
}
