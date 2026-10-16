use std::fs::File;
use std::os::fd::{AsRawFd, OwnedFd};

use virtualization::{
  DiskBlockDeviceStorageDeviceAttachment, DiskImageStorageDeviceAttachment, DiskSynchronizationMode,
  StorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
};
use virtualization_test::{
  BlockDevice, block_device, booting_machine, disk_image, initial_ramdisk, ns_error, start, stop,
};

#[test]
fn attaches_a_block_device_by_the_descriptor_it_was_given() {
  let device = block_device("attaches_a_block_device_by_the_descriptor_it_was_given", b"");
  let file_handle = device.open();
  let descriptor = file_handle.as_raw_fd();

  let attachment =
    DiskBlockDeviceStorageDeviceAttachment::new(file_handle, false, DiskSynchronizationMode::Full).unwrap();

  assert_eq!(attachment.file_handle().as_raw_fd(), descriptor);
  assert!(!attachment.is_read_only());
  assert_eq!(attachment.synchronization_mode(), DiskSynchronizationMode::Full);
}

#[test]
fn attaches_read_only_with_no_synchronization() {
  let device = block_device("attaches_read_only_with_no_synchronization", b"");

  let attachment =
    DiskBlockDeviceStorageDeviceAttachment::new(device.open(), true, DiskSynchronizationMode::None).unwrap();

  assert!(attachment.is_read_only());
  assert_eq!(attachment.synchronization_mode(), DiskSynchronizationMode::None);
}

#[test]
fn fails_as_virtualization_does_on_a_file_that_is_no_block_device() {
  let image = disk_image("fails_as_virtualization_does_on_a_file_that_is_no_block_device");
  let file_handle = OwnedFd::from(File::open(image.to_file_path().unwrap()).unwrap());

  let error = ns_error(DiskBlockDeviceStorageDeviceAttachment::new(
    file_handle,
    true,
    DiskSynchronizationMode::Full,
  ));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn is_the_attachment_of_the_configuration_it_was_given_to() {
  let device = block_device("block_is_the_attachment_of_the_configuration_it_was_given_to", b"");
  let file_handle = device.open();
  let descriptor = file_handle.as_raw_fd();
  let attachment =
    DiskBlockDeviceStorageDeviceAttachment::new(file_handle, true, DiskSynchronizationMode::Full).unwrap();

  let configuration = VirtioBlockDeviceConfiguration::new(&attachment).unwrap();

  assert_eq!(
    configuration
      .attachment::<DiskBlockDeviceStorageDeviceAttachment>()
      .unwrap()
      .file_handle()
      .as_raw_fd(),
    descriptor
  );
  assert!(
    configuration
      .attachment::<DiskImageStorageDeviceAttachment>()
      .is_none()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let device = block_device("disk_block_a_clone_is_the_same_object", b"");
  let attachment =
    DiskBlockDeviceStorageDeviceAttachment::new(device.open(), true, DiskSynchronizationMode::Full).unwrap();
  let descriptor = attachment.file_handle().as_raw_fd();
  let clone = attachment.clone();

  drop(attachment);

  assert_eq!(clone.file_handle().as_raw_fd(), descriptor);
}

/// What a guest reports running `script` with `device` as `vda`, on a line
/// containing `report`, once the machine has stopped.
fn guest(name: &str, script: &str, device: &BlockDevice, read_only: bool, report: &str) -> String {
  let file_handle = device.open();
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |machine| {
    let attachment =
      DiskBlockDeviceStorageDeviceAttachment::new(file_handle, read_only, DiskSynchronizationMode::Full).unwrap();

    machine.set_storage_devices(&[&VirtioBlockDeviceConfiguration::new(&attachment).unwrap()]);
  });

  start(&queue, &machine);

  let reported = console.read_until(report);

  stop(&queue, &machine);

  reported
}

#[test]
fn a_guest_reads_and_writes_the_block_device() {
  let name = "a_guest_reads_and_writes_the_block_device";
  let device = block_device(name, b"from the host");
  let script = "read=\"$(head -c 13 /dev/vda)\"\n\
    printf 'from the guest' | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null\n\
    echo \"read:$read\"";

  assert_eq!(
    guest(name, script, &device, false, "read:").trim_end(),
    "read:from the host"
  );
  assert!(device.first_sector().starts_with(b"from the guest"));
}

#[test]
fn a_guest_cannot_write_a_read_only_block_device() {
  let name = "a_guest_cannot_write_a_read_only_block_device";
  let device = block_device(name, b"from the host");
  let script = "printf x | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null && echo write:done || echo write:refused";

  assert_eq!(guest(name, script, &device, true, "write:").trim_end(), "write:refused");
  assert!(device.first_sector().starts_with(b"from the host"));
}

#[test]
fn debugs_as_its_description() {
  let device = block_device("disk_block_debugs_as_its_description", b"");
  let attachment =
    DiskBlockDeviceStorageDeviceAttachment::new(device.open(), true, DiskSynchronizationMode::Full).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZDiskBlockDeviceStorageDeviceAttachment: 0x"));
}
