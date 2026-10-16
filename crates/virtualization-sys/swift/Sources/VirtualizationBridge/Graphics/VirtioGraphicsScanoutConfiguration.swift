//===----------------------------------------------------------------------===//
// `VZVirtioGraphicsScanoutConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_graphics_scanout_configuration_init(
  width_in_pixels: Int,
  height_in_pixels: Int
) -> VZVirtioGraphicsScanoutConfiguration {
  VZVirtioGraphicsScanoutConfiguration(widthInPixels: width_in_pixels, heightInPixels: height_in_pixels)
}

/// An element of an array of them, which Rust holds.
func vz_virtio_graphics_scanout_configuration_bridge_from(object: UnsafeRawPointer)
  -> VZVirtioGraphicsScanoutConfiguration
{
  bridging(borrowed: object)
}

/// `object as? VZVirtioGraphicsScanoutConfiguration`, for an abstract class's
/// object.
func vz_virtio_graphics_scanout_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioGraphicsScanoutConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioGraphicsScanoutConfiguration)
}

extension VZVirtioGraphicsScanoutConfiguration {
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
}
