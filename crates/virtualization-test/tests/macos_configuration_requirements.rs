//! The latest supported restore image's, fetched from Apple's catalog over
//! the network.

use virtualization::MacOSRestoreImage;
use virtualization_test::{macos_requirements, restore_image};

#[test]
fn has_a_hardware_model_the_host_supports() {
  assert!(restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    image
      .unwrap()
      .most_featureful_supported_configuration()
      .unwrap()
      .hardware_model()
      .is_supported()
  }));
}

#[test]
fn has_a_minimum_supported_cpu_count() {
  assert!(macos_requirements().minimum_supported_cpu_count >= 1);
}

#[test]
fn has_a_minimum_supported_memory_size() {
  assert!(macos_requirements().minimum_supported_memory_size > 0);
}

#[test]
fn a_clone_is_the_same_object() {
  let (clone, original) = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    let requirements = image
      .unwrap()
      .most_featureful_supported_configuration()
      .unwrap();

    (format!("{:?}", requirements.clone()), format!("{requirements:?}"))
  });

  assert_eq!(clone, original);
}

#[test]
fn debugs_as_its_description() {
  let description = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    format!(
      "{:?}",
      image
        .unwrap()
        .most_featureful_supported_configuration()
        .unwrap()
    )
  });

  assert!(
    description.starts_with("<VZMacOSConfigurationRequirements: 0x"),
    "{description}"
  );
}
