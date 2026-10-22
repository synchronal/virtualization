//===----------------------------------------------------------------------===//
// `VZVirtioDeviceSpecificConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `configuration_data` is an `NSData`, lent for the call.
func vz_virtio_device_specific_configuration_init(configuration_data: UnsafeRawPointer)
  -> VZVirtioDeviceSpecificConfiguration
{
  VZVirtioDeviceSpecificConfiguration(configurationData: bridging(borrowed: configuration_data) as NSData as Data)
}

func vz_virtio_device_specific_configuration_configuration_data(this: VZVirtioDeviceSpecificConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.configurationData as NSData)
}
