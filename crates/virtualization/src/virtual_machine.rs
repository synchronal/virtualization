use crate::Error;
use crate::platform::{self, ffi};

/// [`VZVirtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine)
pub struct VirtualMachine {
  _private: (),
}

impl VirtualMachine {
  /// [`isSupported`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/issupported)
  pub fn is_supported() -> Result<bool, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_is_supported())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtualMachine::is_supported(), Err(Error::Unsupported)));
  }
}
