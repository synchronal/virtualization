use virtualization::{
  ErrorCode, MacAuxiliaryStorage, MacMachineIdentifier, MacPlatformConfiguration, VirtualMachineConfiguration,
};
use virtualization_test::{
  file_url, mac_auxiliary_storage, mac_hardware_model, macos_configuration, macos_requirements, ns_error,
};

#[test]
fn has_the_hardware_model_it_was_given() {
  let platform = MacPlatformConfiguration::new().unwrap();
  let hardware_model = mac_hardware_model();

  platform.set_hardware_model(&hardware_model);

  assert_eq!(
    platform.hardware_model().data_representation(),
    hardware_model.data_representation()
  );
}

#[test]
fn has_a_unique_machine_identifier_when_new() {
  assert_ne!(
    MacPlatformConfiguration::new()
      .unwrap()
      .machine_identifier(),
    MacPlatformConfiguration::new()
      .unwrap()
      .machine_identifier()
  );
}

#[test]
fn has_the_machine_identifier_it_was_given() {
  let platform = MacPlatformConfiguration::new().unwrap();
  let identifier = MacMachineIdentifier::new().unwrap();

  platform.set_machine_identifier(&identifier);

  assert_eq!(platform.machine_identifier(), identifier);
}

#[test]
fn has_no_auxiliary_storage_when_new() {
  assert!(
    MacPlatformConfiguration::new()
      .unwrap()
      .auxiliary_storage()
      .is_none()
  );
}

#[test]
fn has_the_auxiliary_storage_it_was_given() {
  let platform = MacPlatformConfiguration::new().unwrap();
  let storage = mac_auxiliary_storage("has_the_auxiliary_storage_it_was_given", &mac_hardware_model());

  platform.set_auxiliary_storage(Some(&storage));

  assert_eq!(
    platform.auxiliary_storage().unwrap().url().unwrap(),
    storage.url().unwrap()
  );

  platform.set_auxiliary_storage(None);

  assert!(platform.auxiliary_storage().is_none());
}

#[test]
fn is_the_platform_of_the_configuration_it_was_given_to() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let platform = MacPlatformConfiguration::new().unwrap();

  configuration.set_platform(&platform);

  assert_eq!(
    configuration
      .platform::<MacPlatformConfiguration>()
      .unwrap()
      .machine_identifier(),
    platform.machine_identifier()
  );
}

#[test]
fn validates_with_a_hardware_model_and_auxiliary_storage() {
  macos_configuration(
    "validates_with_a_hardware_model_and_auxiliary_storage",
    &macos_requirements(),
  )
  .validate()
  .unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_without_auxiliary_storage() {
  let configuration = macos_configuration(
    "fails_validation_as_virtualization_does_without_auxiliary_storage",
    &macos_requirements(),
  );

  configuration
    .platform::<MacPlatformConfiguration>()
    .unwrap()
    .set_auxiliary_storage(None);

  let error = ns_error(configuration.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("“auxiliaryStorage” is nil."),
    "{error:#?}"
  );
}

/// Virtualization reads it only when the machine starts.
#[test]
fn validates_with_auxiliary_storage_that_does_not_exist() {
  let configuration = macos_configuration(
    "validates_with_auxiliary_storage_that_does_not_exist",
    &macos_requirements(),
  );

  configuration
    .platform::<MacPlatformConfiguration>()
    .unwrap()
    .set_auxiliary_storage(Some(
      &MacAuxiliaryStorage::new(&file_url("/nonexistent/auxiliary-storage")).unwrap(),
    ));

  configuration.validate().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let platform = MacPlatformConfiguration::new().unwrap();
  let identifier = MacMachineIdentifier::new().unwrap();

  platform.clone().set_machine_identifier(&identifier);

  assert_eq!(platform.machine_identifier(), identifier);
}

#[test]
fn debugs_as_its_description() {
  let platform = MacPlatformConfiguration::new().unwrap();

  assert!(format!("{platform:?}").starts_with("<VZMacPlatformConfiguration: 0x"));
}
