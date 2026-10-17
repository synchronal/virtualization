//===----------------------------------------------------------------------===//
// `VZMacGraphicsDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZMacGraphicsDevice`, for an abstract class's object.
func vz_mac_graphics_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZMacGraphicsDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZMacGraphicsDevice)
}
