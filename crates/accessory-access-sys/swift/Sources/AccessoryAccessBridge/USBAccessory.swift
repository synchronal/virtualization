//===----------------------------------------------------------------------===//
// `AAUSBAccessory`. Its XPC representation and `NSSecureCoding` take XPC
// objects and coders, which aren't bridged.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess
import IOUSBHost

/// An `AAUSBAccessory` of an array Rust holds, lent by address for the call.
func aa_usb_accessory_bridge_from(object: UnsafeRawPointer) -> AAUSBAccessory {
  bridging(borrowed: object)
}

func aa_usb_accessory_device_descriptor_data(this: AAUSBAccessory) -> UnsafeMutableRawPointer {
  bridging(retained: this.deviceDescriptorData as NSData)
}

/// An `NSData` by address: `0` for `nil`.
func aa_usb_accessory_configuration_descriptor_data(this: AAUSBAccessory) -> UInt {
  bridging(retainedAddress: this.configurationDescriptorData.map { $0 as NSData })
}

/// `service_queue` is a `dispatch_queue_t` Rust lends by address, `0` for
/// `nil`. `completion_handler` takes the `IOUSBHostDevice`, which Rust takes
/// over, then what it failed with, each by address: one is `0`.
func aa_usb_accessory_open(
  this: AAUSBAccessory,
  service_queue: UInt,
  completion_handler: @escaping (UInt, UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)

  this.open(serviceQueue: bridging(borrowedAddress: service_queue)) { device, error in
    // The device is `nil` when the open failed.
    guard let error else {
      completionHandler.value(bridging(retainedAddress: device), 0)
      return
    }

    completionHandler.value(0, bridging(retainedError: error))
  }
}

/// `completion_handler` takes what it failed with, by address: `0` for
/// success.
func aa_usb_accessory_close(this: AAUSBAccessory, completion_handler: @escaping (UInt) -> Void) {
  let completionHandler = Bridged(value: completion_handler)

  this.close { completionHandler.value(bridging(retainedError: $0)) }
}

extension AAUSBAccessory {
  func aa_registry_id() -> UInt64 {
    registryID
  }
}
