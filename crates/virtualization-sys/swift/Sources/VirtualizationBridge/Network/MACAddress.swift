//===----------------------------------------------------------------------===//
// `VZMACAddress`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_address_init_ethernet_address(ethernet_address: UInt64) -> VZMACAddress {
  VZMACAddress(ethernetAddress: bridging(ethernetAddress: ethernet_address))
}

func vz_mac_address_init_string(string: RustStr) throws(BridgedNil) -> VZMACAddress {
  try bridging(VZMACAddress(string: string.toString()))
}

func vz_mac_address_random_locally_administered() -> VZMACAddress {
  VZMACAddress.randomLocallyAdministered()
}

extension VZMACAddress {
  /// `ethernetAddress`, packed as `bridging(ethernetAddress:)` packs it.
  func bridge_ethernet_address() -> UInt64 {
    bridging(ethernetAddress: ethernetAddress)
  }

  func vz_string() -> RustString {
    string.intoRustString()
  }

  func vz_is_broadcast_address() -> Bool {
    isBroadcastAddress
  }

  func vz_is_multicast_address() -> Bool {
    isMulticastAddress
  }

  func vz_is_unicast_address() -> Bool {
    isUnicastAddress
  }

  func vz_is_locally_administered_address() -> Bool {
    isLocallyAdministeredAddress
  }

  func vz_is_universally_administered_address() -> Bool {
    isUniversallyAdministeredAddress
  }
}

/// An `ether_addr_t` Rust passes as its six octets in a `u64`, first octet
/// lowest. (swift-bridge has no fixed-size arrays.)
private func bridging(ethernetAddress packed: UInt64) -> ether_addr_t {
  func octet(_ index: UInt64) -> UInt8 {
    UInt8(truncatingIfNeeded: packed >> (8 * index))
  }

  return ether_addr_t(octet: (octet(0), octet(1), octet(2), octet(3), octet(4), octet(5)))
}

/// An `ether_addr_t` for Rust, as its six octets in a `u64`, first octet
/// lowest.
private func bridging(ethernetAddress address: ether_addr_t) -> UInt64 {
  let octets = address.octet

  return [octets.0, octets.1, octets.2, octets.3, octets.4, octets.5].enumerated()
    .reduce(0) { packed, element in packed | UInt64(element.element) << (8 * UInt64(element.offset)) }
}
