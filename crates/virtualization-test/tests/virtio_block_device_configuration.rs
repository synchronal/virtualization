use virtualization::{
  DiskImageStorageDeviceAttachment, Error, StorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
};
use virtualization_test::disk_image;

fn attachment(name: &str) -> DiskImageStorageDeviceAttachment {
  DiskImageStorageDeviceAttachment::new(&disk_image(name), true).unwrap()
}

#[test]
fn has_the_attachment_it_was_made_with() {
  let url = disk_image("has_the_attachment_it_was_made_with");
  let configuration =
    VirtioBlockDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&url, true).unwrap()).unwrap();

  let attachment = configuration
    .attachment::<DiskImageStorageDeviceAttachment>()
    .unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.is_read_only());
}

#[test]
fn has_no_identifier_when_none_is_set() {
  let configuration = VirtioBlockDeviceConfiguration::new(&attachment("has_no_identifier_when_none_is_set")).unwrap();

  assert_eq!(configuration.block_device_identifier(), "");
}

#[test]
fn sets_the_identifier() {
  let configuration = VirtioBlockDeviceConfiguration::new(&attachment("sets_the_identifier")).unwrap();

  configuration.set_block_device_identifier("root");

  assert_eq!(configuration.block_device_identifier(), "root");
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = VirtioBlockDeviceConfiguration::new(&attachment("block_a_clone_is_the_same_object")).unwrap();
  let clone = configuration.clone();

  configuration.set_block_device_identifier("shared");
  drop(configuration);

  assert_eq!(clone.block_device_identifier(), "shared");
}

#[test]
fn validates_an_identifier_of_20_ascii_bytes() {
  assert!(VirtioBlockDeviceConfiguration::validate_block_device_identifier("abcdefghijklmnopqrst").is_ok());
  assert!(VirtioBlockDeviceConfiguration::validate_block_device_identifier("").is_ok());
}

#[test]
fn fails_as_virtualization_does_on_an_identifier_over_20_bytes() {
  let Err(Error::NSError(error)) =
    VirtioBlockDeviceConfiguration::validate_block_device_identifier("abcdefghijklmnopqrstu")
  else {
    panic!("validated an identifier over 20 bytes");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_on_an_identifier_that_isnt_ascii() {
  let Err(Error::NSError(error)) = VirtioBlockDeviceConfiguration::validate_block_device_identifier("disqué") else {
    panic!("validated an identifier that isn't ASCII");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn debugs_as_its_description() {
  let configuration = VirtioBlockDeviceConfiguration::new(&attachment("block_debugs_as_its_description")).unwrap();

  assert!(format!("{configuration:?}").starts_with("<VZVirtioBlockDeviceConfiguration: 0x"));
}
