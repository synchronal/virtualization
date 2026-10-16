//===----------------------------------------------------------------------===//
// `VZGraphicsDevice`, abstract: its object crosses by address.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_graphics_device_displays(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: (bridging(borrowed: this) as VZGraphicsDevice).displays as NSArray)
}
