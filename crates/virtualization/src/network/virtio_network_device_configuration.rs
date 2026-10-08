use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, NetworkDeviceConfiguration};

/// [`VZVirtioNetworkDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtionetworkdeviceconfiguration)
pub struct VirtioNetworkDeviceConfiguration {
  inner: ffi::VZVirtioNetworkDeviceConfiguration,
}

impl VirtioNetworkDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtionetworkdeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_network_device_configuration_init(),
    })
  }
}

impl Clone for VirtioNetworkDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioNetworkDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioNetworkDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_network_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl NetworkDeviceConfiguration for VirtioNetworkDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioNetworkDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
