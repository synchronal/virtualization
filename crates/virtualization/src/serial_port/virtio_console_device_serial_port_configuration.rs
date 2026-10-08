use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, SerialPortConfiguration};

/// [`VZVirtioConsoleDeviceSerialPortConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceserialportconfiguration)
pub struct VirtioConsoleDeviceSerialPortConfiguration {
  inner: ffi::VZVirtioConsoleDeviceSerialPortConfiguration,
}

impl VirtioConsoleDeviceSerialPortConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledeviceserialportconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_console_device_serial_port_configuration_init(),
    })
  }
}

impl Clone for VirtioConsoleDeviceSerialPortConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsoleDeviceSerialPortConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioConsoleDeviceSerialPortConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_console_device_serial_port_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SerialPortConfiguration for VirtioConsoleDeviceSerialPortConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioConsoleDeviceSerialPortConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
