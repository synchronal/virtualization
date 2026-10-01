#![cfg(all(target_os = "macos", feature = "integration"))]

use std::path::PathBuf;

use virtualization::url::Url;
use virtualization::{DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode, Error};

/// A raw disk image: 1 MiB of zeroes, named for the test that uses it.
fn disk_image(name: &str) -> Url {
  let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}.img"));

  std::fs::write(&path, vec![0u8; 1024 * 1024]).unwrap();

  Url::from_file_path(path).unwrap()
}

#[test]
fn attaches_a_disk_image() {
  let url = disk_image("attaches_a_disk_image");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.is_read_only());
}

#[test]
fn attaches_writable() {
  let url = disk_image("attaches_writable");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, false).unwrap();

  assert!(!attachment.is_read_only());
}

/// Apple documents `init(url:readOnly:)` as automatic caching and full
/// synchronization.
#[test]
fn has_virtualizations_modes_when_none_are_given() {
  let url = disk_image("has_virtualizations_modes_when_none_are_given");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert_eq!(attachment.caching_mode(), DiskImageCachingMode::Automatic);
  assert_eq!(attachment.synchronization_mode(), DiskImageSynchronizationMode::Full);
}

/// Every case crosses to Swift and back as itself.
#[test]
fn attaches_with_each_caching_and_synchronization_mode() {
  let url = disk_image("attaches_with_each_caching_and_synchronization_mode");
  let caching_modes = [
    DiskImageCachingMode::Automatic,
    DiskImageCachingMode::Uncached,
    DiskImageCachingMode::Cached,
  ];
  let synchronization_modes = [
    DiskImageSynchronizationMode::Full,
    DiskImageSynchronizationMode::Fsync,
    DiskImageSynchronizationMode::None,
  ];

  for caching_mode in caching_modes {
    for synchronization_mode in synchronization_modes {
      let attachment = DiskImageStorageDeviceAttachment::with_caching_mode_and_synchronization_mode(
        &url,
        false,
        caching_mode,
        synchronization_mode,
      )
      .unwrap();

      assert!(!attachment.is_read_only());
      assert_eq!(attachment.url().unwrap(), url);
      assert_eq!(attachment.caching_mode(), caching_mode);
      assert_eq!(attachment.synchronization_mode(), synchronization_mode);
    }
  }
}

#[test]
fn a_clone_is_the_same_object() {
  let url = disk_image("a_clone_is_the_same_object");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();
  let clone = attachment.clone();

  drop(attachment);

  assert_eq!(clone.url().unwrap(), url);
  assert!(clone.is_read_only());
}

#[test]
fn fails_as_virtualization_does_with_modes_and_no_image() {
  let url = Url::from_file_path("/nowhere/disk.img").unwrap();

  let result = DiskImageStorageDeviceAttachment::with_caching_mode_and_synchronization_mode(
    &url,
    true,
    DiskImageCachingMode::Cached,
    DiskImageSynchronizationMode::Full,
  );

  assert!(matches!(result, Err(Error::NSError(_))));
}

#[test]
fn fails_as_virtualization_does_when_there_is_no_image() {
  let url = Url::from_file_path("/nowhere/disk.img").unwrap();

  let Err(Error::NSError(error)) = DiskImageStorageDeviceAttachment::new(&url, true) else {
    panic!("attached a disk image that does not exist");
  };

  // What macOS 27 throws: ENOTSUP, not ENOENT.
  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 45, "{error:#?}");
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("Operation not supported")
  );
  assert!(!error.localized_description.is_empty());
}

#[test]
fn fails_as_virtualization_does_on_an_unrecognized_image() {
  let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("unrecognized.img");
  std::fs::write(&path, b"abc").unwrap();
  let url = Url::from_file_path(path).unwrap();

  let Err(Error::NSError(error)) = DiskImageStorageDeviceAttachment::new(&url, true) else {
    panic!("attached a disk image Virtualization can't read");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 5, "{error:#?}");
  assert_eq!(
    error
      .user_info
      .get("NSLocalizedFailureReason")
      .map(String::as_str),
    Some("The disk image format is not recognized.")
  );
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("The disk image format is not recognized.")
  );
}

/// The thrown error is taken from Swift after the call; a second one must not
/// be confused with the first.
#[test]
fn reports_each_thrown_error_as_its_own() {
  let missing = Url::from_file_path("/nowhere/disk.img").unwrap();
  let unrecognized_path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("unrecognized-second.img");
  std::fs::write(&unrecognized_path, b"abc").unwrap();
  let unrecognized = Url::from_file_path(unrecognized_path).unwrap();

  let Err(Error::NSError(first)) = DiskImageStorageDeviceAttachment::new(&missing, true) else {
    panic!("attached a disk image that does not exist");
  };
  let Err(Error::NSError(second)) = DiskImageStorageDeviceAttachment::new(&unrecognized, true) else {
    panic!("attached a disk image Virtualization can't read");
  };

  assert_eq!(first.domain, "NSPOSIXErrorDomain");
  assert_eq!(second.domain, "VZErrorDomain");
}

#[test]
fn debugs_as_its_description() {
  let url = disk_image("debugs_as_its_description");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZDiskImageStorageDeviceAttachment: 0x"));
}
