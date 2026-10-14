//===----------------------------------------------------------------------===//
// `VZUSBControllerConfiguration`, abstract: its object crosses by address.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_usb_controller_configuration_usb_devices(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: (bridging(borrowed: this) as VZUSBControllerConfiguration).usbDevices as NSArray)
}

/// `usb_devices` is an array of any `VZUSBDeviceConfiguration`.
func vz_usb_controller_configuration_set_usb_devices(this: UnsafeRawPointer, usb_devices: UnsafeRawPointer) {
  (bridging(borrowed: this) as VZUSBControllerConfiguration).usbDevices = bridging(borrowedArray: usb_devices)
}
