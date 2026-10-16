use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{AudioInputStreamSource, Error};

/// [`VZHostAudioInputStreamSource`](https://developer.apple.com/documentation/virtualization/vzhostaudioinputstreamsource)
pub struct HostAudioInputStreamSource {
  inner: ffi::VZHostAudioInputStreamSource,
}

impl HostAudioInputStreamSource {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzhostaudioinputstreamsource/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_host_audio_input_stream_source_init(),
    })
  }
}

impl Clone for HostAudioInputStreamSource {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for HostAudioInputStreamSource {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for HostAudioInputStreamSource {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_host_audio_input_stream_source_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl AudioInputStreamSource for HostAudioInputStreamSource {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(HostAudioInputStreamSource::new(), Err(Error::Unsupported)));
  }
}
