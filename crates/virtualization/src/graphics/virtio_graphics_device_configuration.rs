use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, GraphicsDeviceConfiguration, VirtioGraphicsScanoutConfiguration};

/// [`VZVirtioGraphicsDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsdeviceconfiguration)
pub struct VirtioGraphicsDeviceConfiguration {
  inner: ffi::VZVirtioGraphicsDeviceConfiguration,
}

impl VirtioGraphicsDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsdeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_graphics_device_configuration_init(),
    })
  }

  /// [`scanouts`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsdeviceconfiguration/scanouts)
  pub fn scanouts(&self) -> Vec<VirtioGraphicsScanoutConfiguration> {
    self
      .inner
      .vz_scanouts()
      .into_iter()
      .map(|inner| VirtioGraphicsScanoutConfiguration { inner })
      .collect()
  }

  /// [`scanouts`](https://developer.apple.com/documentation/virtualization/vzvirtiographicsdeviceconfiguration/scanouts)
  pub fn set_scanouts(&self, scanouts: &[&VirtioGraphicsScanoutConfiguration]) {
    let scanouts: Vec<_> = scanouts.iter().map(|scanout| &scanout.inner).collect();

    self.inner.vz_set_scanouts(&scanouts);
  }
}

impl Clone for VirtioGraphicsDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioGraphicsDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioGraphicsDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_graphics_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GraphicsDeviceConfiguration for VirtioGraphicsDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioGraphicsDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
