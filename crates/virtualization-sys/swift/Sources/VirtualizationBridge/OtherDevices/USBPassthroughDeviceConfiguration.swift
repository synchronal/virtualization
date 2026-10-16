//===----------------------------------------------------------------------===//
// `VZUSBPassthroughDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess
import Virtualization

/// `device` is an `AAUSBAccessory` AccessoryAccessBridge lends by address.
func vz_usb_passthrough_device_configuration_init(device: UnsafeRawPointer) -> VZUSBPassthroughDeviceConfiguration {
  VZUSBPassthroughDeviceConfiguration(device: bridging(borrowed: device) as AAUSBAccessory)
}

/// `object as? VZUSBPassthroughDeviceConfiguration`, for an abstract class's
/// object.
func vz_usb_passthrough_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZUSBPassthroughDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBPassthroughDeviceConfiguration)
}
