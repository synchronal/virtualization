use std::fmt;

use crate::USBController;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZXHCIController`](https://developer.apple.com/documentation/virtualization/vzxhcicontroller)
pub struct XHCIController {
  inner: ffi::VZXHCIController,
}

impl Clone for XHCIController {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for XHCIController {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for XHCIController {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_xhci_controller_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl USBController for XHCIController {}
