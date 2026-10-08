use virtualization::{GenericMachineIdentifier, GenericPlatformConfiguration};
use virtualization_test::linux_configuration;

#[test]
fn answers_whether_nested_virtualization_is_supported() {
  GenericPlatformConfiguration::is_nested_virtualization_supported().unwrap();
}

#[test]
fn has_a_unique_machine_identifier_when_new() {
  let platform = GenericPlatformConfiguration::new().unwrap();

  assert_ne!(
    platform.machine_identifier(),
    GenericPlatformConfiguration::new()
      .unwrap()
      .machine_identifier()
  );
}

#[test]
fn has_the_machine_identifier_it_was_given() {
  let platform = GenericPlatformConfiguration::new().unwrap();
  let identifier = GenericMachineIdentifier::new().unwrap();

  platform.set_machine_identifier(&identifier);

  assert_eq!(platform.machine_identifier(), identifier);
}

#[test]
fn has_nested_virtualization_disabled_when_new() {
  let platform = GenericPlatformConfiguration::new().unwrap();

  assert!(!platform.is_nested_virtualization_enabled());
}

#[test]
fn has_the_nested_virtualization_it_was_given() {
  let platform = GenericPlatformConfiguration::new().unwrap();

  platform.set_nested_virtualization_enabled(true);

  assert!(platform.is_nested_virtualization_enabled());
}

#[test]
fn is_the_platform_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_the_platform_of_the_configuration_it_was_given_to");
  let platform = GenericPlatformConfiguration::new().unwrap();

  configuration.set_platform(&platform);

  assert_eq!(
    configuration
      .platform::<GenericPlatformConfiguration>()
      .unwrap()
      .machine_identifier(),
    platform.machine_identifier()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let platform = GenericPlatformConfiguration::new().unwrap();
  let clone = platform.clone();

  platform.set_nested_virtualization_enabled(true);

  assert!(clone.is_nested_virtualization_enabled());
}

#[test]
fn debugs_as_its_description() {
  let platform = GenericPlatformConfiguration::new().unwrap();

  assert!(format!("{platform:?}").starts_with("<VZGenericPlatformConfiguration: 0x"));
}
