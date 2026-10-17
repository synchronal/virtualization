//===----------------------------------------------------------------------===//
// `VZMacTrackpadConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_trackpad_configuration_init() -> VZMacTrackpadConfiguration {
  VZMacTrackpadConfiguration()
}

/// `object as? VZMacTrackpadConfiguration`, for an abstract class's object.
func vz_mac_trackpad_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacTrackpadConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacTrackpadConfiguration)
}
