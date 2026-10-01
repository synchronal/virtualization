//! What a call into Virtualization failed with.

use std::collections::BTreeMap;
use std::fmt;

use url::Url;

use crate::platform::ffi;

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

/// [`NSError`](https://developer.apple.com/documentation/foundation/nserror),
/// as thrown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NSError {
  /// [`domain`](https://developer.apple.com/documentation/foundation/nserror/domain)
  pub domain: String,
  /// [`code`](https://developer.apple.com/documentation/foundation/nserror/code)
  pub code: isize,
  /// [`userInfo`](https://developer.apple.com/documentation/foundation/nserror/userinfo),
  /// each value as its `description`.
  pub user_info: BTreeMap<String, String>,
  /// [`localizedDescription`](https://developer.apple.com/documentation/foundation/nserror/localizeddescription)
  pub localized_description: String,
  /// [`localizedFailureReason`](https://developer.apple.com/documentation/foundation/nserror/localizedfailurereason)
  pub localized_failure_reason: Option<String>,
  /// [`localizedRecoverySuggestion`](https://developer.apple.com/documentation/foundation/nserror/localizedrecoverysuggestion)
  pub localized_recovery_suggestion: Option<String>,
  /// [`underlyingErrors`](https://developer.apple.com/documentation/foundation/nserror/underlyingerrors)
  pub underlying_errors: Vec<NSError>,
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

impl fmt::Display for NSError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "{} ({} {})",
      self.localized_description, self.domain, self.code
    )
  }
}

/// Its source is its first underlying error.
impl std::error::Error for NSError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self
      .underlying_errors
      .first()
      .map(|error| error as &(dyn std::error::Error + 'static))
  }
}

impl From<ffi::NSError> for NSError {
  fn from(error: ffi::NSError) -> Self {
    Self {
      domain: error.ns_domain(),
      code: error.ns_code(),
      user_info: (0..error.bridge_user_info_count())
        .map(|index| {
          (
            error.bridge_user_info_key_at(index),
            error.bridge_user_info_value_description_at(index),
          )
        })
        .collect(),
      localized_description: error.ns_localized_description(),
      localized_failure_reason: error.ns_localized_failure_reason(),
      localized_recovery_suggestion: error.ns_localized_recovery_suggestion(),
      underlying_errors: (0..error.bridge_underlying_errors_count())
        .map(|index| error.bridge_underlying_error_at(index).into())
        .collect(),
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
