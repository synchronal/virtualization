//! Foundation's types across the bridge: errors.

use crate::Error;
use crate::platform::ffi;

/// An `NSError` AccessoryAccess failed with.
pub(crate) fn error(error: &ffi::NSError) -> Error {
  Error::NSError(Box::new(error.into()))
}

/// What a completion handler was called with: an error, or `nil` for success.
pub(crate) fn completed(error: Option<ffi::Retained<ffi::NSError>>) -> Result<(), Error> {
  error.map_or(Ok(()), |error| Err(self::error(&error)))
}
