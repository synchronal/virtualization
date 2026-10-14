//===----------------------------------------------------------------------===//
// `VZUSBDevice`, a protocol only Virtualization's classes conform to: its
// object crosses by address, as an abstract class's does.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// A `VZUSBDevice` Rust lends for the call.
func bridging(usbDevice pointer: UnsafeRawPointer) -> any VZUSBDevice {
  bridging(borrowed: pointer) as NSObject as! any VZUSBDevice
}

func vz_usb_device_usb_controller(this: UnsafeRawPointer) -> UInt {
  bridging(retainedAddress: bridging(usbDevice: this).usbController)
}

func vz_usb_device_uuid(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: bridging(usbDevice: this).uuid as NSUUID)
}
