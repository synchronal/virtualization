//! What a call into DiskImageKit failed with.

use std::fmt;

pub use apple_bridge_foundation::NSError;
use url::Url;

/// What a call into DiskImageKit failed with: what it threw, or why Rust
/// couldn't make the call or read its result.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// DiskImageKit exists only on macOS.
  Unsupported,
  /// DiskImageKit threw an [`IncompatibleStackingError`].
  IncompatibleStacking(IncompatibleStackingError),
  /// DiskImageKit threw an [`InvalidBlockCountError`].
  InvalidBlockCount(InvalidBlockCountError),
  /// DiskImageKit threw a [`CorruptedImageError`]. Boxed: it is far larger
  /// than the others.
  CorruptedImage(Box<CorruptedImageError>),
  /// DiskImageKit threw an [`UnsupportedFormatError`]. Boxed: it is far
  /// larger than the others.
  UnsupportedFormat(Box<UnsupportedFormatError>),
  /// Anything else DiskImageKit threw, e.g. a `POSIXError`. Boxed: it is far
  /// larger than the others.
  NSError(Box<NSError>),
  /// A URL from Foundation that [`url::Url`] can't parse.
  UnparseableUrl(url::ParseError),
  /// A URL Foundation's `NSURL(string:)` rejects.
  UnrepresentableUrl(Url),
}

/// [`IncompatibleStackingError`](https://developer.apple.com/documentation/diskimagekit/incompatiblestackingerror)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncompatibleStackingError {
  /// [`reason`](https://developer.apple.com/documentation/diskimagekit/incompatiblestackingerror/reason)
  pub reason: String,
  /// [`errorDescription`](https://developer.apple.com/documentation/foundation/localizederror/errordescription)
  pub error_description: Option<String>,
}

/// [`InvalidBlockCountError`](https://developer.apple.com/documentation/diskimagekit/invalidblockcounterror)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidBlockCountError {
  /// [`errorDescription`](https://developer.apple.com/documentation/foundation/localizederror/errordescription)
  pub error_description: Option<String>,
}

/// [`CorruptedImageError`](https://developer.apple.com/documentation/diskimagekit/corruptedimageerror)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorruptedImageError {
  /// [`underlyingError`](https://developer.apple.com/documentation/diskimagekit/corruptedimageerror/underlyingerror),
  /// as an `NSError`.
  pub underlying_error: NSError,
  /// [`errorDescription`](https://developer.apple.com/documentation/foundation/localizederror/errordescription)
  pub error_description: Option<String>,
}

/// [`UnsupportedFormatError`](https://developer.apple.com/documentation/diskimagekit/unsupportedformaterror)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedFormatError {
  /// [`underlyingError`](https://developer.apple.com/documentation/diskimagekit/unsupportedformaterror/underlyingerror),
  /// as an `NSError`.
  pub underlying_error: NSError,
  /// [`errorDescription`](https://developer.apple.com/documentation/foundation/localizederror/errordescription)
  pub error_description: Option<String>,
}

/// Its `errorDescription`, or, if it has none, its type's name.
fn describe(formatter: &mut fmt::Formatter<'_>, error_description: Option<&str>, name: &str) -> fmt::Result {
  formatter.write_str(error_description.unwrap_or(name))
}

impl fmt::Display for IncompatibleStackingError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    describe(
      formatter,
      self.error_description.as_deref(),
      "IncompatibleStackingError",
    )
  }
}

impl std::error::Error for IncompatibleStackingError {}

impl fmt::Display for InvalidBlockCountError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    describe(formatter, self.error_description.as_deref(), "InvalidBlockCountError")
  }
}

impl std::error::Error for InvalidBlockCountError {}

impl fmt::Display for CorruptedImageError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    describe(formatter, self.error_description.as_deref(), "CorruptedImageError")
  }
}

/// Its source is its underlying error.
impl std::error::Error for CorruptedImageError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(&self.underlying_error)
  }
}

impl fmt::Display for UnsupportedFormatError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    describe(formatter, self.error_description.as_deref(), "UnsupportedFormatError")
  }
}

