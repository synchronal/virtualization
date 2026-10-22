use std::ffi::c_void;

use super::Never;

pub(crate) struct VZGuestMemoryMapping {
  never: Never,
}

impl VZGuestMemoryMapping {
  pub(crate) fn bridge_retained(&self) -> VZGuestMemoryMapping {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_mutable_bytes(&self) -> *mut c_void {
    match self.never {}
  }

  pub(crate) fn vz_physical_address(&self) -> u64 {
    match self.never {}
  }

  pub(crate) fn vz_length(&self) -> isize {
    match self.never {}
  }
}
