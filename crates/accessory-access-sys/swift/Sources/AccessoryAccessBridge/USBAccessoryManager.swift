//===----------------------------------------------------------------------===//
// `AAUSBAccessoryManager`. `events(matching:)` is only `async`, so isn't
// bridged, nor is the `AAUSBAccessory.Event` it yields.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess

func aa_usb_accessory_manager_shared() -> AAUSBAccessoryManager {
  AAUSBAccessoryManager.shared
}

/// `key` names Rust's listener (see `bridging(registeringListener:key:)`);
/// `matching_criteria` is an `NSArray` of `AAUSBAccessoryMatchingCriteria`,
/// lent for the call. `completion_handler` takes the accessories, an
/// `NSArray` Rust takes over, then what it failed with, each by address.
func aa_usb_accessory_manager_register_listener(
  this: AAUSBAccessoryManager,
  listener: AAUSBAccessoryListenerBridge,
  key: UInt,
  matching_criteria: UnsafeRawPointer,
  completion_handler: @escaping (UInt, UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)

  this.registerListener(
    bridging(registeringListener: listener, key: key),
    matchingCriteria: bridging(borrowedArray: matching_criteria)
  ) { accessories, error in
    completionHandler.value(bridging(retainedAddress: accessories as NSArray), bridging(retainedError: error))
  }
}

/// `key` names Rust's listener (see `bridging(registeringListener:key:)`).
func aa_usb_accessory_manager_unregister_listener(
  this: AAUSBAccessoryManager,
  listener: AAUSBAccessoryListenerBridge,
  key: UInt,
  completion_handler: @escaping () -> Void
) {
  let completionHandler = Bridged(value: completion_handler)
  let bridged = bridging(unregisteringListener: listener, key: key)

  this.unregisterListener(bridged) {
    bridging(unregisteredListener: bridged, key: key)
    completionHandler.value()
  }
}
