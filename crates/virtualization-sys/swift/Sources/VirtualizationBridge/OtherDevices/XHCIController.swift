//===----------------------------------------------------------------------===//
// `VZXHCIController`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZXHCIController`, for an abstract class's object.
func vz_xhci_controller_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZXHCIController {
  try bridging(bridging(borrowed: object) as NSObject as? VZXHCIController)
}
