use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::{Sealed, bridge_cast_each, bridge_objects};
use crate::{AudioDeviceConfiguration, Error, VirtioSoundDeviceStreamConfiguration};

/// [`VZVirtioSoundDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceconfiguration)
pub struct VirtioSoundDeviceConfiguration {
  inner: ffi::VZVirtioSoundDeviceConfiguration,
}

impl VirtioSoundDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_sound_device_configuration_init(),
    })
  }

  /// [`streams`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceconfiguration/streams),
  /// each as Swift's `as? T`: `None` for one that isn't a `T`.
  pub fn streams<T: VirtioSoundDeviceStreamConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_streams())
  }

  /// [`streams`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceconfiguration/streams)
  pub fn set_streams(&self, streams: &[&dyn VirtioSoundDeviceStreamConfiguration]) {
    self.inner.vz_set_streams(&bridge_objects(streams));
  }
}

impl Clone for VirtioSoundDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSoundDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioSoundDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_sound_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl AudioDeviceConfiguration for VirtioSoundDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtioSoundDeviceConfiguration::new(), Err(Error::Unsupported)));
  }
}
