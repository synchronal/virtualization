//! A thrown error, as plain Rust.

use std::collections::BTreeMap;
use std::fmt;

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

#[cfg(target_os = "macos")]
impl From<&crate::objc::NSError> for NSError {
  fn from(error: &crate::objc::NSError) -> Self {
    let (keys, values) = error.userInfo().to_vecs();

    Self {
      domain: error.domain().to_string(),
      code: error.code(),
      user_info: keys
        .iter()
        .zip(&values)
        .map(|(key, value)| (key.to_string(), crate::objc::ns_description(value)))
        .collect(),
      localized_description: error.localizedDescription().to_string(),
      localized_failure_reason: error
        .localizedFailureReason()
        .map(|reason| reason.to_string()),
      localized_recovery_suggestion: error
        .localizedRecoverySuggestion()
        .map(|suggestion| suggestion.to_string()),
      underlying_errors: error
        .underlyingErrors()
        .iter()
        .map(|underlying| (&*underlying).into())
        .collect(),
    }
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
  fn says_its_description_domain_and_code() {
    assert_eq!(
      ns_error(vec![]).to_string(),
      "The storage device attachment is invalid. (VZErrorDomain 2)"
    );
  }

  #[test]
  fn has_its_first_underlying_error_as_its_source() {
    let underlying = ns_error(vec![]);
    let error = ns_error(vec![underlying.clone()]);

    assert_eq!(error.source().unwrap().to_string(), underlying.to_string());
    assert!(ns_error(vec![]).source().is_none());
  }
}
