use std::ffi::c_void;
use std::fmt;

use crate::platform::ffi;

/// [`VZGuestMemoryMapping`](https://developer.apple.com/documentation/virtualization/vzguestmemorymapping)
pub struct GuestMemoryMapping {
  pub(crate) inner: ffi::VZGuestMemoryMapping,
}

impl GuestMemoryMapping {
  /// [`mutableBytes`](https://developer.apple.com/documentation/virtualization/vzguestmemorymapping/mutablebytes):
  /// the guest's memory, which it may change at any moment, for the caller's
  /// own `unsafe`. Valid as long as the mapping, and only until the machine
  /// stops or reboots.
  pub fn mutable_bytes(&self) -> *mut c_void {
    self.inner.vz_mutable_bytes()
  }

  /// [`physicalAddress`](https://developer.apple.com/documentation/virtualization/vzguestmemorymapping/physicaladdress)
  pub fn physical_address(&self) -> u64 {
    self.inner.vz_physical_address()
  }

  /// [`length`](https://developer.apple.com/documentation/virtualization/vzguestmemorymapping/length)
  pub fn length(&self) -> isize {
    self.inner.vz_length()
  }
}

impl Clone for GuestMemoryMapping {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for GuestMemoryMapping {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
