use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, NetworkDeviceAttachment};

/// [`VZVmnetNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzvmnetnetworkdeviceattachment)
///
/// A `vmnet_network_ref` is [`vmnet_framework::Network`].
pub struct VmnetNetworkDeviceAttachment {
  inner: ffi::VZVmnetNetworkDeviceAttachment,
}

impl VmnetNetworkDeviceAttachment {
  /// [`init(network:)`](https://developer.apple.com/documentation/virtualization/vzvmnetnetworkdeviceattachment/init(network:))
  pub fn new(network: &vmnet_framework::Network) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_vmnet_network_device_attachment_init(vmnet_framework::bridge::network(network)),
    })
  }

  /// [`network`](https://developer.apple.com/documentation/virtualization/vzvmnetnetworkdeviceattachment/network)
  pub fn network(&self) -> vmnet_framework::Network {
    vmnet_framework::bridge::network_from(self.inner.vz_network())
  }
}

impl Clone for VmnetNetworkDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VmnetNetworkDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VmnetNetworkDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_vmnet_network_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl NetworkDeviceAttachment for VmnetNetworkDeviceAttachment {}
