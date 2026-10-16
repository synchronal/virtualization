use std::fmt;

use crate::platform::{self, ffi};
use crate::{EFISignatureList, Error};

/// [`VZEFISignatureDatabaseConfiguration`](https://developer.apple.com/documentation/virtualization/vzefisignaturedatabaseconfiguration)
///
/// `Send` and `Sync`, as Apple declares it `Sendable`.
pub struct EFISignatureDatabaseConfiguration {
  pub(crate) inner: ffi::VZEFISignatureDatabaseConfiguration,
}

impl EFISignatureDatabaseConfiguration {
  /// [`init(keyExchangeKeys:dbSignatures:dbxSignatures:)`](https://developer.apple.com/documentation/virtualization/vzefisignaturedatabaseconfiguration/init(keyexchangekeys:dbsignatures:dbxsignatures:))
  pub fn new(
    key_exchange_keys: &[EFISignatureList],
    db_signatures: &[EFISignatureList],
    dbx_signatures: &[EFISignatureList],
  ) -> Result<Self, Error> {
    platform::supported()?;

    let [key_exchange_keys, db_signatures, dbx_signatures] =
      [key_exchange_keys, db_signatures, dbx_signatures].map(|signature_lists| {
        signature_lists
          .iter()
          .map(|signature_list| &signature_list.inner)
          .collect::<Vec<_>>()
      });

    Ok(Self {
      inner: ffi::vz_efi_signature_database_configuration_init(&key_exchange_keys, &db_signatures, &dbx_signatures),
    })
  }

  /// [`keyExchangeKeys`](https://developer.apple.com/documentation/virtualization/vzefisignaturedatabaseconfiguration/keyexchangekeys)
  pub fn key_exchange_keys(&self) -> Vec<EFISignatureList> {
    self
      .inner
      .vz_key_exchange_keys()
      .into_iter()
      .map(EFISignatureList::bridge_from)
      .collect()
  }

  /// [`dbSignatures`](https://developer.apple.com/documentation/virtualization/vzefisignaturedatabaseconfiguration/dbsignatures)
  pub fn db_signatures(&self) -> Vec<EFISignatureList> {
    self
      .inner
      .vz_db_signatures()
      .into_iter()
      .map(EFISignatureList::bridge_from)
      .collect()
  }

  /// [`dbxSignatures`](https://developer.apple.com/documentation/virtualization/vzefisignaturedatabaseconfiguration/dbxsignatures)
  pub fn dbx_signatures(&self) -> Vec<EFISignatureList> {
    self
      .inner
      .vz_dbx_signatures()
      .into_iter()
      .map(EFISignatureList::bridge_from)
      .collect()
  }
}

impl Clone for EFISignatureDatabaseConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for EFISignatureDatabaseConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      EFISignatureDatabaseConfiguration::new(&[], &[], &[]),
      Err(Error::Unsupported)
    ));
  }
}
