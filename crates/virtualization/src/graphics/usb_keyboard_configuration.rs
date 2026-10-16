use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, KeyboardConfiguration};

/// [`VZUSBKeyboardConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbkeyboardconfiguration)
pub struct USBKeyboardConfiguration {
  inner: ffi::VZUSBKeyboardConfiguration,
}

impl USBKeyboardConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzusbkeyboardconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_keyboard_configuration_init(),
    })
  }
}

impl Clone for USBKeyboardConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBKeyboardConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBKeyboardConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_keyboard_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl KeyboardConfiguration for USBKeyboardConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(USBKeyboardConfiguration::new(), Err(Error::Unsupported)));
  }
}
