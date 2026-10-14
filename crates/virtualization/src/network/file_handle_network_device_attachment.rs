use std::fmt;
use std::os::fd::{BorrowedFd, OwnedFd};

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, NetworkDeviceAttachment};

/// [`VZFileHandleNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzfilehandlenetworkdeviceattachment)
///
/// A `FileHandle` is the descriptor it wraps: the one given becomes a handle
/// that owns it, and the one returned is borrowed from the attachment, which
/// keeps its handle.
pub struct FileHandleNetworkDeviceAttachment {
  inner: ffi::VZFileHandleNetworkDeviceAttachment,
}

impl FileHandleNetworkDeviceAttachment {
  /// [`init(fileHandle:)`](https://developer.apple.com/documentation/virtualization/vzfilehandlenetworkdeviceattachment/init(filehandle:))
  pub fn new(file_handle: OwnedFd) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_file_handle_network_device_attachment_init(file_handle),
    })
  }

  /// [`fileHandle`](https://developer.apple.com/documentation/virtualization/vzfilehandlenetworkdeviceattachment/filehandle)
  pub fn file_handle(&self) -> BorrowedFd<'_> {
    self.inner.vz_file_handle()
  }

  /// [`maximumTransmissionUnit`](https://developer.apple.com/documentation/virtualization/vzfilehandlenetworkdeviceattachment/maximumtransmissionunit)
  pub fn maximum_transmission_unit(&self) -> isize {
    self.inner.vz_maximum_transmission_unit()
  }

  /// [`maximumTransmissionUnit`](https://developer.apple.com/documentation/virtualization/vzfilehandlenetworkdeviceattachment/maximumtransmissionunit)
  pub fn set_maximum_transmission_unit(&self, maximum_transmission_unit: isize) {
    self
      .inner
      .vz_set_maximum_transmission_unit(maximum_transmission_unit);
  }
}

impl Clone for FileHandleNetworkDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for FileHandleNetworkDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for FileHandleNetworkDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_file_handle_network_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl NetworkDeviceAttachment for FileHandleNetworkDeviceAttachment {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use std::os::unix::net::UnixDatagram;

  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let (socket, _peer) = UnixDatagram::pair().unwrap();

    assert!(matches!(
      FileHandleNetworkDeviceAttachment::new(OwnedFd::from(socket)),
      Err(Error::Unsupported)
    ));
  }
}
