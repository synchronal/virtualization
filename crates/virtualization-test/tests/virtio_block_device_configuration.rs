use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{
  DiskImageStorageDeviceAttachment, StorageDeviceConfiguration, VirtioBlockDeviceConfiguration, VirtualMachine,
};
use virtualization_test::{Console, booting_machine, disk_image, initial_ramdisk, ns_error, raw_image, start, stop};

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
  let error = ns_error(VirtioBlockDeviceConfiguration::validate_block_device_identifier(
    "abcdefghijklmnopqrstu",
  ));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_on_an_identifier_that_isnt_ascii() {
  let error = ns_error(VirtioBlockDeviceConfiguration::validate_block_device_identifier(
    "disqué",
  ));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

/// A started guest running `script` with the raw image at `url` as `vda`.
fn guest(
  name: &str,
  script: &str,
  url: &Url,
  read_only: bool,
  block_device_identifier: &str,
) -> (DispatchRetained<DispatchQueue>, VirtualMachine, Console) {
  let (queue, machine, console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |configuration| {
    let device =
      VirtioBlockDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(url, read_only).unwrap()).unwrap();

    device.set_block_device_identifier(block_device_identifier);
    configuration.set_storage_devices(&[&device]);
  });

  start(&queue, &machine);

  (queue, machine, console)
}

#[test]
fn a_guest_reads_the_disk_image_it_was_given() {
  let name = "a_guest_reads_the_disk_image_it_was_given";
  let url = raw_image(name, b"from the host");

  let (_queue, _machine, mut console) = guest(name, "echo \"read:$(head -c 13 /dev/vda)\"", &url, true, "");

  assert_eq!(console.read_until("read:").trim_end(), "read:from the host");
}

#[test]
fn a_guest_writes_a_writable_disk_image() {
  let name = "a_guest_writes_a_writable_disk_image";
  let url = raw_image(name, b"");
  let script = "printf 'from the guest' | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null\necho written";

  let (queue, machine, mut console) = guest(name, script, &url, false, "");
  console.read_until("written");
  stop(&queue, &machine);

  assert!(
    std::fs::read(url.to_file_path().unwrap())
      .unwrap()
      .starts_with(b"from the guest")
  );
}

#[test]
fn a_guest_cannot_write_a_read_only_disk_image() {
  let name = "a_guest_cannot_write_a_read_only_disk_image";
  let url = raw_image(name, b"from the host");
  let script = "printf x | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null && echo write:done || echo write:refused";

  let (queue, machine, mut console) = guest(name, script, &url, true, "");

  assert_eq!(console.read_until("write:").trim_end(), "write:refused");
  stop(&queue, &machine);
  assert!(
    std::fs::read(url.to_file_path().unwrap())
      .unwrap()
      .starts_with(b"from the host")
  );
}

#[test]
fn a_guest_sees_the_block_device_identifier() {
  let name = "a_guest_sees_the_block_device_identifier";
  let url = raw_image(name, b"");

  let (_queue, _machine, mut console) = guest(name, "echo \"serial:$(cat /sys/block/vda/serial)\"", &url, true, "root");

  assert_eq!(console.read_until("serial:").trim_end(), "serial:root");
}

#[test]
fn debugs_as_its_description() {
  let configuration = VirtioBlockDeviceConfiguration::new(&attachment("block_debugs_as_its_description")).unwrap();

  assert!(format!("{configuration:?}").starts_with("<VZVirtioBlockDeviceConfiguration: 0x"));
}
