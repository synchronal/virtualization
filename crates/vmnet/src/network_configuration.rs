use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::returned;
use crate::platform::{self, ffi};
use crate::{Error, Mode};

/// [`vmnet_network_configuration_ref`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_ref)
pub struct NetworkConfiguration {
  pub(crate) inner: ffi::VmnetNetworkConfiguration,
}

impl NetworkConfiguration {
  /// [`vmnet_network_configuration_create(_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_create(_:_:))
  pub fn new(mode: Mode) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vmnet_network_configuration_create(mode.into())?,
    })
  }

  /// [`vmnet_network_configuration_add_dhcp_reservation(_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_add_dhcp_reservation(_:_:_:))
  pub fn add_dhcp_reservation(&self, client: [u8; 6], reservation: Ipv4Addr) -> Result<(), Error> {
    returned(self.inner.vmnet_add_dhcp_reservation(client, reservation))
  }

  /// [`vmnet_network_configuration_add_port_forwarding_rule(_:_:_:_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_add_port_forwarding_rule(_:_:_:_:_:_:)),
  /// whose `address_family` is `internal_address`'s.
  pub fn add_port_forwarding_rule(
    &self,
    protocol: u8,
    internal_port: u16,
    external_port: u16,
    internal_address: IpAddr,
  ) -> Result<(), Error> {
    returned(
      self
        .inner
        .vmnet_add_port_forwarding_rule(protocol, internal_port, external_port, internal_address),
    )
  }

  /// [`vmnet_network_configuration_disable_router_advertisement(_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_disable_router_advertisement(_:))
  pub fn disable_router_advertisement(&self) {
    self.inner.vmnet_disable_router_advertisement();
  }

  /// [`vmnet_network_configuration_disable_nat66(_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_disable_nat66(_:))
  pub fn disable_nat66(&self) {
    self.inner.vmnet_disable_nat66();
  }

  /// [`vmnet_network_configuration_disable_nat44(_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_disable_nat44(_:))
  pub fn disable_nat44(&self) {
    self.inner.vmnet_disable_nat44();
  }

  /// [`vmnet_network_configuration_disable_dns_proxy(_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_disable_dns_proxy(_:))
  pub fn disable_dns_proxy(&self) {
    self.inner.vmnet_disable_dns_proxy();
  }

  /// [`vmnet_network_configuration_disable_dhcp(_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_disable_dhcp(_:))
  pub fn disable_dhcp(&self) {
    self.inner.vmnet_disable_dhcp();
  }

  /// [`vmnet_network_configuration_set_external_interface(_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_set_external_interface(_:_:))
  pub fn set_external_interface(&self, interface_name: &str) -> Result<(), Error> {
    returned(self.inner.vmnet_set_external_interface(interface_name))
  }

  /// [`vmnet_network_configuration_set_ipv4_subnet(_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_set_ipv4_subnet(_:_:_:))
  pub fn set_ipv4_subnet(&self, subnet_addr: Ipv4Addr, subnet_mask: Ipv4Addr) -> Result<(), Error> {
    returned(self.inner.vmnet_set_ipv4_subnet(subnet_addr, subnet_mask))
  }

  /// [`vmnet_network_configuration_set_ipv6_prefix(_:_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_set_ipv6_prefix(_:_:_:))
  pub fn set_ipv6_prefix(&self, prefix: Ipv6Addr, len: u8) -> Result<(), Error> {
    returned(self.inner.vmnet_set_ipv6_prefix(prefix, len))
  }

  /// [`vmnet_network_configuration_set_mtu(_:_:)`](https://developer.apple.com/documentation/vmnet/vmnet_network_configuration_set_mtu(_:_:))
  pub fn set_mtu(&self, mtu: u32) -> Result<(), Error> {
    returned(self.inner.vmnet_set_mtu(mtu))
  }
}

impl Clone for NetworkConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's description.
impl fmt::Debug for NetworkConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      NetworkConfiguration::new(Mode::Shared),
      Err(Error::Unsupported)
    ));
  }
}
