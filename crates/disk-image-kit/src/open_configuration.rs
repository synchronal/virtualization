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

/// [`OpenConfiguration`](https://developer.apple.com/documentation/diskimagekit/openconfiguration):
/// a struct of two values, in Rust as in Swift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenConfiguration {
  /// [`url`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/url)
  pub url: Url,
  /// [`mode`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/mode)
  pub mode: OpenConfigurationMode,
}

impl OpenConfiguration {
  /// [`init(url:mode:)`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/init(url:mode:))
  pub fn new(url: Url, mode: OpenConfigurationMode) -> Self {
    Self { url, mode }
  }

  /// [`open(url:mode:)`](https://developer.apple.com/documentation/diskimagekit/openconfiguration/open(url:mode:))
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
  fn mode_becomes_the_same_case_in_disk_image_kit_sys() {
    assert!(matches!(
      ffi::OpenConfigurationMode::from(OpenConfigurationMode::Automatic),
      ffi::OpenConfigurationMode::Automatic
    ));
    assert!(matches!(
      ffi::OpenConfigurationMode::from(OpenConfigurationMode::ReadOnly),
      ffi::OpenConfigurationMode::ReadOnly
    ));
    assert!(matches!(
      ffi::OpenConfigurationMode::from(OpenConfigurationMode::ReadWrite),
      ffi::OpenConfigurationMode::ReadWrite
    ));
    assert!(matches!(
      ffi::OpenConfigurationMode::from(OpenConfigurationMode::Unknown),
      ffi::OpenConfigurationMode::Unknown
    ));
  }

  #[test]
  fn mode_comes_from_the_same_case_in_disk_image_kit_sys() {
    assert_eq!(
      OpenConfigurationMode::from(ffi::OpenConfigurationMode::Automatic),
      OpenConfigurationMode::Automatic
    );
    assert_eq!(
      OpenConfigurationMode::from(ffi::OpenConfigurationMode::ReadOnly),
      OpenConfigurationMode::ReadOnly
    );
    assert_eq!(
      OpenConfigurationMode::from(ffi::OpenConfigurationMode::ReadWrite),
      OpenConfigurationMode::ReadWrite
    );
    assert_eq!(
      OpenConfigurationMode::from(ffi::OpenConfigurationMode::Unknown),
      OpenConfigurationMode::Unknown
    );
  }
}
