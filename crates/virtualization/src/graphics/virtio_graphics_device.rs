use std::fmt;

use crate::GraphicsDevice;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZVirtioGraphicsDevice`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsdevice)
pub struct VirtioGraphicsDevice {
  inner: ffi::VZVirtioGraphicsDevice,
}

impl Clone for VirtioGraphicsDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioGraphicsDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioGraphicsDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_graphics_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDevice for VirtioGraphicsDevice {}
