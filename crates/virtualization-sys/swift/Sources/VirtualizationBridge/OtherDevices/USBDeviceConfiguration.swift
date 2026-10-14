//===----------------------------------------------------------------------===//
// `VZUSBDeviceConfiguration`, a protocol only Virtualization's classes conform
// to: its object crosses by address, as an abstract class's does.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// A `VZUSBDeviceConfiguration` Rust lends for the call.
private func bridging(usbDeviceConfiguration pointer: UnsafeRawPointer) -> any VZUSBDeviceConfiguration {
  bridging(borrowed: pointer) as NSObject as! any VZUSBDeviceConfiguration
}

func vz_usb_device_configuration_uuid(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: bridging(usbDeviceConfiguration: this).uuid as NSUUID)
}

func vz_usb_device_configuration_set_uuid(this: UnsafeRawPointer, uuid: UnsafeRawPointer) {
  bridging(usbDeviceConfiguration: this).uuid = bridging(borrowed: uuid) as NSUUID as UUID
}
