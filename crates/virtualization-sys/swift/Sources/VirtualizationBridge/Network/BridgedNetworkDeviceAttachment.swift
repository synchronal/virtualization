//===----------------------------------------------------------------------===//
// `VZBridgedNetworkDeviceAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_bridged_network_device_attachment_init(interface: VZBridgedNetworkInterface)
  -> VZBridgedNetworkDeviceAttachment
{
  VZBridgedNetworkDeviceAttachment(interface: interface)
}

/// `object as? VZBridgedNetworkDeviceAttachment`, for an abstract class's
/// object.
func vz_bridged_network_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZBridgedNetworkDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZBridgedNetworkDeviceAttachment)
}

extension VZBridgedNetworkDeviceAttachment {
  func vz_interface() -> VZBridgedNetworkInterface {
    interface
  }
}
