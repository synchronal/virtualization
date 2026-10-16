use std::fmt;

use crate::GraphicsDisplay;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZVirtioGraphicsScanout`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanout)
pub struct VirtioGraphicsScanout {
  inner: ffi::VZVirtioGraphicsScanout,
}

impl Clone for VirtioGraphicsScanout {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioGraphicsScanout {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioGraphicsScanout {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_graphics_scanout_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDisplay for VirtioGraphicsScanout {}
