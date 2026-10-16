//===----------------------------------------------------------------------===//
// `VZUSBKeyboardConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_usb_keyboard_configuration_init() -> VZUSBKeyboardConfiguration {
  VZUSBKeyboardConfiguration()
}

/// `object as? VZUSBKeyboardConfiguration`, for an abstract class's object.
func vz_usb_keyboard_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZUSBKeyboardConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBKeyboardConfiguration)
}
