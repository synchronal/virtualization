//===----------------------------------------------------------------------===//
// `VZVirtioTraditionalMemoryBalloonDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_traditional_memory_balloon_device_configuration_init()
  -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration
{
  VZVirtioTraditionalMemoryBalloonDeviceConfiguration()
}

/// `object as? VZVirtioTraditionalMemoryBalloonDeviceConfiguration`, for an
/// abstract class's object.
func vz_virtio_traditional_memory_balloon_device_configuration_bridge_cast(object: UnsafeRawPointer)
  throws(BridgedNil) -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioTraditionalMemoryBalloonDeviceConfiguration)
}
