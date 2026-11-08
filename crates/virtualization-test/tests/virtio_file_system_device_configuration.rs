use std::collections::BTreeMap;
use std::path::Path;

use dispatch2::{DispatchQueue, DispatchRetained};
use url::Url;
use virtualization::{
  ErrorCode, MultipleDirectoryShare, SharedDirectory, SingleDirectoryShare, VirtioFileSystemDeviceConfiguration,
  VirtualMachine,
};
use virtualization_test::{Console, booting_machine, directory, initial_ramdisk, linux_configuration, ns_error, start};

#[test]
fn has_the_tag_it_was_made_with() {
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  assert_eq!(device.tag(), "share");

  device.set_tag("other");

  assert_eq!(device.tag(), "other");
}

/// Virtualization's limit is 36 bytes, though Apple documents a tag as under
/// 36.
#[test]
fn validates_a_tag_of_36_bytes() {
  assert!(VirtioFileSystemDeviceConfiguration::validate_tag(&"a".repeat(36)).is_ok());
}

#[test]
fn fails_as_virtualization_does_on_a_tag_of_37_bytes() {
  let error = ns_error(VirtioFileSystemDeviceConfiguration::validate_tag(&"a".repeat(37)));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn fails_as_virtualization_does_on_an_empty_tag() {
  let error = ns_error(VirtioFileSystemDeviceConfiguration::validate_tag(""));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn has_the_macos_guest_automount_tag() {
  assert_eq!(
    VirtioFileSystemDeviceConfiguration::macos_guest_automount_tag().unwrap(),
    "com.apple.virtio-fs.automount"
  );
}

#[test]
fn has_no_share_when_new() {
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  assert!(device.share::<SingleDirectoryShare>().is_none());
  assert!(device.share::<MultipleDirectoryShare>().is_none());
}

#[test]
fn has_the_share_it_was_given() {
  let url = Url::from_directory_path(directory("has_the_share_it_was_given", &[])).unwrap();
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  device.set_share(Some(
    &SingleDirectoryShare::new(&SharedDirectory::new(&url, false).unwrap()).unwrap(),
  ));

  assert_eq!(
    device
      .share::<SingleDirectoryShare>()
      .unwrap()
      .directory()
      .url()
      .unwrap(),
    url
  );
  assert!(device.share::<MultipleDirectoryShare>().is_none());

  device.set_share(None);

  assert!(device.share::<SingleDirectoryShare>().is_none());
}

#[test]
fn is_a_directory_sharing_device_of_the_configuration_it_was_given_to() {
  let name = "is_a_directory_sharing_device_of_the_configuration_it_was_given_to";
  let url = Url::from_directory_path(directory(name, &[])).unwrap();
  let configuration = linux_configuration(name);
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  device.set_share(Some(
    &SingleDirectoryShare::new(&SharedDirectory::new(&url, false).unwrap()).unwrap(),
  ));
  configuration.set_directory_sharing_devices(&[&device]);

  let given = configuration.directory_sharing_devices::<VirtioFileSystemDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert_eq!(given[0].as_ref().unwrap().tag(), "share");
  configuration.validate().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  device.clone().set_tag("other");

  assert_eq!(device.tag(), "other");
}

#[test]
fn debugs_as_its_description() {
  let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

  assert!(format!("{device:?}").starts_with("<VZVirtioFileSystemDeviceConfiguration: 0x"));
}

/// Mounts the device tagged `share` at `/mnt`.
const MOUNT: &str = "mkdir /mnt\nmount -t virtiofs share /mnt || echo mount:failed";

/// A started guest that mounts a device tagged `share`, sharing `path` alone,
/// then runs `script`.
fn single_share_guest(
  name: &str,
  script: &str,
  path: &Path,
  read_only: bool,
) -> (DispatchRetained<DispatchQueue>, VirtualMachine, Console) {
  let url = Url::from_directory_path(path).unwrap();
  let (queue, machine, console) = booting_machine(
    name,
    Some(&initial_ramdisk(name, &format!("{MOUNT}\n{script}"))),
    |configuration| {
      let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

      device.set_share(Some(
        &SingleDirectoryShare::new(&SharedDirectory::new(&url, read_only).unwrap()).unwrap(),
      ));
      configuration.set_directory_sharing_devices(&[&device]);
    },
  );

  start(&queue, &machine);

  (queue, machine, console)
}

#[test]
fn a_guest_reads_a_file_the_host_wrote() {
  let name = "a_guest_reads_a_file_the_host_wrote";
  let path = directory(name, &[("hello", "from the host")]);
  let (_queue, _machine, mut console) = single_share_guest(name, "echo \"read:$(cat /mnt/hello)\"", &path, true);

  assert_eq!(console.read_until("read:").trim_end(), "read:from the host");
}

#[test]
fn the_host_reads_a_file_the_guest_wrote() {
  let name = "the_host_reads_a_file_the_guest_wrote";
  let path = directory(name, &[]);
  let (_queue, _machine, mut console) = single_share_guest(
    name,
    "echo from the guest > /mnt/written\numount /mnt\necho wrote",
    &path,
    false,
  );

  console.read_until("wrote");

  assert_eq!(
    std::fs::read_to_string(path.join("written")).unwrap(),
    "from the guest\n"
  );
}

#[test]
fn a_read_only_share_refuses_the_guest_s_writes() {
  let name = "a_read_only_share_refuses_the_guest_s_writes";
  let path = directory(name, &[]);
  let (_queue, _machine, mut console) = single_share_guest(
    name,
    "echo from the guest > /mnt/written && echo write:written || echo write:refused",
    &path,
    true,
  );

  assert_eq!(console.read_until("write:").trim_end(), "write:refused");
  assert!(!path.join("written").exists());
}

#[test]
fn a_guest_sees_each_directory_of_a_multiple_share_by_its_name() {
  let name = "a_guest_sees_each_directory_of_a_multiple_share_by_its_name";
  let first = Url::from_directory_path(directory(&format!("{name}_first"), &[("which", "first")])).unwrap();
  let second = Url::from_directory_path(directory(&format!("{name}_second"), &[("which", "second")])).unwrap();
  let script = format!("{MOUNT}\necho \"names:$(ls /mnt | xargs)\"\necho \"second:$(cat /mnt/second/which)\"");
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, &script)), |configuration| {
    let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

    device.set_share(Some(
      &MultipleDirectoryShare::with_directories(&BTreeMap::from([
        ("first".to_string(), SharedDirectory::new(&first, true).unwrap()),
        ("second".to_string(), SharedDirectory::new(&second, true).unwrap()),
      ]))
      .unwrap(),
    ));
    configuration.set_directory_sharing_devices(&[&device]);
  });

  start(&queue, &machine);

  assert_eq!(console.read_until("names:").trim_end(), "names:first second");
  assert_eq!(console.read_until("second:").trim_end(), "second:second");
}
