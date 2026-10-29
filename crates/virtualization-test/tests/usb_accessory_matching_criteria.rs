use virtualization::accessory_access::{
  USBAccessoryMatchingCriteria, USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
  USBAccessoryMatchingCriteriaInterfaceMatchingCriteria, USBAccessoryMatchingCriteriaInterfaceMatchingOption,
};
use virtualization_test::apple_device_matching_dictionary;

fn device_matching_criteria() -> USBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  USBAccessoryMatchingCriteriaDeviceMatchingCriteria {
    vendor_id: Some(0x05ac),
    ..Default::default()
  }
}

/// Even without the entitlement, unlike interface matching dictionaries.
#[test]
fn makes_one_of_a_device_matching_dictionary() {
  let criteria = USBAccessoryMatchingCriteria::new(&apple_device_matching_dictionary())
    .unwrap()
    .unwrap();

  assert!(format!("{criteria:?}").starts_with("<AAUSBAccessoryMatchingCriteria: 0x"));
}

#[test]
fn makes_none_of_interface_matching_dictionaries_without_the_entitlement() {
  let dictionary = apple_device_matching_dictionary();

  assert!(
    USBAccessoryMatchingCriteria::with_interface_matching_dictionaries_and_interface_matching_option(
      Some(&dictionary),
      std::slice::from_ref(&dictionary),
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAny,
    )
    .unwrap()
    .is_none()
  );
}

#[test]
fn makes_one_of_device_matching_criteria() {
  let criteria = USBAccessoryMatchingCriteria::with_device_matching_criteria(&device_matching_criteria()).unwrap();

  assert!(format!("{criteria:?}").starts_with("<AAUSBAccessoryMatchingCriteria: 0x"));
}

#[test]
fn makes_one_of_interface_matching_criteria() {
  let interface = USBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
    interface_class: Some(8),
    ..Default::default()
  };

  for device in [None, Some(&device_matching_criteria())] {
    let criteria =
      USBAccessoryMatchingCriteria::with_device_matching_criteria_and_interface_matching_criteria_and_interface_matching_option(
        device,
        &[interface],
        USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAll,
      )
      .unwrap();

    assert!(format!("{criteria:?}").starts_with("<AAUSBAccessoryMatchingCriteria: 0x"));
  }
}

#[test]
fn a_clone_is_the_same_object() {
  let criteria = USBAccessoryMatchingCriteria::with_device_matching_criteria(&device_matching_criteria()).unwrap();

  assert_eq!(format!("{:?}", criteria.clone()), format!("{criteria:?}"));
}
