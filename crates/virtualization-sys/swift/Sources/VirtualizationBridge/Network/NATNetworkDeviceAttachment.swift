//===----------------------------------------------------------------------===//
// `VZNATNetworkDeviceAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_nat_network_device_attachment_init() -> VZNATNetworkDeviceAttachment {
  VZNATNetworkDeviceAttachment()
}

/// `object as? VZNATNetworkDeviceAttachment`, for an abstract class's object.
func vz_nat_network_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZNATNetworkDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZNATNetworkDeviceAttachment)
}
