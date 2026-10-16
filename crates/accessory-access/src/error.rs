//! What a call into AccessoryAccess failed with, and `AAError`'s codes.

use std::fmt;

pub use foundation_sys::NSError;

use crate::platform::{self, ffi};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// AccessoryAccess exists only on macOS.
  Unsupported,
  /// What AccessoryAccess failed with: in [`error_domain`], with an
  /// [`ErrorCode`], or another domain's. Boxed: it is far larger than the
  /// other.
  NSError(Box<NSError>),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "AccessoryAccess is only available on macOS"),
      Self::NSError(error) => error.fmt(formatter),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::NSError(error) => error.source(),
      Self::Unsupported => None,
    }
  }
}

/// [`AAErrorDomain`](https://developer.apple.com/documentation/accessoryaccess/aaerrordomain)
pub fn error_domain() -> Result<String, Error> {
  platform::supported()?;

  Ok(ffi::aa_error_domain())
}

/// [`AAError.Code`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code):
/// an `NSError`'s `code` in [`error_domain`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
  /// [`internalError`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/internalerror)
  InternalError,
  /// [`accessoryListenerAlreadyRegistered`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/accessorylisteneralreadyregistered)
  AccessoryListenerAlreadyRegistered,
  /// [`accessoryNotAccessible`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/accessorynotaccessible)
  AccessoryNotAccessible,
  /// [`invalidAccessoryState`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/invalidaccessorystate)
  InvalidAccessoryState,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl ErrorCode {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/init(rawvalue:)):
  /// `Unknown` for a raw value with no case.
  pub fn new(raw_value: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(ffi::aa_error_code_init_raw_value(raw_value).into())
  }

  /// [`rawValue`](https://developer.apple.com/documentation/accessoryaccess/aaerror/code/rawvalue)
  pub fn raw_value(self) -> Result<isize, Error> {
    platform::supported()?;

    Ok(ffi::aa_error_code_raw_value(self.into()))
  }
}

impl From<ffi::AAErrorCode> for ErrorCode {
  fn from(code: ffi::AAErrorCode) -> Self {
    match code {
      ffi::AAErrorCode::InternalError => Self::InternalError,
      ffi::AAErrorCode::AccessoryListenerAlreadyRegistered => Self::AccessoryListenerAlreadyRegistered,
      ffi::AAErrorCode::AccessoryNotAccessible => Self::AccessoryNotAccessible,
      ffi::AAErrorCode::InvalidAccessoryState => Self::InvalidAccessoryState,
      ffi::AAErrorCode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ErrorCode> for ffi::AAErrorCode {
  fn from(code: ErrorCode) -> Self {
    match code {
      ErrorCode::InternalError => Self::InternalError,
      ErrorCode::AccessoryListenerAlreadyRegistered => Self::AccessoryListenerAlreadyRegistered,
      ErrorCode::AccessoryNotAccessible => Self::AccessoryNotAccessible,
      ErrorCode::InvalidAccessoryState => Self::InvalidAccessoryState,
      ErrorCode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;
  use std::error::Error as _;

  use super::*;

  fn ns_error(underlying_errors: Vec<NSError>) -> NSError {
    NSError {
      domain: "AAErrorDomain".to_string(),
      code: 1,
      user_info: BTreeMap::new(),
      localized_description: "Unable to communicate with service".to_string(),
      localized_failure_reason: None,
      localized_recovery_suggestion: None,
      underlying_errors,
    }
  }

  #[test]
  fn says_accessory_access_is_macos_only() {
    assert_eq!(
      Error::Unsupported.to_string(),
      "AccessoryAccess is only available on macOS"
    );
  }

  #[test]
  fn says_what_accessory_access_failed_with() {
    assert_eq!(
      Error::NSError(Box::new(ns_error(vec![]))).to_string(),
      "Unable to communicate with service (AAErrorDomain 1)"
    );
  }

  #[test]
  fn has_an_nserrors_first_underlying_error_as_its_source() {
    let underlying = ns_error(vec![]);
    let error = Error::NSError(Box::new(ns_error(vec![underlying.clone()])));

    assert_eq!(error.source().unwrap().to_string(), underlying.to_string());
    assert!(Error::Unsupported.source().is_none());
  }

  #[test]
  fn maps_error_codes_both_ways() {
    for code in [
      ErrorCode::InternalError,
      ErrorCode::AccessoryListenerAlreadyRegistered,
      ErrorCode::AccessoryNotAccessible,
      ErrorCode::InvalidAccessoryState,
      ErrorCode::Unknown(99),
    ] {
      assert_eq!(ErrorCode::from(ffi::AAErrorCode::from(code)), code);
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(error_domain(), Err(Error::Unsupported)));
    assert!(matches!(ErrorCode::new(1), Err(Error::Unsupported)));
    assert!(matches!(ErrorCode::InternalError.raw_value(), Err(Error::Unsupported)));
  }
}
