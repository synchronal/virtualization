use std::fmt;

use crate::GraphicsDisplay;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZMacGraphicsDisplay`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplay)
pub struct MacGraphicsDisplay {
  inner: ffi::VZMacGraphicsDisplay,
}

impl MacGraphicsDisplay {
  /// [`pixelsPerInch`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplay/pixelsperinch)
  pub fn pixels_per_inch(&self) -> isize {
    self.inner.vz_pixels_per_inch()
  }
}

impl Clone for MacGraphicsDisplay {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacGraphicsDisplay {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacGraphicsDisplay {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_mac_graphics_display_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDisplay for MacGraphicsDisplay {}
