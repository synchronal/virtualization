//! Security's `SecCertificate`, which Virtualization's Secure Boot members
//! take and return.

use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`SecCertificate`](https://developer.apple.com/documentation/security/seccertificate)
///
/// `Data` is its bytes. `Send` and `Sync`: a certificate never changes.
pub struct SecCertificate {
  pub(crate) inner: ffi::SecCertificate,
}

impl SecCertificate {
  /// [`SecCertificateCreateWithData(_:_:)`](https://developer.apple.com/documentation/security/seccertificatecreatewithdata(_:_:)),
  /// with the default allocator.
  pub fn create_with_data(data: &[u8]) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::sec_certificate_create_with_data(data).map(|inner| Self { inner }))
  }

  /// [`SecCertificateCopyData(_:)`](https://developer.apple.com/documentation/security/seccertificatecopydata(_:))
  pub fn copy_data(&self) -> Vec<u8> {
    self.inner.sec_certificate_copy_data()
  }
}

impl Clone for SecCertificate {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for SecCertificate {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(SecCertificate::create_with_data(&[]), Err(Error::Unsupported)));
  }
}
