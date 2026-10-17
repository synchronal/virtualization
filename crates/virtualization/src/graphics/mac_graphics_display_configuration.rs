use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{CGSize, Error, GraphicsDisplayConfiguration, NSScreen};

/// [`VZMacGraphicsDisplayConfiguration`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration)
pub struct MacGraphicsDisplayConfiguration {
  pub(crate) inner: ffi::VZMacGraphicsDisplayConfiguration,
}

impl MacGraphicsDisplayConfiguration {
  /// [`init(widthInPixels:heightInPixels:pixelsPerInch:)`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/init(widthinpixels:heightinpixels:pixelsperinch:))
  pub fn new(width_in_pixels: isize, height_in_pixels: isize, pixels_per_inch: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_graphics_display_configuration_init(width_in_pixels, height_in_pixels, pixels_per_inch),
    })
  }

  /// [`init(for:sizeInPoints:)`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/init(for:sizeinpoints:)).
  /// `NSSize` is a `CGSize`.
  pub fn with_for_and_size_in_points(screen: &NSScreen, size_in_points: CGSize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_graphics_display_configuration_init_for_size_in_points(&screen.inner, size_in_points.into()),
    })
  }

  /// [`widthInPixels`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/widthinpixels)
  pub fn width_in_pixels(&self) -> isize {
    self.inner.vz_width_in_pixels()
  }

  /// [`widthInPixels`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/widthinpixels)
  pub fn set_width_in_pixels(&self, width_in_pixels: isize) {
    self.inner.vz_set_width_in_pixels(width_in_pixels);
  }

  /// [`heightInPixels`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/heightinpixels)
  pub fn height_in_pixels(&self) -> isize {
    self.inner.vz_height_in_pixels()
  }

  /// [`heightInPixels`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/heightinpixels)
  pub fn set_height_in_pixels(&self, height_in_pixels: isize) {
    self.inner.vz_set_height_in_pixels(height_in_pixels);
  }

  /// [`pixelsPerInch`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/pixelsperinch)
  pub fn pixels_per_inch(&self) -> isize {
    self.inner.vz_pixels_per_inch()
  }

  /// [`pixelsPerInch`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdisplayconfiguration/pixelsperinch)
  pub fn set_pixels_per_inch(&self, pixels_per_inch: isize) {
    self.inner.vz_set_pixels_per_inch(pixels_per_inch);
  }
}

impl Clone for MacGraphicsDisplayConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacGraphicsDisplayConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacGraphicsDisplayConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_mac_graphics_display_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDisplayConfiguration for MacGraphicsDisplayConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      MacGraphicsDisplayConfiguration::new(1920, 1080, 80),
      Err(Error::Unsupported)
    ));
  }
}
