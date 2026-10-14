use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::platform::{self, ffi};
use crate::{Error, NetworkConfiguration};

/// [`vmnet_network_ref`](https://developer.apple.com/documentation/vmnet/vmnet_network_ref)
pub struct Network {
  pub(crate) inner: ffi::VmnetNetwork,
}

impl Network {
  /// [`vmnet_network_create(_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_create(_:_:))
  pub fn new(configuration: &NetworkConfiguration) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vmnet_network_create(&configuration.inner)?,
    })
  }

  /// [`vmnet_network_get_ipv4_subnet(_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_get_ipv4_subnet(_:_:_:)):
  /// `subnet` and `mask`.
  pub fn ipv4_subnet(&self) -> (Ipv4Addr, Ipv4Addr) {
    self.inner.vmnet_get_ipv4_subnet()
  }

  /// [`vmnet_network_get_ipv6_prefix(_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_get_ipv6_prefix(_:_:_:)):
  /// `prefix` and `prefix_len`.
  pub fn ipv6_prefix(&self) -> (Ipv6Addr, u8) {
    self.inner.vmnet_get_ipv6_prefix()
  }
}

impl Clone for Network {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's description.
impl fmt::Debug for Network {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}
