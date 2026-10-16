//===----------------------------------------------------------------------===//
// `VZUSBPassthroughDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_usb_passthrough_device_init_configuration(configuration: VZUSBPassthroughDeviceConfiguration)
  throws(BridgedThrown) -> VZUSBPassthroughDevice
{
  try bridging {
    try VZUSBPassthroughDevice(configuration: configuration)
  }
}

/// `object as? VZUSBPassthroughDevice`, for an abstract class's object.
func vz_usb_passthrough_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZUSBPassthroughDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBPassthroughDevice)
}
