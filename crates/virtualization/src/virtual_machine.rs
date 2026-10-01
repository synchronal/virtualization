//! [`VZVirtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine)

use virtualization_sys::ffi;

/// [`VZVirtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine)
pub struct VirtualMachine {
  _private: (),
}

impl VirtualMachine {
  /// [`isSupported`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/issupported)
  pub fn is_supported() -> bool {
    ffi::vz_virtual_machine_is_supported()
  }
}
