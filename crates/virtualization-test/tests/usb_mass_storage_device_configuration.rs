use uuid::Uuid;
use virtualization::{
  DiskImageStorageDeviceAttachment, StorageDeviceConfiguration, USBDeviceConfiguration,
  USBMassStorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
};
use virtualization_test::{disk_image, linux_configuration, usb_mass_storage_device_configuration};

#[test]
fn has_the_attachment_it_was_made_with() {
  let url = disk_image("usb_has_the_attachment_it_was_made_with");
  let configuration =
    USBMassStorageDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&url, true).unwrap()).unwrap();

  let attachment = configuration
    .attachment::<DiskImageStorageDeviceAttachment>()
    .unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.is_read_only());
}

#[test]
fn has_a_uuid_of_its_own() {
  let configuration = usb_mass_storage_device_configuration("has_a_uuid_of_its_own");
  let another = usb_mass_storage_device_configuration("has_a_uuid_of_its_own_too");

  assert_eq!(configuration.uuid(), configuration.uuid());
  assert_ne!(configuration.uuid(), another.uuid());
}

#[test]
fn sets_its_uuid() {
  let configuration = usb_mass_storage_device_configuration("sets_its_uuid");
  let uuid: Uuid = "01234567-89ab-cdef-0123-456789abcdef".parse().unwrap();

  configuration.set_uuid(&uuid);

  assert_eq!(configuration.uuid(), uuid);
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = usb_mass_storage_device_configuration("usb_a_clone_is_the_same_object");
  let clone = configuration.clone();
  let uuid: Uuid = "01234567-89ab-cdef-0123-456789abcdef".parse().unwrap();

  configuration.set_uuid(&uuid);
  drop(configuration);

  assert_eq!(clone.uuid(), uuid);
}

/// It validates as a storage device with no USB controller to attach it to.
#[test]
fn is_a_storage_device_of_the_configuration_it_was_given_to() {
  let name = "usb_is_a_storage_device_of_the_configuration_it_was_given_to";
  let machine = linux_configuration(name);

  machine.set_storage_devices(&[&usb_mass_storage_device_configuration(name)]);

  assert!(matches!(
    machine.storage_devices::<USBMassStorageDeviceConfiguration>()[..],
    [Some(_)]
  ));
  assert!(matches!(
    machine.storage_devices::<VirtioBlockDeviceConfiguration>()[..],
    [None]
  ));
  machine.validate().unwrap();
}

#[test]
fn debugs_as_its_description() {
  let configuration = usb_mass_storage_device_configuration("usb_debugs_as_its_description");

  assert!(format!("{configuration:?}").starts_with("<VZUSBMassStorageDeviceConfiguration: 0x"));
}
