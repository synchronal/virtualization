//! What a call into Virtualization failed with.

use std::fmt;

pub use foundation_sys::NSError;
use url::Url;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// Virtualization exists only on macOS.
  Unsupported,
  /// What Virtualization threw. Boxed: it is far larger than the others.
  NSError(Box<NSError>),
  /// A URL from Foundation that [`url::Url`] can't parse.
  UnparseableUrl(url::ParseError),
  /// A URL Foundation's `NSURL(string:)` rejects.
  UnrepresentableUrl(Url),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "Virtualization is only available on macOS"),
      Self::NSError(error) => error.fmt(formatter),
      Self::UnparseableUrl(error) => write!(formatter, "Foundation returned an unparseable URL: {error}"),
      Self::UnrepresentableUrl(url) => write!(formatter, "Foundation can't represent the URL {url}"),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::NSError(error) => error.source(),
      Self::UnparseableUrl(error) => Some(error),
      Self::Unsupported | Self::UnrepresentableUrl(_) => None,
    }
  }
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Self {
    Self::UnparseableUrl(error)
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;
  use std::error::Error as _;

  use super::*;

  fn ns_error(underlying_errors: Vec<NSError>) -> NSError {
    NSError {
      domain: "VZErrorDomain".to_string(),
      code: 2,
      user_info: BTreeMap::new(),
      localized_description: "The storage device attachment is invalid.".to_string(),
      localized_failure_reason: None,
      localized_recovery_suggestion: None,
      underlying_errors,
    }
  }

  #[test]
  fn says_virtualization_is_macos_only() {
    assert_eq!(
      Error::Unsupported.to_string(),
      "Virtualization is only available on macOS"
    );
  }

  #[test]
  fn says_what_virtualization_threw() {
    assert_eq!(
      Error::NSError(Box::new(ns_error(vec![]))).to_string(),
      "The storage device attachment is invalid. (VZErrorDomain 2)"
    );
  }

  #[test]
  fn says_why_a_url_did_not_parse() {
    let error = Error::from(url::ParseError::RelativeUrlWithoutBase);

    assert!(matches!(
      error,
      Error::UnparseableUrl(url::ParseError::RelativeUrlWithoutBase)
    ));
    assert_eq!(
      error.to_string(),
      "Foundation returned an unparseable URL: relative URL without a base"
    );
  }

  #[test]
  fn names_a_url_foundation_rejects() {
    let url = Url::parse("file:///nowhere").unwrap();

    assert_eq!(
      Error::UnrepresentableUrl(url).to_string(),
      "Foundation can't represent the URL file:///nowhere"
    );
  }

  #[test]
  fn has_an_nserrors_first_underlying_error_as_its_source() {
    let underlying = ns_error(vec![]);
    let error = Error::NSError(Box::new(ns_error(vec![underlying.clone()])));

    assert_eq!(error.source().unwrap().to_string(), underlying.to_string());
    assert!(
      Error::NSError(Box::new(ns_error(vec![])))
        .source()
        .is_none()
    );
  }

  #[test]
  fn has_a_parse_error_as_its_source() {
    assert!(
      Error::UnparseableUrl(url::ParseError::EmptyHost)
        .source()
        .is_some()
    );
  }

  #[test]
  fn has_no_source_when_nothing_underlies_it() {
    assert!(Error::Unsupported.source().is_none());
    assert!(
      Error::UnrepresentableUrl(Url::parse("file:///nowhere").unwrap())
        .source()
        .is_none()
    );
  }
}
