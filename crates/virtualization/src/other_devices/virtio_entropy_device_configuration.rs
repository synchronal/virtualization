use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{EntropyDeviceConfiguration, Error};

/// [`VZVirtioEntropyDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtioentropydeviceconfiguration)
pub struct VirtioEntropyDeviceConfiguration {
  inner: ffi::VZVirtioEntropyDeviceConfiguration,
}

impl VirtioEntropyDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtioentropydeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_entropy_device_configuration_init(),
    })
  }
}

impl Clone for VirtioEntropyDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioEntropyDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioEntropyDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_entropy_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl EntropyDeviceConfiguration for VirtioEntropyDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioEntropyDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
