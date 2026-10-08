use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{ConsoleDeviceConfiguration, Error, VirtioConsolePortConfigurationArray};

/// [`VZVirtioConsoleDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceconfiguration)
pub struct VirtioConsoleDeviceConfiguration {
  inner: ffi::VZVirtioConsoleDeviceConfiguration,
}

impl VirtioConsoleDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_console_device_configuration_init(),
    })
  }

  /// [`ports`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceconfiguration/ports)
  pub fn ports(&self) -> VirtioConsolePortConfigurationArray {
    VirtioConsolePortConfigurationArray::bridge_from(self.inner.vz_ports())
  }
}

impl Clone for VirtioConsoleDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsoleDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioConsoleDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_console_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl ConsoleDeviceConfiguration for VirtioConsoleDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioConsoleDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
