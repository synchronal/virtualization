use std::fmt;
use std::os::fd::{BorrowedFd, OwnedFd};

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDeviceAttachment, foundation};

/// [`VZDiskBlockDeviceStorageDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzdiskblockdevicestoragedeviceattachment)
///
/// A `FileHandle` is the descriptor it wraps: the one given becomes a handle
/// that owns it, and the one returned is borrowed from the attachment, which
/// keeps its handle.
pub struct DiskBlockDeviceStorageDeviceAttachment {
  inner: ffi::VZDiskBlockDeviceStorageDeviceAttachment,
}

impl DiskBlockDeviceStorageDeviceAttachment {
  /// [`init(fileHandle:readOnly:synchronizationMode:)`](https://developer.apple.com/documentation/virtualization/vzdiskblockdevicestoragedeviceattachment/init(filehandle:readonly:synchronizationmode:))
  pub fn new(
    file_handle: OwnedFd,
    read_only: bool,
    synchronization_mode: DiskSynchronizationMode,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_disk_block_device_storage_device_attachment_init(
        file_handle,
        read_only,
        synchronization_mode.into(),
      )
      .map_err(foundation::thrown)?,
    })
  }

  /// [`fileHandle`](https://developer.apple.com/documentation/virtualization/vzdiskblockdevicestoragedeviceattachment/filehandle)
  pub fn file_handle(&self) -> BorrowedFd<'_> {
    self.inner.vz_file_handle()
  }

  /// [`isReadOnly`](https://developer.apple.com/documentation/virtualization/vzdiskblockdevicestoragedeviceattachment/isreadonly)
  pub fn is_read_only(&self) -> bool {
    self.inner.vz_is_read_only()
  }

  /// [`synchronizationMode`](https://developer.apple.com/documentation/virtualization/vzdiskblockdevicestoragedeviceattachment/synchronizationmode)
  pub fn synchronization_mode(&self) -> DiskSynchronizationMode {
    self.inner.vz_synchronization_mode().into()
  }
}

impl Clone for DiskBlockDeviceStorageDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for DiskBlockDeviceStorageDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for DiskBlockDeviceStorageDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_disk_block_device_storage_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDeviceAttachment for DiskBlockDeviceStorageDeviceAttachment {}

/// [`VZDiskSynchronizationMode`](https://developer.apple.com/documentation/virtualization/vzdisksynchronizationmode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiskSynchronizationMode {
  /// [`full`](https://developer.apple.com/documentation/virtualization/vzdisksynchronizationmode/full)
  Full,
  /// [`none`](https://developer.apple.com/documentation/virtualization/vzdisksynchronizationmode/none)
  None,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl From<DiskSynchronizationMode> for ffi::VZDiskSynchronizationMode {
  fn from(mode: DiskSynchronizationMode) -> Self {
    match mode {
      DiskSynchronizationMode::Full => Self::Full,
      DiskSynchronizationMode::None => Self::None,
      DiskSynchronizationMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::VZDiskSynchronizationMode> for DiskSynchronizationMode {
  fn from(mode: ffi::VZDiskSynchronizationMode) -> Self {
    match mode {
      ffi::VZDiskSynchronizationMode::Full => Self::Full,
      ffi::VZDiskSynchronizationMode::None => Self::None,
      ffi::VZDiskSynchronizationMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn synchronization_mode_becomes_the_same_case_in_virtualization_sys() {
    assert!(matches!(
      ffi::VZDiskSynchronizationMode::from(DiskSynchronizationMode::Full),
      ffi::VZDiskSynchronizationMode::Full
    ));
    assert!(matches!(
      ffi::VZDiskSynchronizationMode::from(DiskSynchronizationMode::None),
      ffi::VZDiskSynchronizationMode::None
    ));
    assert!(matches!(
      ffi::VZDiskSynchronizationMode::from(DiskSynchronizationMode::Unknown(7)),
      ffi::VZDiskSynchronizationMode::Unknown(7)
    ));
  }

  #[test]
  fn synchronization_mode_comes_from_the_same_case_in_virtualization_sys() {
    assert_eq!(
      DiskSynchronizationMode::from(ffi::VZDiskSynchronizationMode::Full),
      DiskSynchronizationMode::Full
    );
    assert_eq!(
      DiskSynchronizationMode::from(ffi::VZDiskSynchronizationMode::None),
      DiskSynchronizationMode::None
    );
    assert_eq!(
      DiskSynchronizationMode::from(ffi::VZDiskSynchronizationMode::Unknown(7)),
      DiskSynchronizationMode::Unknown(7)
    );
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    let file_handle = OwnedFd::from(std::fs::File::open("/dev/null").unwrap());

    assert!(matches!(
      DiskBlockDeviceStorageDeviceAttachment::new(file_handle, true, DiskSynchronizationMode::Full),
      Err(Error::Unsupported)
    ));
  }
}
