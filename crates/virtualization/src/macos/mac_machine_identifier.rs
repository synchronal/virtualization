use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZMacMachineIdentifier`](https://developer.apple.com/documentation/virtualization/vzmacmachineidentifier)
///
/// `Data` is its bytes. Equal when Virtualization's `isEqual(_:)` says so.
pub struct MacMachineIdentifier {
  pub(crate) inner: ffi::VZMacMachineIdentifier,
}

impl MacMachineIdentifier {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmacmachineidentifier/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_machine_identifier_init(),
    })
  }

  /// [`init(dataRepresentation:)`](https://developer.apple.com/documentation/virtualization/vzmacmachineidentifier/init(datarepresentation:))
  pub fn with_data_representation(data_representation: &[u8]) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::vz_mac_machine_identifier_init_data_representation(data_representation).map(|inner| Self { inner }))
  }

  /// [`dataRepresentation`](https://developer.apple.com/documentation/virtualization/vzmacmachineidentifier/datarepresentation)
  pub fn data_representation(&self) -> Vec<u8> {
    self.inner.vz_data_representation()
  }
}

impl Clone for MacMachineIdentifier {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacMachineIdentifier {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`isEqual(_:)`](https://developer.apple.com/documentation/objectivec/nsobjectprotocol/isequal(_:))
impl PartialEq for MacMachineIdentifier {
  fn eq(&self, other: &Self) -> bool {
    self.inner.ns_is_equal(other.inner.bridge_retained())
  }
}

impl Eq for MacMachineIdentifier {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacMachineIdentifier::new(), Err(Error::Unsupported)));
    assert!(matches!(
      MacMachineIdentifier::with_data_representation(&[]),
      Err(Error::Unsupported)
    ));
  }
}
