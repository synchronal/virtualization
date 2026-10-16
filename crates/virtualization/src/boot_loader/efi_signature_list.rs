use std::fmt;

use url::Url;

use crate::foundation;
use crate::platform::{self, ffi};
use crate::{Error, SecCertificate};

/// [`VZEFISignatureList`](https://developer.apple.com/documentation/virtualization/vzefisignaturelist)
///
/// `Send` and `Sync`, as Apple declares it `Sendable`.
pub struct EFISignatureList {
  pub(crate) inner: ffi::VZEFISignatureList,
}

impl EFISignatureList {
  pub(crate) fn bridge_from(inner: ffi::VZEFISignatureList) -> Self {
    Self { inner }
  }

  /// [`init(contentsOf:)`](https://developer.apple.com/documentation/virtualization/vzefisignaturelist/init(contentsof:))
  pub fn new(url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_efi_signature_list_init_contents_of(&*foundation::ns_url(url)?).map_err(foundation::thrown)?,
    })
  }

  /// [`init(signatures:)`](https://developer.apple.com/documentation/virtualization/vzefisignaturelist/init(signatures:))
  pub fn with_signatures(signatures: &[EFISignature]) -> Result<Self, Error> {
    platform::supported()?;

    let signatures: Vec<_> = signatures.iter().map(Into::into).collect();

    Ok(Self {
      inner: ffi::vz_efi_signature_list_init_signatures(&signatures),
    })
  }

  /// [`signatures`](https://developer.apple.com/documentation/virtualization/vzefisignaturelist/signatures-3cz50)
  pub fn signatures(&self) -> Vec<EFISignature> {
    self
      .inner
      .vz_signatures()
      .into_iter()
      .map(Into::into)
      .collect()
  }
}

impl Clone for EFISignatureList {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for EFISignatureList {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZEFISignature`](https://developer.apple.com/documentation/virtualization/vzefisignature-swift.enum)
///
/// `Data` is its bytes. `Unknown` is a case newer than this crate; giving it to
/// Virtualization stops the process.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EFISignature {
  /// [`sha256(_:)`](https://developer.apple.com/documentation/virtualization/vzefisignature-swift.enum/sha256(_:))
  Sha256(Vec<u8>),
  /// [`x509Certificate(_:)`](https://developer.apple.com/documentation/virtualization/vzefisignature-swift.enum/x509certificate(_:))
  X509Certificate(SecCertificate),
  Unknown,
}

impl From<&EFISignature> for ffi::VZEFISignature {
  fn from(signature: &EFISignature) -> Self {
    match signature {
      EFISignature::Sha256(data) => Self::Sha256(data.clone()),
      EFISignature::X509Certificate(certificate) => Self::X509Certificate(certificate.inner.bridge_retained()),
      EFISignature::Unknown => Self::Unknown,
    }
  }
}

impl From<ffi::VZEFISignature> for EFISignature {
  fn from(signature: ffi::VZEFISignature) -> Self {
    match signature {
      ffi::VZEFISignature::Sha256(data) => Self::Sha256(data),
      ffi::VZEFISignature::X509Certificate(inner) => Self::X509Certificate(SecCertificate { inner }),
      ffi::VZEFISignature::Unknown => Self::Unknown,
    }
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/signatures.esl").unwrap();

    assert!(matches!(EFISignatureList::new(&url), Err(Error::Unsupported)));
    assert!(matches!(
      EFISignatureList::with_signatures(&[]),
      Err(Error::Unsupported)
    ));
  }
}
