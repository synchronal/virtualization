//===----------------------------------------------------------------------===//
// `vmnet_network_configuration_ref` and its functions.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import vmnet

/// A `vmnet_network_configuration_ref`.
final class VmnetNetworkConfiguration {
  let object: AnyObject

  init(_ object: AnyObject) {
    self.object = object
  }

  var reference: vmnet_network_configuration_ref {
    bridging(reference: object)
  }
}

func vmnet_network_configuration_create(mode: BridgedVmnetMode) throws(BridgedVmnetReturn)
  -> VmnetNetworkConfiguration
{
  var status = vmnet_return_t.VMNET_SUCCESS

  guard let configuration = vmnet.vmnet_network_configuration_create(vmnet_mode_t(mode), &status) else {
    throw BridgedVmnetReturn(status)
  }

  return VmnetNetworkConfiguration(bridging(retained: configuration))
}

extension VmnetNetworkConfiguration {
  func bridge_retained() -> VmnetNetworkConfiguration {
    VmnetNetworkConfiguration(object)
  }

  func bridge_description() -> RustString {
    String(describing: object).intoRustString()
  }

  /// `client` is an `ether_addr_t`, packed as `bridging(ethernetAddress:)`
  /// packs it; `reservation`, an `in_addr`, as `bridging(inAddr:)`.
  func bridge_add_dhcp_reservation(client: UInt64, reservation: UInt32) -> BridgedVmnetReturn {
    var client = bridging(ethernetAddress: client)
    var reservation = bridging(inAddr: reservation)

    return BridgedVmnetReturn(vmnet_network_configuration_add_dhcp_reservation(reference, &client, &reservation))
  }

  /// `vmnet_network_configuration_add_port_forwarding_rule` with an `AF_INET`
  /// `internal_address`, passed as `bridging(inAddr:)` takes it. `protocol`
  /// is `ip_protocol`: Swift's generated glue doesn't escape the keyword.
  func bridge_add_port_forwarding_rule_in_addr(
    ip_protocol: UInt8,
    internal_port: UInt16,
    external_port: UInt16,
    internal_address: UInt32
  ) -> BridgedVmnetReturn {
    var internalAddress = bridging(inAddr: internal_address)

    return BridgedVmnetReturn(
      vmnet_network_configuration_add_port_forwarding_rule(
        reference,
        ip_protocol,
        sa_family_t(AF_INET),
        internal_port,
        external_port,
        &internalAddress
      )
    )
  }

  /// `vmnet_network_configuration_add_port_forwarding_rule` with an
  /// `AF_INET6` `internal_address`, passed as `bridging(in6AddrHigh:low:)`
  /// takes it.
  func bridge_add_port_forwarding_rule_in6_addr(
    ip_protocol: UInt8,
    internal_port: UInt16,
    external_port: UInt16,
    internal_address_high: UInt64,
    internal_address_low: UInt64
  ) -> BridgedVmnetReturn {
    var internalAddress = bridging(in6AddrHigh: internal_address_high, low: internal_address_low)

    return BridgedVmnetReturn(
      vmnet_network_configuration_add_port_forwarding_rule(
        reference,
        ip_protocol,
        sa_family_t(AF_INET6),
        internal_port,
        external_port,
        &internalAddress
      )
    )
  }

  func vmnet_disable_router_advertisement() {
    vmnet_network_configuration_disable_router_advertisement(reference)
  }

  func vmnet_disable_nat66() {
    vmnet_network_configuration_disable_nat66(reference)
  }

  func vmnet_disable_nat44() {
    vmnet_network_configuration_disable_nat44(reference)
  }

  func vmnet_disable_dns_proxy() {
    vmnet_network_configuration_disable_dns_proxy(reference)
  }

  func vmnet_disable_dhcp() {
    vmnet_network_configuration_disable_dhcp(reference)
  }

  func vmnet_set_external_interface(interface_name: RustStr) -> BridgedVmnetReturn {
    BridgedVmnetReturn(vmnet_network_configuration_set_external_interface(reference, interface_name.toString()))
  }

  /// Each an `in_addr`, as `bridging(inAddr:)` takes it.
  func bridge_set_ipv4_subnet(subnet_addr: UInt32, subnet_mask: UInt32) -> BridgedVmnetReturn {
    var subnetAddr = bridging(inAddr: subnet_addr)
    var subnetMask = bridging(inAddr: subnet_mask)

    return BridgedVmnetReturn(vmnet_network_configuration_set_ipv4_subnet(reference, &subnetAddr, &subnetMask))
  }

  /// `prefix` is an `in6_addr`, as `bridging(in6AddrHigh:low:)` takes it.
  func bridge_set_ipv6_prefix(prefix_high: UInt64, prefix_low: UInt64, len: UInt8) -> BridgedVmnetReturn {
    var prefix = bridging(in6AddrHigh: prefix_high, low: prefix_low)

    return BridgedVmnetReturn(vmnet_network_configuration_set_ipv6_prefix(reference, &prefix, len))
  }

  func vmnet_set_mtu(mtu: UInt32) -> BridgedVmnetReturn {
    BridgedVmnetReturn(vmnet_network_configuration_set_mtu(reference, mtu))
  }
}
