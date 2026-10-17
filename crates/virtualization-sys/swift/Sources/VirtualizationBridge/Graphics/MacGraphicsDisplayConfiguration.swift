//===----------------------------------------------------------------------===//
// `VZMacGraphicsDisplayConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AppKit
import Virtualization

func vz_mac_graphics_display_configuration_init(
  width_in_pixels: Int,
  height_in_pixels: Int,
  pixels_per_inch: Int
) -> VZMacGraphicsDisplayConfiguration {
  VZMacGraphicsDisplayConfiguration(
    widthInPixels: width_in_pixels,
    heightInPixels: height_in_pixels,
    pixelsPerInch: pixels_per_inch
  )
}

/// `screen` is an `NSScreen`, lent for the call.
func vz_mac_graphics_display_configuration_init_for_size_in_points(
  screen: UnsafeRawPointer,
  size_in_points: BridgedCGSize
) -> VZMacGraphicsDisplayConfiguration {
  VZMacGraphicsDisplayConfiguration(for: bridging(borrowed: screen), sizeInPoints: CGSize(size_in_points))
}

/// An element of an array of them, which Rust holds.
func vz_mac_graphics_display_configuration_bridge_from(object: UnsafeRawPointer) -> VZMacGraphicsDisplayConfiguration {
  bridging(borrowed: object)
}

/// `object as? VZMacGraphicsDisplayConfiguration`, for an abstract class's
/// object.
func vz_mac_graphics_display_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacGraphicsDisplayConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacGraphicsDisplayConfiguration)
}

extension VZMacGraphicsDisplayConfiguration {
  func vz_width_in_pixels() -> Int {
    widthInPixels
  }

  func vz_set_width_in_pixels(width_in_pixels: Int) {
    widthInPixels = width_in_pixels
  }

  func vz_height_in_pixels() -> Int {
    heightInPixels
  }

  func vz_set_height_in_pixels(height_in_pixels: Int) {
    heightInPixels = height_in_pixels
  }

  func vz_pixels_per_inch() -> Int {
    pixelsPerInch
  }

  func vz_set_pixels_per_inch(pixels_per_inch: Int) {
    pixelsPerInch = pixels_per_inch
  }
}
