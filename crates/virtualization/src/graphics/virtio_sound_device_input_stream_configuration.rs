use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{AudioInputStreamSource, Error, VirtioSoundDeviceStreamConfiguration};

/// [`VZVirtioSoundDeviceInputStreamConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceinputstreamconfiguration)
pub struct VirtioSoundDeviceInputStreamConfiguration {
  inner: ffi::VZVirtioSoundDeviceInputStreamConfiguration,
}

impl VirtioSoundDeviceInputStreamConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceinputstreamconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_sound_device_input_stream_configuration_init(),
    })
  }

  /// [`source`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceinputstreamconfiguration/source),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn source<T: AudioInputStreamSource>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_source()?)
  }

  /// [`source`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceinputstreamconfiguration/source)
  pub fn set_source(&self, source: Option<&dyn AudioInputStreamSource>) {
    self
      .inner
      .vz_set_source(source.map(|source| source.bridge_object()));
  }
}

impl Clone for VirtioSoundDeviceInputStreamConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSoundDeviceInputStreamConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioSoundDeviceInputStreamConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_sound_device_input_stream_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl VirtioSoundDeviceStreamConfiguration for VirtioSoundDeviceInputStreamConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioSoundDeviceInputStreamConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
