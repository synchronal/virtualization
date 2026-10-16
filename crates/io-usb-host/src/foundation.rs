//! Foundation's types across the bridge: I/O buffers and thrown errors.

use crate::platform::ffi;
use crate::{Error, NSMutableData};

/// An `NSError` IOUSBHost failed with.
pub(crate) fn error(error: &ffi::NSError) -> Error {
  Error::NSError(Box::new(error.into()))
}

/// What a failing call left in its `error`.
pub(crate) fn thrown(thrown: ffi::Retained<ffi::NSError>) -> Error {
  error(&thrown)
}

/// The object an I/O buffer is.
#[cfg(target_os = "macos")]
pub(crate) fn ns_mutable_data(data: &NSMutableData) -> &ffi::NSMutableData {
  data.object()
}

/// Off macOS no buffer exists: this is never called.
#[cfg(not(target_os = "macos"))]
pub(crate) fn ns_mutable_data(data: &NSMutableData) -> &NSMutableData {
  data
}

/// An I/O buffer IOUSBHost made.
#[cfg(target_os = "macos")]
pub(crate) fn mutable_data(object: ffi::Retained<ffi::NSMutableData>) -> NSMutableData {
  NSMutableData::from_object(object)
}

/// Off macOS no buffer exists: this is never called.
#[cfg(not(target_os = "macos"))]
pub(crate) fn mutable_data(data: NSMutableData) -> NSMutableData {
  data
}
