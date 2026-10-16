//===----------------------------------------------------------------------===//
// `VZVirtioGraphicsScanout`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioGraphicsScanout`, for an abstract class's object.
func vz_virtio_graphics_scanout_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZVirtioGraphicsScanout {
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioGraphicsScanout)
}
