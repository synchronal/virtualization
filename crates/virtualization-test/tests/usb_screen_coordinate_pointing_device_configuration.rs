use virtualization::USBScreenCoordinatePointingDeviceConfiguration;
use virtualization_test::{guest_pci_devices, linux_configuration};

#[test]
fn is_a_pointing_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_pointing_device_of_the_configuration_it_was_given_to");

  configuration.set_pointing_devices(&[&USBScreenCoordinatePointingDeviceConfiguration::new().unwrap()]);

  assert!(matches!(
    configuration.pointing_devices::<USBScreenCoordinatePointingDeviceConfiguration>()[..],
    [Some(_)]
  ));
  configuration.validate().unwrap();
}

/// The suites' kernel has no USB drivers, so the guest has the pointing
/// device's controller, by its PCI class (serial bus, USB, xHCI), and no
/// pointing device.
#[test]
fn a_guest_has_a_usb_controller_for_it() {
  let devices = guest_pci_devices("pointing_device_a_guest_has_a_usb_controller_for_it", |configuration| {
    configuration.set_pointing_devices(&[&USBScreenCoordinatePointingDeviceConfiguration::new().unwrap()]);
  });

  assert!(
    devices.iter().any(|device| device.ends_with(":0x0c0330")),
    "{devices:?}"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let pointing_device = USBScreenCoordinatePointingDeviceConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", pointing_device.clone()), format!("{pointing_device:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", USBScreenCoordinatePointingDeviceConfiguration::new().unwrap())
      .starts_with("<VZUSBScreenCoordinatePointingDeviceConfiguration: 0x")
  );
}
