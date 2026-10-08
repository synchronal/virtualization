//===----------------------------------------------------------------------===//
// `VZVirtioEntropyDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_entropy_device_configuration_init() -> VZVirtioEntropyDeviceConfiguration {
  VZVirtioEntropyDeviceConfiguration()
}

/// `object as? VZVirtioEntropyDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_entropy_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioEntropyDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioEntropyDeviceConfiguration)
}
