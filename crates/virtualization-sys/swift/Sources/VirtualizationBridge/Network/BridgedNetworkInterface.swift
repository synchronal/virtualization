//===----------------------------------------------------------------------===//
// `VZBridgedNetworkInterface`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// An element of an array of them, which Rust holds.
func vz_bridged_network_interface_bridge_from(object: UnsafeRawPointer) -> VZBridgedNetworkInterface {
  bridging(borrowed: object)
}

func vz_bridged_network_interface_network_interfaces() -> UnsafeMutableRawPointer {
  bridging(retained: VZBridgedNetworkInterface.networkInterfaces as NSArray)
}

extension VZBridgedNetworkInterface {
  func vz_identifier() -> RustString {
    identifier.intoRustString()
  }

  func vz_localized_display_name() -> RustString? {
    localizedDisplayName?.intoRustString()
  }
}
