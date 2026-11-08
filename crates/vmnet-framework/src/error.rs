//! What a call into vmnet failed with.

use std::fmt;

use crate::platform::ffi;
use crate::r#return::Return;

/// What a call into vmnet failed with: a [`vmnet_return_t`] other than
/// `VMNET_SUCCESS`, or vmnet's absence.
///
/// [`vmnet_return_t`]: https://developer.apple.com/documentation/vmnet/vmnet_return_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
  /// vmnet exists only on macOS.
  Unsupported,
  /// What vmnet returned for a call that failed.
  Return(Return),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "vmnet is only available on macOS"),
      Self::Return(value) => write!(formatter, "vmnet returned {value:?}"),
    }
  }
}

impl std::error::Error for Error {}

/// `Success` is `Ok`; anything else, an [`Error::Return`].
pub(crate) fn returned(value: ffi::VmnetReturn) -> Result<(), Error> {
  match Return::from(value) {
    Return::Success => Ok(()),
    value => Err(Error::Return(value)),
  }
}

/// A status vmnet set when it returned `NULL`.
impl From<ffi::VmnetReturn> for Error {
  fn from(value: ffi::VmnetReturn) -> Self {
    Self::Return(value.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success_is_ok() {
    assert_eq!(returned(ffi::VmnetReturn::Success), Ok(()));
  }

  #[test]
  fn anything_else_is_its_error() {
    assert_eq!(
      returned(ffi::VmnetReturn::NotAuthorized),
      Err(Error::Return(Return::NotAuthorized))
    );
    assert_eq!(
      returned(ffi::VmnetReturn::Unknown(2000)),
      Err(Error::Return(Return::Unknown(2000)))
    );
  }

  #[test]
  fn displays_what_vmnet_returned() {
    assert_eq!(
      Error::Return(Return::InvalidArgument).to_string(),
      "vmnet returned InvalidArgument"
    );
  }
}
