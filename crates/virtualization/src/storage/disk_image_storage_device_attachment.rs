use std::fmt;

use url::Url;

use crate::foundation;
use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDeviceAttachment};

/// [`VZDiskImageStorageDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment)
pub struct DiskImageStorageDeviceAttachment {
  inner: ffi::VZDiskImageStorageDeviceAttachment,
}

impl DiskImageStorageDeviceAttachment {
  /// [`init(url:readOnly:)`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/init(url:readonly:))
  pub fn new(url: &Url, read_only: bool) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_disk_image_storage_device_attachment_init(foundation::ns_url(url)?, read_only)
        .map_err(foundation::thrown)?,
    })
  }

  /// [`init(url:readOnly:cachingMode:synchronizationMode:)`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/init(url:readonly:cachingmode:synchronizationmode:))
  pub fn with_caching_mode_and_synchronization_mode(
    url: &Url,
    read_only: bool,
    caching_mode: DiskImageCachingMode,
    synchronization_mode: DiskImageSynchronizationMode,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
        foundation::ns_url(url)?,
        read_only,
        caching_mode.into(),
        synchronization_mode.into(),
      )
      .map_err(foundation::thrown)?,
    })
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`isReadOnly`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/isreadonly)
  pub fn is_read_only(&self) -> bool {
    self.inner.vz_is_read_only()
  }

  /// [`cachingMode`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/cachingmode)
  pub fn caching_mode(&self) -> DiskImageCachingMode {
    self.inner.vz_caching_mode().into()
  }

  /// [`synchronizationMode`](https://developer.apple.com/documentation/virtualization/vzdiskimagestoragedeviceattachment/synchronizationmode)
  pub fn synchronization_mode(&self) -> DiskImageSynchronizationMode {
    self.inner.vz_synchronization_mode().into()
  }
}

impl Clone for DiskImageStorageDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for DiskImageStorageDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for DiskImageStorageDeviceAttachment {}

impl StorageDeviceAttachment for DiskImageStorageDeviceAttachment {}

