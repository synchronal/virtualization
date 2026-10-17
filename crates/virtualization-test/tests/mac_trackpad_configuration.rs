use virtualization::MacTrackpadConfiguration;
use virtualization_test::{guest_pci_devices, linux_configuration};

#[test]
fn is_a_pointing_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_pointing_device_of_the_configuration_it_was_given_to");

  configuration.set_pointing_devices(&[&MacTrackpadConfiguration::new().unwrap()]);

  assert!(matches!(
    configuration.pointing_devices::<MacTrackpadConfiguration>()[..],
    [Some(_)]
  ));
  configuration.validate().unwrap();
}

/// Only a macOS guest has a device for it: a Linux guest has the PCI devices
/// it would without it.
#[test]
fn a_linux_guest_has_no_device_for_it() {
  let without = guest_pci_devices("mac_trackpad_configuration_a_linux_guest_without_it", |_| {});
  let with = guest_pci_devices("mac_trackpad_configuration_a_linux_guest_with_it", |configuration| {
    configuration.set_pointing_devices(&[&MacTrackpadConfiguration::new().unwrap()]);
  });

  assert_eq!(with, without);
}

#[test]
fn a_clone_is_the_same_object() {
  let device = MacTrackpadConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", MacTrackpadConfiguration::new().unwrap()).starts_with("<VZMacTrackpadConfiguration: 0x"));
}
