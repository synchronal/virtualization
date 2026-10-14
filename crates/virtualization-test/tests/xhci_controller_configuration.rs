use virtualization::{
  USBControllerConfiguration, USBDeviceConfiguration, USBMassStorageDeviceConfiguration, XHCIControllerConfiguration,
};
use virtualization_test::{linux_configuration, usb_mass_storage_device_configuration};

#[test]
fn has_no_usb_devices_when_new() {
  let configuration = XHCIControllerConfiguration::new().unwrap();

  assert!(
    configuration
      .usb_devices::<USBMassStorageDeviceConfiguration>()
      .is_empty()
  );
}

#[test]
fn has_the_usb_devices_it_was_given() {
  let configuration = XHCIControllerConfiguration::new().unwrap();
  let device = usb_mass_storage_device_configuration("has_the_usb_devices_it_was_given");

  configuration.set_usb_devices(&[&device]);

  let devices = configuration.usb_devices::<USBMassStorageDeviceConfiguration>();

  assert_eq!(devices.len(), 1);
  assert_eq!(devices[0].as_ref().unwrap().uuid(), device.uuid());
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = XHCIControllerConfiguration::new().unwrap();
  let clone = configuration.clone();

  configuration.set_usb_devices(&[&usb_mass_storage_device_configuration(
    "xhci_a_clone_is_the_same_object",
  )]);
  drop(configuration);

  assert_eq!(
    clone
      .usb_devices::<USBMassStorageDeviceConfiguration>()
      .len(),
    1
  );
}

#[test]
fn is_a_usb_controller_of_the_configuration_it_was_given_to() {
  let machine = linux_configuration("xhci_is_a_usb_controller_of_the_configuration_it_was_given_to");

  machine.set_usb_controllers(&[
    &XHCIControllerConfiguration::new().unwrap(),
    &XHCIControllerConfiguration::new().unwrap(),
  ]);

  assert!(matches!(
    machine.usb_controllers::<XHCIControllerConfiguration>()[..],
    [Some(_), Some(_)]
  ));
  machine.validate().unwrap();
}

#[test]
fn debugs_as_its_description() {
  let configuration = XHCIControllerConfiguration::new().unwrap();

  assert!(format!("{configuration:?}").starts_with("<VZXHCIControllerConfiguration: 0x"));
}
