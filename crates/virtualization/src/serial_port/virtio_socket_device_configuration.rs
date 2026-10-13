use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, SocketDeviceConfiguration};

/// [`VZVirtioSocketDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdeviceconfiguration)
pub struct VirtioSocketDeviceConfiguration {
  inner: ffi::VZVirtioSocketDeviceConfiguration,
}

impl VirtioSocketDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_socket_device_configuration_init(),
    })
  }
}

impl Clone for VirtioSocketDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSocketDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioSocketDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_socket_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SocketDeviceConfiguration for VirtioSocketDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioSocketDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
