use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, NetworkDeviceAttachment};

/// [`VZNATNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vznatnetworkdeviceattachment)
pub struct NATNetworkDeviceAttachment {
  inner: ffi::VZNATNetworkDeviceAttachment,
}

impl NATNetworkDeviceAttachment {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vznatnetworkdeviceattachment/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_nat_network_device_attachment_init(),
    })
  }
}

impl Clone for NATNetworkDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NATNetworkDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for NATNetworkDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_nat_network_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl NetworkDeviceAttachment for NATNetworkDeviceAttachment {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(NATNetworkDeviceAttachment::new(), Err(Error::Unsupported)));
  }
}