/// [`VZDiskImageCachingMode`](https://developer.apple.com/documentation/virtualization/vzdiskimagecachingmode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiskImageCachingMode {
  /// [`automatic`](https://developer.apple.com/documentation/virtualization/vzdiskimagecachingmode/automatic)
  Automatic,
  /// [`uncached`](https://developer.apple.com/documentation/virtualization/vzdiskimagecachingmode/uncached)
  Uncached,
  /// [`cached`](https://developer.apple.com/documentation/virtualization/vzdiskimagecachingmode/cached)
  Cached,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl From<DiskImageCachingMode> for ffi::VZDiskImageCachingMode {
  fn from(mode: DiskImageCachingMode) -> Self {
    match mode {
      DiskImageCachingMode::Automatic => Self::Automatic,
      DiskImageCachingMode::Uncached => Self::Uncached,
      DiskImageCachingMode::Cached => Self::Cached,
      DiskImageCachingMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::VZDiskImageCachingMode> for DiskImageCachingMode {
  fn from(mode: ffi::VZDiskImageCachingMode) -> Self {
    match mode {
      ffi::VZDiskImageCachingMode::Automatic => Self::Automatic,
      ffi::VZDiskImageCachingMode::Uncached => Self::Uncached,
      ffi::VZDiskImageCachingMode::Cached => Self::Cached,
      ffi::VZDiskImageCachingMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`VZDiskImageSynchronizationMode`](https://developer.apple.com/documentation/virtualization/vzdiskimagesynchronizationmode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiskImageSynchronizationMode {
  /// [`full`](https://developer.apple.com/documentation/virtualization/vzdiskimagesynchronizationmode/full)
  Full,
  /// [`fsync`](https://developer.apple.com/documentation/virtualization/vzdiskimagesynchronizationmode/fsync)
  Fsync,
  /// [`none`](https://developer.apple.com/documentation/virtualization/vzdiskimagesynchronizationmode/none)
  None,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl From<DiskImageSynchronizationMode> for ffi::VZDiskImageSynchronizationMode {
  fn from(mode: DiskImageSynchronizationMode) -> Self {
    match mode {
      DiskImageSynchronizationMode::Full => Self::Full,
      DiskImageSynchronizationMode::Fsync => Self::Fsync,
      DiskImageSynchronizationMode::None => Self::None,
      DiskImageSynchronizationMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::VZDiskImageSynchronizationMode> for DiskImageSynchronizationMode {
  fn from(mode: ffi::VZDiskImageSynchronizationMode) -> Self {
    match mode {
      ffi::VZDiskImageSynchronizationMode::Full => Self::Full,
      ffi::VZDiskImageSynchronizationMode::Fsync => Self::Fsync,
      ffi::VZDiskImageSynchronizationMode::None => Self::None,
      ffi::VZDiskImageSynchronizationMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn caching_mode_becomes_the_same_case_in_virtualization_sys() {
    assert!(matches!(
      ffi::VZDiskImageCachingMode::from(DiskImageCachingMode::Automatic),
      ffi::VZDiskImageCachingMode::Automatic
    ));
    assert!(matches!(
      ffi::VZDiskImageCachingMode::from(DiskImageCachingMode::Uncached),
      ffi::VZDiskImageCachingMode::Uncached
    ));
    assert!(matches!(
      ffi::VZDiskImageCachingMode::from(DiskImageCachingMode::Cached),
      ffi::VZDiskImageCachingMode::Cached
    ));
    assert!(matches!(
      ffi::VZDiskImageCachingMode::from(DiskImageCachingMode::Unknown(7)),
      ffi::VZDiskImageCachingMode::Unknown(7)
    ));
  }

  #[test]
  fn caching_mode_comes_from_the_same_case_in_virtualization_sys() {
    assert_eq!(
      DiskImageCachingMode::from(ffi::VZDiskImageCachingMode::Automatic),
      DiskImageCachingMode::Automatic
    );
    assert_eq!(
      DiskImageCachingMode::from(ffi::VZDiskImageCachingMode::Uncached),
      DiskImageCachingMode::Uncached
    );
    assert_eq!(
      DiskImageCachingMode::from(ffi::VZDiskImageCachingMode::Cached),
      DiskImageCachingMode::Cached
    );
    assert_eq!(
      DiskImageCachingMode::from(ffi::VZDiskImageCachingMode::Unknown(7)),
      DiskImageCachingMode::Unknown(7)
    );
  }

  #[test]
  fn synchronization_mode_becomes_the_same_case_in_virtualization_sys() {
    assert!(matches!(
      ffi::VZDiskImageSynchronizationMode::from(DiskImageSynchronizationMode::Full),
      ffi::VZDiskImageSynchronizationMode::Full
    ));
    assert!(matches!(
      ffi::VZDiskImageSynchronizationMode::from(DiskImageSynchronizationMode::Fsync),
      ffi::VZDiskImageSynchronizationMode::Fsync
    ));
    assert!(matches!(
      ffi::VZDiskImageSynchronizationMode::from(DiskImageSynchronizationMode::None),
      ffi::VZDiskImageSynchronizationMode::None
    ));
    assert!(matches!(
      ffi::VZDiskImageSynchronizationMode::from(DiskImageSynchronizationMode::Unknown(7)),
      ffi::VZDiskImageSynchronizationMode::Unknown(7)
    ));
  }

  #[test]
  fn synchronization_mode_comes_from_the_same_case_in_virtualization_sys() {
    assert_eq!(
      DiskImageSynchronizationMode::from(ffi::VZDiskImageSynchronizationMode::Full),
      DiskImageSynchronizationMode::Full
    );
    assert_eq!(
      DiskImageSynchronizationMode::from(ffi::VZDiskImageSynchronizationMode::Fsync),
      DiskImageSynchronizationMode::Fsync
    );
    assert_eq!(
      DiskImageSynchronizationMode::from(ffi::VZDiskImageSynchronizationMode::None),
      DiskImageSynchronizationMode::None
    );
    assert_eq!(
      DiskImageSynchronizationMode::from(ffi::VZDiskImageSynchronizationMode::Unknown(7)),
      DiskImageSynchronizationMode::Unknown(7)
    );
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/disk.img").unwrap();

    assert!(matches!(
      DiskImageStorageDeviceAttachment::new(&url, true),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      DiskImageStorageDeviceAttachment::with_caching_mode_and_synchronization_mode(
        &url,
        true,
        DiskImageCachingMode::Automatic,
        DiskImageSynchronizationMode::Full,
      ),
      Err(Error::Unsupported)
    ));
  }
}
