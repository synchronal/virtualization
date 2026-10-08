//===----------------------------------------------------------------------===//
// `VZVirtioConsoleDeviceSerialPortConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_console_device_serial_port_configuration_init() -> VZVirtioConsoleDeviceSerialPortConfiguration {
  VZVirtioConsoleDeviceSerialPortConfiguration()
}

/// `object as? VZVirtioConsoleDeviceSerialPortConfiguration`, for an abstract
/// class's object.
func vz_virtio_console_device_serial_port_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioConsoleDeviceSerialPortConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioConsoleDeviceSerialPortConfiguration)
}
