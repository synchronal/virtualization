//===----------------------------------------------------------------------===//
// `VZMacGraphicsDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_graphics_device_configuration_init() -> VZMacGraphicsDeviceConfiguration {
  VZMacGraphicsDeviceConfiguration()
}

/// `object as? VZMacGraphicsDeviceConfiguration`, for an abstract class's
/// object.
func vz_mac_graphics_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacGraphicsDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacGraphicsDeviceConfiguration)
}

/// A new configuration's `displays` is `nil`, though declared nonnull: Swift
/// reads that as `[]`, which `as NSArray` alone skips to unwrap `nil`.
func vz_mac_graphics_device_configuration_displays(this: VZMacGraphicsDeviceConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: Array(this.displays) as NSArray)
}

/// `displays` is an array of `VZMacGraphicsDisplayConfiguration`.
func vz_mac_graphics_device_configuration_set_displays(
  this: VZMacGraphicsDeviceConfiguration,
  displays: UnsafeRawPointer
) {
  this.displays = bridging(borrowedArray: displays)
}
