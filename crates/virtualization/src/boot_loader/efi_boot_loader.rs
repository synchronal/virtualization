use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{BootLoader, EFIVariableStore, Error};

/// [`VZEFIBootLoader`](https://developer.apple.com/documentation/virtualization/vzefibootloader)
pub struct EFIBootLoader {
  inner: ffi::VZEFIBootLoader,
}

impl EFIBootLoader {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzefibootloader/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_efi_boot_loader_init(),
    })
  }

  /// [`variableStore`](https://developer.apple.com/documentation/virtualization/vzefibootloader/variablestore)
  pub fn variable_store(&self) -> Option<EFIVariableStore> {
    self
      .inner
      .vz_variable_store()
      .ok()
      .map(|inner| EFIVariableStore { inner })
  }

  /// [`variableStore`](https://developer.apple.com/documentation/virtualization/vzefibootloader/variablestore)
  pub fn set_variable_store(&self, variable_store: Option<&EFIVariableStore>) {
    match variable_store {
      Some(variable_store) => self
        .inner
        .vz_set_variable_store(variable_store.inner.bridge_retained()),
      None => self.inner.bridge_set_variable_store_nil(),
    }
  }
}

impl Clone for EFIBootLoader {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for EFIBootLoader {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for EFIBootLoader {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_efi_boot_loader_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl BootLoader for EFIBootLoader {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(EFIBootLoader::new(), Err(Error::Unsupported)));
  }
}
