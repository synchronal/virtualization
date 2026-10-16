//===----------------------------------------------------------------------===//
// `VZUSBScreenCoordinatePointingDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_usb_screen_coordinate_pointing_device_configuration_init() -> VZUSBScreenCoordinatePointingDeviceConfiguration {
  VZUSBScreenCoordinatePointingDeviceConfiguration()
}

/// `object as? VZUSBScreenCoordinatePointingDeviceConfiguration`, for an abstract class's object.
func vz_usb_screen_coordinate_pointing_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZUSBScreenCoordinatePointingDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBScreenCoordinatePointingDeviceConfiguration)
}
