use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use super::{GUARDED, Never, VmnetMode, VmnetReturn};

pub(crate) struct VmnetNetworkConfiguration {
  never: Never,
}

pub(crate) fn vmnet_network_configuration_create(_mode: VmnetMode) -> Result<VmnetNetworkConfiguration, VmnetReturn> {
  unreachable!("{GUARDED}")
}

impl VmnetNetworkConfiguration {
  pub(crate) fn bridge_retained(&self) -> VmnetNetworkConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vmnet_add_dhcp_reservation(&self, _client: [u8; 6], _reservation: Ipv4Addr) -> VmnetReturn {
    match self.never {}
  }

  pub(crate) fn vmnet_add_port_forwarding_rule(
    &self,
    _protocol: u8,
    _internal_port: u16,
    _external_port: u16,
    _internal_address: IpAddr,
  ) -> VmnetReturn {
    match self.never {}
  }

  pub(crate) fn vmnet_disable_router_advertisement(&self) {
    match self.never {}
  }

  pub(crate) fn vmnet_disable_nat66(&self) {
    match self.never {}
  }

  pub(crate) fn vmnet_disable_nat44(&self) {
    match self.never {}
  }

  pub(crate) fn vmnet_disable_dns_proxy(&self) {
    match self.never {}
  }

  pub(crate) fn vmnet_disable_dhcp(&self) {
    match self.never {}
  }

  pub(crate) fn vmnet_set_external_interface(&self, _interface_name: &str) -> VmnetReturn {
    match self.never {}
  }

  pub(crate) fn vmnet_set_ipv4_subnet(&self, _subnet_addr: Ipv4Addr, _subnet_mask: Ipv4Addr) -> VmnetReturn {
    match self.never {}
  }

  pub(crate) fn vmnet_set_ipv6_prefix(&self, _prefix: Ipv6Addr, _len: u8) -> VmnetReturn {
    match self.never {}
  }

  pub(crate) fn vmnet_set_mtu(&self, _mtu: u32) -> VmnetReturn {
    match self.never {}
  }
}
