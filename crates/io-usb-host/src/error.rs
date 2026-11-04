//! What a call into IOUSBHost failed with.

use std::fmt;

pub use apple_bridge_foundation::NSError;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// IOUSBHost exists only on macOS.
  Unsupported,
  /// What IOUSBHost failed with. Boxed: it is far larger than the other.
  NSError(Box<NSError>),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "IOUSBHost is only available on macOS"),
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

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;
  use std::error::Error as _;

  use super::*;

  fn ns_error(underlying_errors: Vec<NSError>) -> NSError {
    NSError {
      domain: "IOUSBHostErrorDomain".to_string(),
      code: -536870206,
      user_info: BTreeMap::new(),
      localized_description: "The device isn't open.".to_string(),
      localized_failure_reason: None,
      localized_recovery_suggestion: None,
      underlying_errors,
    }
  }

  #[test]
  fn says_iousbhost_is_macos_only() {
    assert_eq!(Error::Unsupported.to_string(), "IOUSBHost is only available on macOS");
  }

  #[test]
  fn says_what_iousbhost_failed_with() {
    assert_eq!(
      Error::NSError(Box::new(ns_error(vec![]))).to_string(),
      "The device isn't open. (IOUSBHostErrorDomain -536870206)"
    );
  }

  #[test]
  fn has_an_nserrors_first_underlying_error_as_its_source() {
    let underlying = ns_error(vec![]);
    let error = Error::NSError(Box::new(ns_error(vec![underlying.clone()])));

    assert_eq!(error.source().unwrap().to_string(), underlying.to_string());
    assert!(Error::Unsupported.source().is_none());
  }
}
