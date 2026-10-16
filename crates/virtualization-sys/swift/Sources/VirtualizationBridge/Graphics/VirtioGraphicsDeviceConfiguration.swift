//===----------------------------------------------------------------------===//
// `VZVirtioGraphicsDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_graphics_device_configuration_init() -> VZVirtioGraphicsDeviceConfiguration {
  VZVirtioGraphicsDeviceConfiguration()
}

/// `object as? VZVirtioGraphicsDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_graphics_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioGraphicsDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioGraphicsDeviceConfiguration)
}

/// A new configuration's `scanouts` is `nil`, though declared nonnull: Swift
/// reads that as `[]`, which `as NSArray` alone skips to unwrap `nil`.
func vz_virtio_graphics_device_configuration_scanouts(
  this: VZVirtioGraphicsDeviceConfiguration
) -> UnsafeMutableRawPointer {
  bridging(retained: Array(this.scanouts) as NSArray)
}

/// `scanouts` is an array of `VZVirtioGraphicsScanoutConfiguration`.
func vz_virtio_graphics_device_configuration_set_scanouts(
  this: VZVirtioGraphicsDeviceConfiguration,
  scanouts: UnsafeRawPointer
) {
  this.scanouts = bridging(borrowedArray: scanouts)
}
