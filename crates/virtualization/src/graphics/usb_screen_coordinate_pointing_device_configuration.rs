use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, PointingDeviceConfiguration};

/// [`VZUSBScreenCoordinatePointingDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbscreencoordinatepointingdeviceconfiguration)
pub struct USBScreenCoordinatePointingDeviceConfiguration {
  inner: ffi::VZUSBScreenCoordinatePointingDeviceConfiguration,
}

impl USBScreenCoordinatePointingDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzusbscreencoordinatepointingdeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_screen_coordinate_pointing_device_configuration_init(),
    })
  }
}

impl Clone for USBScreenCoordinatePointingDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBScreenCoordinatePointingDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBScreenCoordinatePointingDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_screen_coordinate_pointing_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl PointingDeviceConfiguration for USBScreenCoordinatePointingDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      USBScreenCoordinatePointingDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
