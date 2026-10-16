use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, GraphicsDisplayConfiguration};

/// [`VZVirtioGraphicsScanoutConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration)
pub struct VirtioGraphicsScanoutConfiguration {
  pub(crate) inner: ffi::VZVirtioGraphicsScanoutConfiguration,
}

impl VirtioGraphicsScanoutConfiguration {
  /// [`init(widthInPixels:heightInPixels:)`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration/init(widthinpixels:heightinpixels:))
  pub fn new(width_in_pixels: isize, height_in_pixels: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_graphics_scanout_configuration_init(width_in_pixels, height_in_pixels),
    })
  }

  /// [`widthInPixels`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration/widthinpixels)
  pub fn width_in_pixels(&self) -> isize {
    self.inner.vz_width_in_pixels()
  }

  /// [`widthInPixels`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration/widthinpixels)
  pub fn set_width_in_pixels(&self, width_in_pixels: isize) {
    self.inner.vz_set_width_in_pixels(width_in_pixels);
  }

  /// [`heightInPixels`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration/heightinpixels)
  pub fn height_in_pixels(&self) -> isize {
    self.inner.vz_height_in_pixels()
  }

  /// [`heightInPixels`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanoutconfiguration/heightinpixels)
  pub fn set_height_in_pixels(&self, height_in_pixels: isize) {
    self.inner.vz_set_height_in_pixels(height_in_pixels);
  }
}

impl Clone for VirtioGraphicsScanoutConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioGraphicsScanoutConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioGraphicsScanoutConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_graphics_scanout_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDisplayConfiguration for VirtioGraphicsScanoutConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioGraphicsScanoutConfiguration::new(1920, 1080),
      Err(Error::Unsupported)
    ));
  }
}
