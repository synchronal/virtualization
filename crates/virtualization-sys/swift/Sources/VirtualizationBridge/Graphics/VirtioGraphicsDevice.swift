//===----------------------------------------------------------------------===//
// `VZVirtioGraphicsDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioGraphicsDevice`, for an abstract class's object.
func vz_virtio_graphics_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZVirtioGraphicsDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioGraphicsDevice)
}
