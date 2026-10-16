use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{AudioOutputStreamSink, Error, VirtioSoundDeviceStreamConfiguration};

/// [`VZVirtioSoundDeviceOutputStreamConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceoutputstreamconfiguration)
pub struct VirtioSoundDeviceOutputStreamConfiguration {
  inner: ffi::VZVirtioSoundDeviceOutputStreamConfiguration,
}

impl VirtioSoundDeviceOutputStreamConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceoutputstreamconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_sound_device_output_stream_configuration_init(),
    })
  }

  /// [`sink`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceoutputstreamconfiguration/sink),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn sink<T: AudioOutputStreamSink>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_sink()?)
  }

  /// [`sink`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddeviceoutputstreamconfiguration/sink)
  pub fn set_sink(&self, sink: Option<&dyn AudioOutputStreamSink>) {
    self
      .inner
      .vz_set_sink(sink.map(|sink| sink.bridge_object()));
  }
}

impl Clone for VirtioSoundDeviceOutputStreamConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSoundDeviceOutputStreamConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioSoundDeviceOutputStreamConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_sound_device_output_stream_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl VirtioSoundDeviceStreamConfiguration for VirtioSoundDeviceOutputStreamConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioSoundDeviceOutputStreamConfiguration::new(),
      Err(Error::Unsupported)
    ));
  }
}
