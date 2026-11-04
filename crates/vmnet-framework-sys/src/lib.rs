//! FFI to Apple's [vmnet] framework, through swift-bridge. Use the
//! `vmnet-framework` crate instead; this one holds its `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them, the way
//! `virtualization-sys`'s are:
//!
//! - vmnet is C: a reference type crosses in a Swift class named for it
//!   without `_ref` (`vmnet_network_ref` is [`VmnetNetwork`]), which holds
//!   the reference and releases it with its last use. Its functions are
//!   methods `vmnet_<function without the type's prefix>`; those that make
//!   one are free functions with Apple's names.
//! - A function that returns `NULL` and sets its `status` returns
//!   `Result<T, VmnetReturn>`.
//! - C's structs cross as integers in host byte order: `in_addr` a `u32`,
//!   `in6_addr` its first and last eight octets as two `u64`s, `ether_addr_t`
//!   its six octets in a `u64`, first octet lowest. An out parameter pair
//!   returns a shared struct.
//! - Enums keep Apple's cases, plus `Unknown(rawValue)` for a case newer than
//!   this crate.
//!
//! The glue's names differ from the other bridges': all link into one binary,
//! and swift-bridge names symbols for each bridge's types.
//!
//! macOS only: the build fails elsewhere.
//!
//! [vmnet]: https://developer.apple.com/documentation/vmnet

use std::ffi::c_void;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use self::ffi::{
  VmnetMode, VmnetNetwork, VmnetNetworkConfiguration, VmnetReturn, vmnet_network_configuration_create,
};

// No doc comments inside the module: swift-bridge can't parse them. Shared
// enums are renamed on the Swift side, where Apple's names are taken.
#[swift_bridge::bridge]
mod ffi {
  // Return.swift

  #[swift_bridge(swift_name = "BridgedVmnetReturn")]
  enum VmnetReturn {
    Success,
    Failure,
    MemFailure,
    InvalidArgument,
    SetupIncomplete,
    InvalidAccess,
    PacketTooBig,
    BufferExhausted,
    TooManyPackets,
    SharingServiceBusy,
    NotAuthorized,
    Unknown(u32),
  }

  // Mode.swift

  #[swift_bridge(swift_name = "BridgedVmnetMode")]
  enum VmnetMode {
    Host,
    Shared,
    Bridged,
    Unknown(u32),
  }

  // Network.swift

  #[swift_bridge(swift_repr = "struct")]
  struct VmnetIpv4Subnet {
    subnet: u32,
    mask: u32,
  }

  #[swift_bridge(swift_repr = "struct")]
  struct VmnetIpv6Prefix {
    prefix_high: u64,
    prefix_low: u64,
    prefix_len: u8,
  }

  extern "Swift" {
    type VmnetNetwork;

    fn vmnet_network_create(configuration: VmnetNetworkConfiguration) -> Result<VmnetNetwork, VmnetReturn>;
    fn vmnet_network_bridge_from(reference: *const c_void) -> VmnetNetwork;
    fn bridge_retained(self: &VmnetNetwork) -> VmnetNetwork;
    fn bridge_description(self: &VmnetNetwork) -> String;
    fn bridge_borrowed(self: &VmnetNetwork) -> *const c_void;
    fn bridge_get_ipv4_subnet(self: &VmnetNetwork) -> VmnetIpv4Subnet;
    fn bridge_get_ipv6_prefix(self: &VmnetNetwork) -> VmnetIpv6Prefix;
  }

  // NetworkConfiguration.swift

