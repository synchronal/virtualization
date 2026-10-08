//===----------------------------------------------------------------------===//
// `VZVirtioNetworkDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_network_device_configuration_init() -> VZVirtioNetworkDeviceConfiguration {
  VZVirtioNetworkDeviceConfiguration()
}

/// `object as? VZVirtioNetworkDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_network_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioNetworkDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioNetworkDeviceConfiguration)
}
