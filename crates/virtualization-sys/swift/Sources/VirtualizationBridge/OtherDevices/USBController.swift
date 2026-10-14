//===----------------------------------------------------------------------===//
// `VZUSBController`, abstract: its object crosses by address. Its delegate
// hears only of `VZUSBPassthroughDevice`s, and waits for them.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `this` is any `VZUSBController`; `device`, any `VZUSBDevice`.
func vz_usb_controller_attach(
  this: UnsafeRawPointer,
  device: UnsafeRawPointer,
  completion_handler: @escaping (UInt) -> Void
) {
  (bridging(borrowed: this) as VZUSBController).attach(device: bridging(usbDevice: device)) {
    completion_handler(bridging(retainedError: $0))
  }
}

/// `this` is any `VZUSBController`; `device`, any `VZUSBDevice`.
func vz_usb_controller_detach(
  this: UnsafeRawPointer,
  device: UnsafeRawPointer,
  completion_handler: @escaping (UInt) -> Void
) {
  (bridging(borrowed: this) as VZUSBController).detach(device: bridging(usbDevice: device)) {
    completion_handler(bridging(retainedError: $0))
  }
}

func vz_usb_controller_usb_devices(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: (bridging(borrowed: this) as VZUSBController).usbDevices as NSArray)
}
