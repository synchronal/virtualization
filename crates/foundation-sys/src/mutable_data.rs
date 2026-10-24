//! A mutable buffer whose identity matters, as plain Rust.

use std::fmt;

/// [`NSMutableData`](https://developer.apple.com/documentation/foundation/nsmutabledata),
/// for a buffer a framework makes and reads and writes as itself, such as
/// IOUSBHost's I/O buffers. Only what Rust needs to reach its bytes is here.
///
/// A shared reference, as in Swift: [`Clone`] is the same object. [`Debug`]
/// is its `description`. Off macOS none exists.
pub struct NSMutableData {
  #[cfg(target_os = "macos")]
  object: crate::objc::Retained<crate::objc::NSMutableData>,
  #[cfg(not(target_os = "macos"))]
  never: Never,
  /// Neither `Send` nor `Sync`, as objc2's `NSMutableData` isn't.
  #[cfg(not(target_os = "macos"))]
  local: std::marker::PhantomData<*const ()>,
}

#[cfg(not(target_os = "macos"))]
enum Never {}

impl NSMutableData {
  /// [`length`](https://developer.apple.com/documentation/foundation/nsdata/length)
  pub fn length(&self) -> usize {
    #[cfg(target_os = "macos")]
    {
      self.object.len()
    }
    #[cfg(not(target_os = "macos"))]
    {
      match self.never {}
    }
  }

  /// A copy of [`bytes`](https://developer.apple.com/documentation/foundation/nsdata/bytes).
  pub fn bytes(&self) -> Vec<u8> {
    #[cfg(target_os = "macos")]
    {
      self.object.to_vec()
    }
    #[cfg(not(target_os = "macos"))]
    {
      match self.never {}
    }
  }

  /// [`replaceBytes(in:withBytes:)`](https://developer.apple.com/documentation/foundation/nsmutabledata/replacebytes(in:withbytes:)),
  /// over the `bytes.len()` bytes from `location`: Foundation copies as many
  /// as the range holds.
  pub fn replace_bytes(&self, location: usize, bytes: &[u8]) {
    #[cfg(target_os = "macos")]
    {
      crate::objc::ns_mutable_data_replace_bytes(&self.object, location, bytes);
    }
    #[cfg(not(target_os = "macos"))]
    {
      let _ = (location, bytes);
      match self.never {}
    }
  }
}

#[cfg(target_os = "macos")]
impl NSMutableData {
  /// The object, for a sys crate to lend across its bridge.
  pub fn object(&self) -> &crate::objc::NSMutableData {
    &self.object
  }

  /// One a sys crate's bridge returned.
  pub fn from_object(object: crate::objc::Retained<crate::objc::NSMutableData>) -> Self {
    Self { object }
  }
}

impl Clone for NSMutableData {
  fn clone(&self) -> Self {
    #[cfg(target_os = "macos")]
    {
      Self {
        object: self.object.clone(),
      }
    }
    #[cfg(not(target_os = "macos"))]
    {
      match self.never {}
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NSMutableData {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[cfg(target_os = "macos")]
    {
      formatter.write_str(&crate::objc::ns_description(&self.object))
    }
    #[cfg(not(target_os = "macos"))]
    {
      let _ = formatter;
      match self.never {}
    }
  }
}
