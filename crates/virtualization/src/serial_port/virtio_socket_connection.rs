use std::fmt;
use std::os::fd::RawFd;

use crate::platform::ffi;

/// [`VZVirtioSocketConnection`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketconnection)
pub struct VirtioSocketConnection {
  pub(crate) inner: ffi::VZVirtioSocketConnection,
}

impl VirtioSocketConnection {
  /// [`destinationPort`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketconnection/destinationport)
  pub fn destination_port(&self) -> u32 {
    self.inner.vz_destination_port()
  }

  /// [`sourcePort`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketconnection/sourceport)
  pub fn source_port(&self) -> u32 {
    self.inner.vz_source_port()
  }

  /// [`fileDescriptor`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketconnection/filedescriptor):
  /// the connection's own, which it closes; `-1` once it has.
  pub fn file_descriptor(&self) -> RawFd {
    self.inner.vz_file_descriptor()
  }

  /// [`close()`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketconnection/close())
  pub fn close(&self) {
    self.inner.vz_close();
  }
}

impl Clone for VirtioSocketConnection {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSocketConnection {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
