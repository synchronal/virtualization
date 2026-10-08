use std::fmt;
use std::os::fd::{BorrowedFd, OwnedFd};

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, SerialPortAttachment};

/// [`VZFileHandleSerialPortAttachment`](https://developer.apple.com/documentation/virtualization/vzfilehandleserialportattachment)
///
/// A `FileHandle` is the descriptor it wraps: each one given becomes a handle
/// that owns it, and each one returned is borrowed from the attachment, which
/// keeps its handles.
pub struct FileHandleSerialPortAttachment {
  inner: ffi::VZFileHandleSerialPortAttachment,
}

impl FileHandleSerialPortAttachment {
  /// [`init(fileHandleForReading:fileHandleForWriting:)`](https://developer.apple.com/documentation/virtualization/vzfilehandleserialportattachment/init(filehandleforreading:filehandleforwriting:))
  pub fn new(
    file_handle_for_reading: Option<OwnedFd>,
    file_handle_for_writing: Option<OwnedFd>,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_file_handle_serial_port_attachment_init(file_handle_for_reading, file_handle_for_writing),
    })
  }

  /// [`fileHandleForReading`](https://developer.apple.com/documentation/virtualization/vzfilehandleserialportattachment/filehandleforreading)
  pub fn file_handle_for_reading(&self) -> Option<BorrowedFd<'_>> {
    self.inner.vz_file_handle_for_reading()
  }

  /// [`fileHandleForWriting`](https://developer.apple.com/documentation/virtualization/vzfilehandleserialportattachment/filehandleforwriting)
  pub fn file_handle_for_writing(&self) -> Option<BorrowedFd<'_>> {
    self.inner.vz_file_handle_for_writing()
  }
}

impl Clone for FileHandleSerialPortAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for FileHandleSerialPortAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for FileHandleSerialPortAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_file_handle_serial_port_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SerialPortAttachment for FileHandleSerialPortAttachment {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      FileHandleSerialPortAttachment::new(None, None),
      Err(Error::Unsupported)
    ));
  }
}