/// Its source is its underlying error.
impl std::error::Error for UnsupportedFormatError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(&self.underlying_error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "DiskImageKit is only available on macOS"),
      Self::IncompatibleStacking(error) => error.fmt(formatter),
      Self::InvalidBlockCount(error) => error.fmt(formatter),
      Self::CorruptedImage(error) => error.fmt(formatter),
      Self::UnsupportedFormat(error) => error.fmt(formatter),
      Self::NSError(error) => error.fmt(formatter),
      Self::UnparseableUrl(error) => write!(formatter, "Foundation returned an unparseable URL: {error}"),
      Self::UnrepresentableUrl(url) => write!(formatter, "Foundation can't represent the URL {url}"),
    }
  }
}

/// The source of what DiskImageKit threw.
impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::CorruptedImage(error) => error.source(),
      Self::UnsupportedFormat(error) => error.source(),
      Self::NSError(error) => error.source(),
      Self::UnparseableUrl(error) => Some(error),
      Self::Unsupported | Self::IncompatibleStacking(_) | Self::InvalidBlockCount(_) | Self::UnrepresentableUrl(_) => {
        None
      }
    }
  }
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Self {
    Self::UnparseableUrl(error)
  }
}

/// What DiskImageKit threw, read through objc2. Off macOS,
/// `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl From<disk_image_kit_sys::DiskImageKitError> for Error {
  fn from(error: disk_image_kit_sys::DiskImageKitError) -> Self {
    use disk_image_kit_sys::DiskImageKitError as Thrown;

    match error {
      Thrown::IncompatibleStackingError {
        reason,
        error_description,
      } => Self::IncompatibleStacking(IncompatibleStackingError {
        reason,
        error_description,
      }),
      Thrown::InvalidBlockCountError { error_description } => {
        Self::InvalidBlockCount(InvalidBlockCountError { error_description })
      }
      Thrown::CorruptedImageError {
        underlying_error,
        error_description,
      } => Self::CorruptedImage(Box::new(CorruptedImageError {
        underlying_error: (&*underlying_error).into(),
        error_description,
      })),
      Thrown::UnsupportedFormatError {
        underlying_error,
        error_description,
      } => Self::UnsupportedFormat(Box::new(UnsupportedFormatError {
        underlying_error: (&*underlying_error).into(),
        error_description,
      })),
      Thrown::Error(error) => Self::NSError(Box::new((&*error).into())),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;
  use std::error::Error as _;

  use super::*;

  fn ns_error() -> NSError {
    NSError {
      domain: "NSPOSIXErrorDomain".to_string(),
      code: 2,
      user_info: BTreeMap::new(),
      localized_description: "No such file or directory".to_string(),
      localized_failure_reason: None,
      localized_recovery_suggestion: None,
      underlying_errors: vec![],
    }
  }

  #[test]
  fn says_diskimagekit_is_macos_only() {
    assert_eq!(
      Error::Unsupported.to_string(),
      "DiskImageKit is only available on macOS"
    );
  }

  #[test]
  fn says_an_errors_description() {
    let error = Error::IncompatibleStacking(IncompatibleStackingError {
      reason: "UUID mismatch".to_string(),
      error_description: Some("The layer isn't compatible with the stack.".to_string()),
    });

    assert_eq!(error.to_string(), "The layer isn't compatible with the stack.");
  }

  #[test]
  fn names_an_error_without_a_description() {
    let error = Error::InvalidBlockCount(InvalidBlockCountError {
      error_description: None,
    });

    assert_eq!(error.to_string(), "InvalidBlockCountError");
  }

  #[test]
  fn has_an_underlying_error_as_its_source() {
    let error = Error::CorruptedImage(Box::new(CorruptedImageError {
      underlying_error: ns_error(),
      error_description: None,
    }));

    assert_eq!(error.source().unwrap().to_string(), ns_error().to_string());
  }

  #[test]
  fn says_what_else_diskimagekit_threw() {
    assert_eq!(
      Error::NSError(Box::new(ns_error())).to_string(),
      "No such file or directory (NSPOSIXErrorDomain 2)"
    );
  }

  #[test]
  fn says_why_a_url_did_not_parse() {
    assert_eq!(
      Error::from(url::ParseError::RelativeUrlWithoutBase).to_string(),
      "Foundation returned an unparseable URL: relative URL without a base"
    );
  }
}
