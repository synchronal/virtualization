use virtualization::CustomVirtioDeviceConfiguration;

/// Virtualization keeps `VIRTIO_F_VERSION_1` (bit 32) mandatory.
#[test]
fn keeps_the_features_it_is_given_with_virtio_version_1() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .mandatory_features();

  features.set_subset0(0x8000_0001);
  features.set_subset1(0x2);

  assert_eq!((features.subset0(), features.subset1()), (0x8000_0001, 0x3));
}

/// Virtualization keeps indirect descriptors and event indexes (bits 28 and
/// 29) optional.
#[test]
fn keeps_the_features_it_is_given_with_what_performs_better() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .optional_features();

  features.set_subset0(0);

  assert_eq!(features.subset0(), 0x3000_0000);
}

#[test]
fn is_its_configurations_own() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration.optional_features().set_subset0(0x10);

  assert_eq!(configuration.optional_features().subset0(), 0x3000_0010);
  assert_eq!(configuration.mandatory_features().subset0(), 0);
}

#[test]
fn a_clone_is_the_same_object() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .mandatory_features();

  features.clone().set_subset0(1);

  assert_eq!(features.subset0(), 1);
}

#[test]
fn debugs_as_its_description() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .mandatory_features();

  assert!(format!("{features:?}").starts_with("<VZVirtioFeatureSet: 0x"));
}
