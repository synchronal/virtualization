use std::net::{Ipv4Addr, Ipv6Addr};

use super::{GUARDED, Local, Never, VmnetNetworkConfiguration, VmnetReturn};

pub(crate) struct VmnetNetwork {
  never: Never,
  local: Local,
}

pub(crate) fn vmnet_network_create(_configuration: &VmnetNetworkConfiguration) -> Result<VmnetNetwork, VmnetReturn> {
  unreachable!("{GUARDED}")
}

impl VmnetNetwork {
  pub(crate) fn bridge_retained(&self) -> VmnetNetwork {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vmnet_get_ipv4_subnet(&self) -> (Ipv4Addr, Ipv4Addr) {
    match self.never {}
  }

  pub(crate) fn vmnet_get_ipv6_prefix(&self) -> (Ipv6Addr, u8) {
    match self.never {}
  }
}
