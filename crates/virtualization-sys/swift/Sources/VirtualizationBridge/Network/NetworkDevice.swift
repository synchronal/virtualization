//===----------------------------------------------------------------------===//
// `VZNetworkDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// An element of an array of them, which Rust holds.
func vz_network_device_bridge_from(object: UnsafeRawPointer) -> VZNetworkDevice {
  bridging(borrowed: object)
}

func vz_network_device_attachment(this: VZNetworkDevice) -> UInt {
  bridging(retainedAddress: this.attachment)
}

/// `attachment` is any `VZNetworkDeviceAttachment`, or `0` for `nil`.
func vz_network_device_set_attachment(this: VZNetworkDevice, attachment: UInt) {
  this.attachment = bridging(borrowedAddress: attachment)
}
