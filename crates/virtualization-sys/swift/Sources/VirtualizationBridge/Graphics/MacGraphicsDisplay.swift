//===----------------------------------------------------------------------===//
// `VZMacGraphicsDisplay`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZMacGraphicsDisplay`, for an abstract class's object.
func vz_mac_graphics_display_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZMacGraphicsDisplay {
  try bridging(bridging(borrowed: object) as NSObject as? VZMacGraphicsDisplay)
}

extension VZMacGraphicsDisplay {
  func vz_pixels_per_inch() -> Int {
    pixelsPerInch
  }
}
