//===----------------------------------------------------------------------===//
// `VZXHCIControllerConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_xhci_controller_configuration_init() -> VZXHCIControllerConfiguration {
  VZXHCIControllerConfiguration()
}

/// `object as? VZXHCIControllerConfiguration`, for an abstract class's object.
func vz_xhci_controller_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZXHCIControllerConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZXHCIControllerConfiguration)
}
