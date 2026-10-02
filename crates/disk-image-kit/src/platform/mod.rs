//! disk-image-kit-sys on macOS, stand-ins elsewhere, so every type is
//! written once.

use crate::Error;

#[cfg(not(target_os = "macos"))]
pub(crate) mod unsupported;

#[cfg(target_os = "macos")]
pub(crate) use disk_image_kit_sys as ffi;

#[cfg(not(target_os = "macos"))]
pub(crate) use self::unsupported as ffi;

/// Whether DiskImageKit exists here. Every entry point asks first.
#[cfg(target_os = "macos")]
pub(crate) fn supported() -> Result<(), Error> {
  Ok(())
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn supported() -> Result<(), Error> {
  Err(Error::Unsupported)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(target_os = "macos")]
  #[test]
  fn is_supported_on_macos() {
    assert!(supported().is_ok());
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_elsewhere() {
    assert!(matches!(supported(), Err(Error::Unsupported)));
  }
}
