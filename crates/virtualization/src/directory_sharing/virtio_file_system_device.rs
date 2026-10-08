use std::fmt;

use crate::platform::ffi;
use crate::sealed::Sealed;
use crate::{DirectoryShare, DirectorySharingDevice};

/// [`VZVirtioFileSystemDevice`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdevice)
pub struct VirtioFileSystemDevice {
  inner: ffi::VZVirtioFileSystemDevice,
}

impl VirtioFileSystemDevice {
  /// [`tag`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdevice/tag)
  pub fn tag(&self) -> String {
    self.inner.vz_tag()
  }

  /// [`share`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdevice/share),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn share<T: DirectoryShare>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_share()?)
  }

  /// [`share`](https://developer.apple.com/documentation/virtualization/vzvirtiofilesystemdevice/share)
  pub fn set_share(&self, share: Option<&dyn DirectoryShare>) {
    self
      .inner
      .vz_set_share(share.map(|share| share.bridge_object()));
  }
}

impl Clone for VirtioFileSystemDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioFileSystemDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioFileSystemDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_file_system_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl DirectorySharingDevice for VirtioFileSystemDevice {}
