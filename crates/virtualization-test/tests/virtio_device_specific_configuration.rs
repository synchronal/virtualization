use virtualization::VirtioDeviceSpecificConfiguration;

#[test]
fn has_the_data_it_was_made_with() {
  assert_eq!(
    VirtioDeviceSpecificConfiguration::new(&[1, 2, 3, 4])
      .unwrap()
      .configuration_data(),
    [1, 2, 3, 4]
  );
}

#[test]
fn may_be_empty() {
  assert!(
    VirtioDeviceSpecificConfiguration::new(&[])
      .unwrap()
      .configuration_data()
      .is_empty()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = VirtioDeviceSpecificConfiguration::new(&[1]).unwrap();

  assert_eq!(format!("{:?}", configuration.clone()), format!("{configuration:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioDeviceSpecificConfiguration::new(&[1]).unwrap())
      .starts_with("<VZVirtioDeviceSpecificConfiguration: 0x")
  );
}
