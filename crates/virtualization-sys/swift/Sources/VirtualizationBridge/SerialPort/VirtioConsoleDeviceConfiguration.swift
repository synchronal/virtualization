//===----------------------------------------------------------------------===//
// `VZVirtioConsoleDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_console_device_configuration_init() -> VZVirtioConsoleDeviceConfiguration {
  VZVirtioConsoleDeviceConfiguration()
}

/// `object as? VZVirtioConsoleDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_console_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioConsoleDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioConsoleDeviceConfiguration)
}

extension VZVirtioConsoleDeviceConfiguration {
  func vz_ports() -> VZVirtioConsolePortConfigurationArray {
    ports
  }
}
