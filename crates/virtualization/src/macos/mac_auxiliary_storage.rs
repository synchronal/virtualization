use std::fmt;

use url::Url;

use crate::platform::{self, ffi};
use crate::{Error, MacHardwareModel, foundation};

/// [`VZMacAuxiliaryStorage`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage)
pub struct MacAuxiliaryStorage {
  pub(crate) inner: ffi::VZMacAuxiliaryStorage,
}

impl MacAuxiliaryStorage {
  /// [`init(url:)`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/init(url:))
  pub fn new(url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_auxiliary_storage_init(&*foundation::ns_url(url)?),
    })
  }

  /// [`init(creatingStorageAt:hardwareModel:options:)`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/init(creatingstorageat:hardwaremodel:options:))
  pub fn with_creating_storage_at_and_hardware_model_and_options(
    url: &Url,
    hardware_model: &MacHardwareModel,
    options: MacAuxiliaryStorageInitializationOptions,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_auxiliary_storage_init_creating_storage_at_hardware_model_options(
        &*foundation::ns_url(url)?,
        &hardware_model.inner,
        options.raw_value,
      )
      .map_err(foundation::thrown)?,
    })
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }
}

impl Clone for MacAuxiliaryStorage {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacAuxiliaryStorage {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZMacAuxiliaryStorage.InitializationOptions`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/initializationoptions):
/// an option set, which is its raw value. Swift's `[]` is `new(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAuxiliaryStorageInitializationOptions {
  /// [`rawValue`](https://developer.apple.com/documentation/swift/rawrepresentable/rawvalue-swift.property)
  pub raw_value: usize,
}

impl MacAuxiliaryStorageInitializationOptions {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/initializationoptions/init(rawvalue:))
  pub fn new(raw_value: usize) -> Self {
    Self { raw_value }
  }

  /// [`allowOverwrite`](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage/initializationoptions/allowoverwrite)
  pub fn allow_overwrite() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      raw_value: ffi::vz_mac_auxiliary_storage_initialization_options_allow_overwrite(),
    })
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/auxiliary-storage").unwrap();

    assert!(matches!(
      MacAuxiliaryStorageInitializationOptions::allow_overwrite(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(MacAuxiliaryStorage::new(&url), Err(Error::Unsupported)));
  }
}
