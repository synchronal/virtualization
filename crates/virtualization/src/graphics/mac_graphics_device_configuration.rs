use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, GraphicsDeviceConfiguration, MacGraphicsDisplayConfiguration};

/// [`VZMacGraphicsDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdeviceconfiguration)
pub struct MacGraphicsDeviceConfiguration {
  inner: ffi::VZMacGraphicsDeviceConfiguration,
}

impl MacGraphicsDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_graphics_device_configuration_init(),
    })
  }

  /// [`displays`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdeviceconfiguration/displays)
  pub fn displays(&self) -> Vec<MacGraphicsDisplayConfiguration> {
    self
      .inner
      .vz_displays()
      .into_iter()
      .map(|inner| MacGraphicsDisplayConfiguration { inner })
      .collect()
  }

  /// [`displays`](https://developer.apple.com/documentation/virtualization/vzmacgraphicsdeviceconfiguration/displays)
  pub fn set_displays(&self, displays: &[&MacGraphicsDisplayConfiguration]) {
    let displays: Vec<_> = displays.iter().map(|display| &display.inner).collect();

    self.inner.vz_set_displays(&displays);
  }
}

impl Clone for MacGraphicsDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacGraphicsDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacGraphicsDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_mac_graphics_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDeviceConfiguration for MacGraphicsDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacGraphicsDeviceConfiguration::new(), Err(Error::Unsupported)));
  }
}
