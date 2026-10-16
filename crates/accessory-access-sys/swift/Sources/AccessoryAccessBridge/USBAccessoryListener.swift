//===----------------------------------------------------------------------===//
// `AAUSBAccessoryListener`, which Rust implements:
// `BridgedUSBAccessoryListener` forwards to Rust's.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess
import Synchronization

/// An `AAUSBAccessoryListener` that forwards to Rust's.
final class BridgedUSBAccessoryListener: NSObject, AAUSBAccessoryListener, @unchecked Sendable {
  let listener: AAUSBAccessoryListenerBridge

  init(_ listener: AAUSBAccessoryListenerBridge) {
    self.listener = listener
  }

  func usbAccessoryDidConnect(_ usbAccessory: AAUSBAccessory) {
    listener.usb_accessory_did_connect(usbAccessory)
  }

  func usbAccessoryDidDisconnect(_ usbAccessory: AAUSBAccessory) {
    listener.usb_accessory_did_disconnect(usbAccessory)
  }
}

/// The `BridgedUSBAccessoryListener` of each Rust listener, by the key Rust
/// gives it: AccessoryAccess knows a listener by its object, so registering
/// and unregistering one Rust listener must pass the same one. Each is kept
/// from registering until it is unregistered.
private let listeners = Mutex<[UInt: BridgedUSBAccessoryListener]>([:])

/// The `BridgedUSBAccessoryListener` for the Rust listener `key` names,
/// kept for registering.
func bridging(registeringListener listener: AAUSBAccessoryListenerBridge, key: UInt)
  -> BridgedUSBAccessoryListener
{
  listeners.withLock { listeners in
    if let bridged = listeners[key] {
      return bridged
    }

    let bridged = BridgedUSBAccessoryListener(listener)
    listeners[key] = bridged
    return bridged
  }
}

/// The `BridgedUSBAccessoryListener` for the Rust listener `key` names, for
/// unregistering: the one registered, or a new one for a listener never
/// registered, as AccessoryAccess would be given in Swift.
func bridging(unregisteringListener listener: AAUSBAccessoryListenerBridge, key: UInt)
  -> BridgedUSBAccessoryListener
{
  listeners.withLock { $0[key] } ?? BridgedUSBAccessoryListener(listener)
}

/// Forgets the listener `key` names, once AccessoryAccess has unregistered it.
func bridging(unregisteredListener bridged: BridgedUSBAccessoryListener, key: UInt) {
  listeners.withLock { listeners in
    if listeners[key] === bridged {
      listeners[key] = nil
    }
  }
}

extension AAUSBAccessoryListenerBridge: @unchecked Sendable {}
