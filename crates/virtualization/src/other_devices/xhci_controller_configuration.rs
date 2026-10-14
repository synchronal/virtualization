use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, USBControllerConfiguration};

/// [`VZXHCIControllerConfiguration`](https://developer.apple.com/documentation/virtualization/vzxhcicontrollerconfiguration)
pub struct XHCIControllerConfiguration {
  inner: ffi::VZXHCIControllerConfiguration,
}

impl XHCIControllerConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzxhcicontrollerconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_xhci_controller_configuration_init(),
    })
  }
}

impl Clone for XHCIControllerConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for XHCIControllerConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for XHCIControllerConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_xhci_controller_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl USBControllerConfiguration for XHCIControllerConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(XHCIControllerConfiguration::new(), Err(Error::Unsupported)));
  }
}
