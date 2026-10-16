use url::Url;

use crate::platform::ffi;

/// [`OpenConfigurationProtocol`](https://developer.apple.com/documentation/diskimagekit/openconfigurationprotocol):
/// what `DiskImage::with_opening` takes. Its requirements are all DiskImageKit
/// reads, so any type can conform.
pub trait OpenConfigurationProtocol {
  /// [`url`](https://developer.apple.com/documentation/diskimagekit/openconfigurationprotocol/url)
  fn url(&self) -> &Url;

  /// [`mode`](https://developer.apple.com/documentation/diskimagekit/openconfigurationprotocol/mode)
  fn mode(&self) -> OpenConfigurationMode;
}

/// [`OpenConfiguration`](https://developer.apple.com/documentation/diskimagekit/openconfiguration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConfiguration {
  /// [`url`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/url)
  pub url: Url,
  /// [`mode`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode-swift.property)
  pub mode: OpenConfigurationMode,
}

impl OpenConfiguration {
  /// [`init(url:mode:)`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/init(url:mode:))
  pub fn new(url: Url, mode: OpenConfigurationMode) -> Self {
    Self { url, mode }
  }

  /// [`open(url:mode:)`](https://developer.apple.com/documentation/diskimagekit/openconfigurationprotocol/open(url:mode:)),
  /// declared on `OpenConfigurationProtocol` where `Self` is
  /// `OpenConfiguration`.
  pub fn open(url: Url, mode: OpenConfigurationMode) -> Self {
    Self { url, mode }
  }
}

impl OpenConfigurationProtocol for OpenConfiguration {
  fn url(&self) -> &Url {
    &self.url
  }

  fn mode(&self) -> OpenConfigurationMode {
    self.mode
  }
}

/// [`OpenConfiguration.Mode`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode-swift.enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OpenConfigurationMode {
  /// [`automatic`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode-swift.enum/automatic)
  Automatic,
  /// [`readOnly`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode-swift.enum/readonly)
  ReadOnly,
  /// [`readWrite`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode-swift.enum/readwrite)
  ReadWrite,
  /// A case newer than this crate. Swift's `Mode` has no raw values, so
  /// passing this to DiskImageKit stops the process.
  Unknown,
}

impl From<OpenConfigurationMode> for ffi::OpenConfigurationMode {
  fn from(mode: OpenConfigurationMode) -> Self {
    match mode {
      OpenConfigurationMode::Automatic => Self::Automatic,
      OpenConfigurationMode::ReadOnly => Self::ReadOnly,
      OpenConfigurationMode::ReadWrite => Self::ReadWrite,
      OpenConfigurationMode::Unknown => Self::Unknown,
    }
  }
}

impl From<ffi::OpenConfigurationMode> for OpenConfigurationMode {
  fn from(mode: ffi::OpenConfigurationMode) -> Self {
    match mode {
      ffi::OpenConfigurationMode::Automatic => Self::Automatic,
      ffi::OpenConfigurationMode::ReadOnly => Self::ReadOnly,
      ffi::OpenConfigurationMode::ReadWrite => Self::ReadWrite,
      ffi::OpenConfigurationMode::Unknown => Self::Unknown,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_modes_both_ways() {
    for mode in [
      OpenConfigurationMode::Automatic,
      OpenConfigurationMode::ReadOnly,
      OpenConfigurationMode::ReadWrite,
      OpenConfigurationMode::Unknown,
    ] {
      assert_eq!(
        OpenConfigurationMode::from(ffi::OpenConfigurationMode::from(mode)),
        mode
      );
    }
  }
}
