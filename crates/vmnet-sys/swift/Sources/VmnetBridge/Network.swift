//===----------------------------------------------------------------------===//
// `vmnet_network_ref` and its functions.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import vmnet

/// A `vmnet_network_ref`.
final class VmnetNetwork {
  let object: AnyObject

  init(_ object: AnyObject) {
    self.object = object
  }

  var reference: vmnet_network_ref {
    bridging(reference: object)
  }
}

func vmnet_network_create(configuration: VmnetNetworkConfiguration) throws(BridgedVmnetReturn) -> VmnetNetwork {
  var status = vmnet_return_t.VMNET_SUCCESS

  guard let network = vmnet.vmnet_network_create(configuration.reference, &status) else {
    throw BridgedVmnetReturn(status)
  }

  return VmnetNetwork(bridging(retained: network))
}

/// A `vmnet_network_ref` another bridge lends by address for the call.
func vmnet_network_bridge_from(reference: UnsafeRawPointer) -> VmnetNetwork {
  VmnetNetwork(Unmanaged<AnyObject>.fromOpaque(reference).takeUnretainedValue())
}

extension VmnetNetwork {
  func bridge_retained() -> VmnetNetwork {
    VmnetNetwork(object)
  }

  func bridge_description() -> RustString {
    String(describing: object).intoRustString()
  }

  /// The `vmnet_network_ref`, for another bridge, for as long as this lives.
  func bridge_borrowed() -> UnsafeRawPointer {
    UnsafeRawPointer(Unmanaged.passUnretained(object).toOpaque())
  }

  /// `subnet` and `mask` as `bridging(inAddr:)` returns them.
  func bridge_get_ipv4_subnet() -> VmnetIpv4Subnet {
    var subnet = in_addr()
    var mask = in_addr()

    vmnet_network_get_ipv4_subnet(reference, &subnet, &mask)

    return VmnetIpv4Subnet(subnet: bridging(inAddr: subnet), mask: bridging(inAddr: mask))
  }

  /// `prefix` as `bridging(in6Addr:)` returns it.
  func bridge_get_ipv6_prefix() -> VmnetIpv6Prefix {
    var prefix = in6_addr()
    var prefixLen: UInt8 = 0

    vmnet_network_get_ipv6_prefix(reference, &prefix, &prefixLen)

    let (high, low) = bridging(in6Addr: prefix)

    return VmnetIpv6Prefix(prefix_high: high, prefix_low: low, prefix_len: prefixLen)
  }
}
