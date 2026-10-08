//===----------------------------------------------------------------------===//
// `VZNetworkDeviceConfiguration`, abstract: its object crosses by address.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_network_device_configuration_mac_address(this: UnsafeRawPointer) -> VZMACAddress {
  (bridging(borrowed: this) as VZNetworkDeviceConfiguration).macAddress
}

func vz_network_device_configuration_set_mac_address(this: UnsafeRawPointer, mac_address: VZMACAddress) {
  (bridging(borrowed: this) as VZNetworkDeviceConfiguration).macAddress = mac_address
}

func vz_network_device_configuration_attachment(this: UnsafeRawPointer) -> UInt {
  bridging(retainedAddress: (bridging(borrowed: this) as VZNetworkDeviceConfiguration).attachment)
}

/// `attachment` is any `VZNetworkDeviceAttachment`, or `0` for `nil`.
func vz_network_device_configuration_set_attachment(this: UnsafeRawPointer, attachment: UInt) {
  (bridging(borrowed: this) as VZNetworkDeviceConfiguration).attachment = bridging(borrowedAddress: attachment)
}