  extern "Swift" {
    type VmnetNetworkConfiguration;

    fn vmnet_network_configuration_create(mode: VmnetMode) -> Result<VmnetNetworkConfiguration, VmnetReturn>;
    fn bridge_retained(self: &VmnetNetworkConfiguration) -> VmnetNetworkConfiguration;
    fn bridge_description(self: &VmnetNetworkConfiguration) -> String;
    fn bridge_add_dhcp_reservation(self: &VmnetNetworkConfiguration, client: u64, reservation: u32) -> VmnetReturn;
    fn bridge_add_port_forwarding_rule_in_addr(
      self: &VmnetNetworkConfiguration,
      ip_protocol: u8,
      internal_port: u16,
      external_port: u16,
      internal_address: u32,
    ) -> VmnetReturn;
    fn bridge_add_port_forwarding_rule_in6_addr(
      self: &VmnetNetworkConfiguration,
      ip_protocol: u8,
      internal_port: u16,
      external_port: u16,
      internal_address_high: u64,
      internal_address_low: u64,
    ) -> VmnetReturn;
    fn vmnet_disable_router_advertisement(self: &VmnetNetworkConfiguration);
    fn vmnet_disable_nat66(self: &VmnetNetworkConfiguration);
    fn vmnet_disable_nat44(self: &VmnetNetworkConfiguration);
    fn vmnet_disable_dns_proxy(self: &VmnetNetworkConfiguration);
    fn vmnet_disable_dhcp(self: &VmnetNetworkConfiguration);
    fn vmnet_set_external_interface(self: &VmnetNetworkConfiguration, interface_name: &str) -> VmnetReturn;
    fn bridge_set_ipv4_subnet(self: &VmnetNetworkConfiguration, subnet_addr: u32, subnet_mask: u32) -> VmnetReturn;
    fn bridge_set_ipv6_prefix(
      self: &VmnetNetworkConfiguration,
      prefix_high: u64,
      prefix_low: u64,
      len: u8,
    ) -> VmnetReturn;
    fn vmnet_set_mtu(self: &VmnetNetworkConfiguration, mtu: u32) -> VmnetReturn;
  }
}

/// An `in6_addr` as Swift's `bridging(in6AddrHigh:low:)` takes it.
fn in6_addr(address: Ipv6Addr) -> (u64, u64) {
  let address = u128::from(address);

  ((address >> 64) as u64, address as u64)
}

// Network.swift

pub fn vmnet_network_create(configuration: &VmnetNetworkConfiguration) -> Result<VmnetNetwork, VmnetReturn> {
  ffi::vmnet_network_create(configuration.bridge_retained())
}

/// Another reference to the `vmnet_network_ref` at `reference`.
///
/// # Safety
///
/// `reference` is a `vmnet_network_ref`, alive for the call: e.g. one another
/// bridge's object holds.
pub unsafe fn vmnet_network_bridge_from(reference: *const c_void) -> VmnetNetwork {
  ffi::vmnet_network_bridge_from(reference)
}

impl VmnetNetwork {
  /// `subnet` and `mask`.
  pub fn vmnet_get_ipv4_subnet(&self) -> (Ipv4Addr, Ipv4Addr) {
    let subnet = self.bridge_get_ipv4_subnet();

    (Ipv4Addr::from(subnet.subnet), Ipv4Addr::from(subnet.mask))
  }

  /// `prefix` and `prefix_len`.
  pub fn vmnet_get_ipv6_prefix(&self) -> (Ipv6Addr, u8) {
    let prefix = self.bridge_get_ipv6_prefix();

    (
      Ipv6Addr::from(u128::from(prefix.prefix_high) << 64 | u128::from(prefix.prefix_low)),
      prefix.prefix_len,
    )
  }
}

// NetworkConfiguration.swift

impl VmnetNetworkConfiguration {
  /// `client` is an `ether_addr_t`'s six octets.
  pub fn vmnet_add_dhcp_reservation(&self, client: [u8; 6], reservation: Ipv4Addr) -> VmnetReturn {
    let mut packed = [0; 8];

    packed[..6].copy_from_slice(&client);
    self.bridge_add_dhcp_reservation(u64::from_le_bytes(packed), u32::from(reservation))
  }

  /// `address_family` is `internal_address`'s: `AF_INET` or `AF_INET6`.
  pub fn vmnet_add_port_forwarding_rule(
    &self,
    protocol: u8,
    internal_port: u16,
    external_port: u16,
    internal_address: IpAddr,
  ) -> VmnetReturn {
    match internal_address {
      IpAddr::V4(address) => {
        self.bridge_add_port_forwarding_rule_in_addr(protocol, internal_port, external_port, u32::from(address))
      }
      IpAddr::V6(address) => {
        let (high, low) = in6_addr(address);

        self.bridge_add_port_forwarding_rule_in6_addr(protocol, internal_port, external_port, high, low)
      }
    }
  }

  pub fn vmnet_set_ipv4_subnet(&self, subnet_addr: Ipv4Addr, subnet_mask: Ipv4Addr) -> VmnetReturn {
    self.bridge_set_ipv4_subnet(u32::from(subnet_addr), u32::from(subnet_mask))
  }

  pub fn vmnet_set_ipv6_prefix(&self, prefix: Ipv6Addr, len: u8) -> VmnetReturn {
    let (high, low) = in6_addr(prefix);

    self.bridge_set_ipv6_prefix(high, low, len)
  }
}
