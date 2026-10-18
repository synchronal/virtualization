use virtualization::{MacHardwareModel, MacPlatformConfiguration};
use virtualization_test::{mac_hardware_model, macos_requirements};

#[test]
fn has_the_data_representation_it_was_made_from() {
  let data_representation = macos_requirements().hardware_model;

  let hardware_model = MacHardwareModel::new(&data_representation)
    .unwrap()
    .unwrap();

  assert_eq!(hardware_model.data_representation(), data_representation);
}

#[test]
fn is_none_from_invalid_data() {
  assert!(
    MacHardwareModel::new(b"not a hardware model")
      .unwrap()
      .is_none()
  );
  assert!(MacHardwareModel::new(b"").unwrap().is_none());
}

#[test]
fn is_supported_when_the_latest_supported_restore_image_requires_it() {
  assert!(mac_hardware_model().is_supported());
}

/// A new platform configuration's, which no configuration validates with.
#[test]
fn is_unsupported_when_a_platform_configuration_is_new() {
  assert!(
    !MacPlatformConfiguration::new()
      .unwrap()
      .hardware_model()
      .is_supported()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let hardware_model = mac_hardware_model();

  assert_eq!(format!("{:?}", hardware_model.clone()), format!("{hardware_model:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", mac_hardware_model()).starts_with("<VZMacHardwareModel: 0x"));
}
