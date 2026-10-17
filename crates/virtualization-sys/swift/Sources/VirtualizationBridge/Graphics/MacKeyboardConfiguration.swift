//===----------------------------------------------------------------------===//
// `VZMacKeyboardConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_keyboard_configuration_init() -> VZMacKeyboardConfiguration {
  VZMacKeyboardConfiguration()
}

/// `object as? VZMacKeyboardConfiguration`, for an abstract class's object.
func vz_mac_keyboard_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacKeyboardConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacKeyboardConfiguration)
}
