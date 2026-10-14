//===----------------------------------------------------------------------===//
// `VZVmnetNetworkDeviceAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization
import vmnet

/// `network` is a `vmnet_network_ref` VmnetBridge lends by address.
func vz_vmnet_network_device_attachment_init(network: UnsafeRawPointer) -> VZVmnetNetworkDeviceAttachment {
  VZVmnetNetworkDeviceAttachment(network: OpaquePointer(network))
}

/// `object as? VZVmnetNetworkDeviceAttachment`, for an abstract class's
/// object.
func vz_vmnet_network_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVmnetNetworkDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVmnetNetworkDeviceAttachment)
}

/// `network`, by address, for VmnetBridge to take another reference to while
/// the attachment holds it.
func vz_vmnet_network_device_attachment_network(this: VZVmnetNetworkDeviceAttachment) -> UnsafeRawPointer {
  UnsafeRawPointer(this.network)
}
