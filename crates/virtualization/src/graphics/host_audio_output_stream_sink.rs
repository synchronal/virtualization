use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{AudioOutputStreamSink, Error};

/// [`VZHostAudioOutputStreamSink`](https://developer.apple.com/documentation/virtualization/vzhostaudiooutputstreamsink)
pub struct HostAudioOutputStreamSink {
  inner: ffi::VZHostAudioOutputStreamSink,
}

impl HostAudioOutputStreamSink {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzhostaudiooutputstreamsink/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_host_audio_output_stream_sink_init(),
    })
  }
}

impl Clone for HostAudioOutputStreamSink {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for HostAudioOutputStreamSink {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for HostAudioOutputStreamSink {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_host_audio_output_stream_sink_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl AudioOutputStreamSink for HostAudioOutputStreamSink {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(HostAudioOutputStreamSink::new(), Err(Error::Unsupported)));
  }
}
