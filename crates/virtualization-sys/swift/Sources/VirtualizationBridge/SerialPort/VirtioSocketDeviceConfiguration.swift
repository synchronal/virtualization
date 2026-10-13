//===----------------------------------------------------------------------===//
// `VZVirtioSocketDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_socket_device_configuration_init() -> VZVirtioSocketDeviceConfiguration {
  VZVirtioSocketDeviceConfiguration()
}

/// `object as? VZVirtioSocketDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_socket_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioSocketDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioSocketDeviceConfiguration)
}
