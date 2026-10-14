use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{BridgedNetworkInterface, Error, NetworkDeviceAttachment};

/// [`VZBridgedNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment)
pub struct BridgedNetworkDeviceAttachment {
  inner: ffi::VZBridgedNetworkDeviceAttachment,
}

impl BridgedNetworkDeviceAttachment {
  /// [`init(interface:)`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment/init(interface:))
  pub fn new(interface: &BridgedNetworkInterface) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_bridged_network_device_attachment_init(&interface.inner),
    })
  }

  /// [`interface`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkdeviceattachment/interface)
  pub fn interface(&self) -> BridgedNetworkInterface {
    BridgedNetworkInterface::from_inner(self.inner.vz_interface())
  }
}

impl Clone for BridgedNetworkDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for BridgedNetworkDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for BridgedNetworkDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_bridged_network_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl NetworkDeviceAttachment for BridgedNetworkDeviceAttachment {}
