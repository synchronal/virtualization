use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{BootLoader, Error};

/// [`VZMacOSBootLoader`](https://developer.apple.com/documentation/virtualization/vzmacosbootloader)
pub struct MacOSBootLoader {
  inner: ffi::VZMacOSBootLoader,
}

impl MacOSBootLoader {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmacosbootloader/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_macos_boot_loader_init(),
    })
  }
}

impl Clone for MacOSBootLoader {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacOSBootLoader {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacOSBootLoader {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_macos_boot_loader_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl BootLoader for MacOSBootLoader {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacOSBootLoader::new(), Err(Error::Unsupported)));
  }
}
