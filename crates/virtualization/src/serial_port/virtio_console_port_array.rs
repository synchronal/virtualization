use std::fmt;

use crate::VirtioConsolePort;
use crate::platform::ffi;

/// [`VZVirtioConsolePortArray`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportarray)
pub struct VirtioConsolePortArray {
  inner: ffi::VZVirtioConsolePortArray,
}

impl VirtioConsolePortArray {
  pub(crate) fn bridge_from(inner: ffi::VZVirtioConsolePortArray) -> Self {
    Self { inner }
  }

  /// [`maximumPortCount`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportarray/maximumportcount)
  pub fn maximum_port_count(&self) -> u32 {
    self.inner.vz_maximum_port_count()
  }

  /// [`subscript(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportarray/subscript(_:))
  pub fn get(&self, port_index: isize) -> Option<VirtioConsolePort> {
    self
      .inner
      .vz_subscript(port_index)
      .ok()
      .map(VirtioConsolePort::bridge_from)
  }
}

impl Clone for VirtioConsolePortArray {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsolePortArray {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
