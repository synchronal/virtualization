use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZMacHardwareModel`](https://developer.apple.com/documentation/virtualization/vzmachardwaremodel)
///
/// `Data` is its bytes.
pub struct MacHardwareModel {
  pub(crate) inner: ffi::VZMacHardwareModel,
}

impl MacHardwareModel {
  /// [`init(dataRepresentation:)`](https://developer.apple.com/documentation/virtualization/vzmachardwaremodel/init(datarepresentation:))
  pub fn new(data_representation: &[u8]) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::vz_mac_hardware_model_init_data_representation(data_representation).map(|inner| Self { inner }))
  }

  /// [`dataRepresentation`](https://developer.apple.com/documentation/virtualization/vzmachardwaremodel/datarepresentation)
  pub fn data_representation(&self) -> Vec<u8> {
    self.inner.vz_data_representation()
  }

  /// [`isSupported`](https://developer.apple.com/documentation/virtualization/vzmachardwaremodel/issupported)
  pub fn is_supported(&self) -> bool {
    self.inner.vz_is_supported()
  }
}

impl Clone for MacHardwareModel {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacHardwareModel {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacHardwareModel::new(&[]), Err(Error::Unsupported)));
  }
}
