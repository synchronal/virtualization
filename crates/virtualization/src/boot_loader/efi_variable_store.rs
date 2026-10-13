use std::fmt;

use url::Url;

use crate::foundation;
use crate::platform::{self, ffi};
use crate::{EFISignatureDatabaseConfiguration, Error, SecCertificate};

/// [`VZEFIVariableStore`](https://developer.apple.com/documentation/virtualization/vzefivariablestore)
pub struct EFIVariableStore {
  pub(crate) inner: ffi::VZEFIVariableStore,
}

impl EFIVariableStore {
  /// [`init(url:)`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/init(url:))
  pub fn new(url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_efi_variable_store_init(&*foundation::ns_url(url)?),
    })
  }

  /// [`init(creatingVariableStoreAt:options:)`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/init(creatingvariablestoreat:options:))
  pub fn with_creating_variable_store_at_and_options(
    url: &Url,
    options: EFIVariableStoreInitializationOptions,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_efi_variable_store_init_creating_variable_store_at_options(
        &*foundation::ns_url(url)?,
        options.raw_value,
      )
      .map_err(foundation::thrown)?,
    })
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`enableSecureBootUsingDefaultPlatformKey()`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/enablesecurebootusingdefaultplatformkey())
  pub fn enable_secure_boot_using_default_platform_key(&self) -> Result<(), Error> {
    self
      .inner
      .vz_enable_secure_boot_using_default_platform_key()
      .map_err(foundation::thrown)
  }

  /// [`enableSecureBoot(platformKey:)`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/enablesecureboot(platformkey:))
  pub fn enable_secure_boot(&self, platform_key: &SecCertificate) -> Result<(), Error> {
    self
      .inner
      .vz_enable_secure_boot(&platform_key.inner)
      .map_err(foundation::thrown)
  }

  /// [`disableSecureBoot()`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/disablesecureboot())
  pub fn disable_secure_boot(&self) -> Result<(), Error> {
    self
      .inner
      .vz_disable_secure_boot()
      .map_err(foundation::thrown)
  }

  /// [`resetSecureBoot()`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/resetsecureboot())
  pub fn reset_secure_boot(&self) -> Result<(), Error> {
    self
      .inner
      .vz_reset_secure_boot()
      .map_err(foundation::thrown)
  }

  /// [`isSecureBootEnabled`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/issecurebootenabled)
  pub fn is_secure_boot_enabled(&self) -> Result<bool, Error> {
    self
      .inner
      .vz_is_secure_boot_enabled()
      .map_err(foundation::thrown)
  }

  /// [`enrollDefaultSecureBootSignatures()`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/enrolldefaultsecurebootsignatures())
  pub fn enroll_default_secure_boot_signatures(&self) -> Result<(), Error> {
    self
      .inner
      .vz_enroll_default_secure_boot_signatures()
      .map_err(foundation::thrown)
  }

  /// [`enrollSecureBootSignatures(_:)`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/enrollsecurebootsignatures(_:))
  pub fn enroll_secure_boot_signatures(&self, signatures: &EFISignatureDatabaseConfiguration) -> Result<(), Error> {
    self
      .inner
      .vz_enroll_secure_boot_signatures(signatures.inner.bridge_retained())
      .map_err(foundation::thrown)
  }

  /// [`enrolledSecureBootSignatures`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/enrolledsecurebootsignatures)
  pub fn enrolled_secure_boot_signatures(&self) -> Result<EFISignatureDatabaseConfiguration, Error> {
    self
      .inner
      .vz_enrolled_secure_boot_signatures()
      .map(|inner| EFISignatureDatabaseConfiguration { inner })
      .map_err(foundation::thrown)
  }
}

impl Clone for EFIVariableStore {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for EFIVariableStore {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZEFIVariableStore.InitializationOptions`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/initializationoptions):
/// an option set, which is its raw value. Swift's `[]` is `new(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EFIVariableStoreInitializationOptions {
  /// [`rawValue`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/initializationoptions/rawvalue)
  pub raw_value: usize,
}

impl EFIVariableStoreInitializationOptions {
  /// [`allowOverwrite`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/initializationoptions/allowoverwrite)
  pub const ALLOW_OVERWRITE: Self = Self { raw_value: 1 << 0 };

  /// [`init(rawValue:)`](https://developer.apple.com/documentation/virtualization/vzefivariablestore/initializationoptions/init(rawvalue:))
  pub fn new(raw_value: usize) -> Self {
    Self { raw_value }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn allow_overwrite_is_apples_raw_value() {
    assert_eq!(EFIVariableStoreInitializationOptions::ALLOW_OVERWRITE.raw_value, 1);
  }

  #[test]
  fn has_the_raw_value_it_was_made_with() {
    assert_eq!(
      EFIVariableStoreInitializationOptions::new(1),
      EFIVariableStoreInitializationOptions::ALLOW_OVERWRITE
    );
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/efi-variable-store").unwrap();

    assert!(matches!(EFIVariableStore::new(&url), Err(Error::Unsupported)));
    assert!(matches!(
      EFIVariableStore::with_creating_variable_store_at_and_options(
        &url,
        EFIVariableStoreInitializationOptions::new(0)
      ),
      Err(Error::Unsupported)
    ));
  }
}
