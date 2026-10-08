use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{DirectoryShare, DirectorySharingDeviceConfiguration, Error, foundation};

/// [`VZVirtioFileSystemDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration)
pub struct VirtioFileSystemDeviceConfiguration {
  inner: ffi::VZVirtioFileSystemDeviceConfiguration,
}

impl VirtioFileSystemDeviceConfiguration {
  /// [`init(tag:)`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/init(tag:))
  pub fn new(tag: &str) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_file_system_device_configuration_init(tag),
    })
  }

  /// [`validateTag(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/validatetag(_:))
  pub fn validate_tag(tag: &str) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_virtio_file_system_device_configuration_validate_tag(tag).map_err(foundation::thrown)
  }

  /// [`macOSGuestAutomountTag`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/macosguestautomounttag)
  pub fn macos_guest_automount_tag() -> Result<String, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtio_file_system_device_configuration_macos_guest_automount_tag())
  }

  /// [`tag`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/tag)
  pub fn tag(&self) -> String {
    self.inner.vz_tag()
  }

  /// [`tag`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/tag)
  pub fn set_tag(&self, tag: &str) {
    self.inner.vz_set_tag(tag);
  }

  /// [`share`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/share),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn share<T: DirectoryShare>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_share()?)
  }

  /// [`share`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdeviceconfiguration/share)
  pub fn set_share(&self, share: Option<&dyn DirectoryShare>) {
    self
      .inner
      .vz_set_share(share.map(|share| share.bridge_object()));
  }
}

impl Clone for VirtioFileSystemDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioFileSystemDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioFileSystemDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_file_system_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl DirectorySharingDeviceConfiguration for VirtioFileSystemDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioFileSystemDeviceConfiguration::new("tag"),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      VirtioFileSystemDeviceConfiguration::validate_tag("tag"),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      VirtioFileSystemDeviceConfiguration::macos_guest_automount_tag(),
      Err(Error::Unsupported)
    ));
  }
}
