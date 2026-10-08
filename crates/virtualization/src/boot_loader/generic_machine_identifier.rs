use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZGenericMachineIdentifier`](https://developer.apple.com/documentation/virtualization/vzgenericmachineidentifier)
///
/// `Data` is its bytes. Equal when Virtualization's `isEqual(_:)` says so: when
/// their contents match.
pub struct GenericMachineIdentifier {
  pub(crate) inner: ffi::VZGenericMachineIdentifier,
}

impl GenericMachineIdentifier {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzgenericmachineidentifier/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::from_inner(ffi::vz_generic_machine_identifier_init()))
  }

  /// [`init(dataRepresentation:)`](https://developer.apple.com/documentation/virtualization/vzgenericmachineidentifier/init(datarepresentation:))
  pub fn with_data_representation(data_representation: &[u8]) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::vz_generic_machine_identifier_init_data_representation(data_representation).map(Self::from_inner))
  }

  pub(crate) fn from_inner(inner: ffi::VZGenericMachineIdentifier) -> Self {
    Self { inner }
  }

  /// [`dataRepresentation`](https://developer.apple.com/documentation/virtualization/vzgenericmachineidentifier/datarepresentation)
  pub fn data_representation(&self) -> Vec<u8> {
    self.inner.vz_data_representation()
  }
}

impl Clone for GenericMachineIdentifier {
  fn clone(&self) -> Self {
    Self::from_inner(self.inner.bridge_retained())
  }
}

/// The object's `description`.
impl fmt::Debug for GenericMachineIdentifier {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`isEqual(_:)`](https://developer.apple.com/documentation/objectivec/nsobjectprotocol/isequal(_:))
impl PartialEq for GenericMachineIdentifier {
  fn eq(&self, other: &Self) -> bool {
    self.inner.ns_is_equal(other.inner.bridge_retained())
  }
}

impl Eq for GenericMachineIdentifier {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(GenericMachineIdentifier::new(), Err(Error::Unsupported)));
    assert!(matches!(
      GenericMachineIdentifier::with_data_representation(&[]),
      Err(Error::Unsupported)
    ));
  }
}
