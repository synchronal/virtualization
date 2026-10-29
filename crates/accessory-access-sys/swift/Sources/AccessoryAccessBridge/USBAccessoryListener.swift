//===----------------------------------------------------------------------===//
// `AAUSBAccessoryListener`, which Rust implements:
// `BridgedUSBAccessoryListener` forwards to Rust's.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess
import Synchronization

/// An `AAUSBAccessoryListener` that forwards to Rust's.
final class BridgedUSBAccessoryListener: NSObject, AAUSBAccessoryListener, Sendable {
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

/// A Rust listener's `BridgedUSBAccessoryListener`, with how many of its
/// registrations are in flight and whether one has succeeded.
private struct Registration {
  let bridged: BridgedUSBAccessoryListener
  var pending = 0
  var registered = false
}

/// The `Registration` of each Rust listener, by the key Rust gives it:
/// AccessoryAccess knows a listener by its object, so registering and
/// unregistering one Rust listener must pass the same one. Each is kept from
/// registering until an unregister starts, or every registration in flight
/// has failed and none succeeded.
private let listeners = Mutex<[UInt: Registration]>([:])

/// The `BridgedUSBAccessoryListener` for the Rust listener `key` names, kept
/// for a registration now in flight: one already kept belongs to an earlier
/// registration.
func bridging(registeringListener listener: AAUSBAccessoryListenerBridge, key: UInt)
  -> BridgedUSBAccessoryListener
{
  listeners.withLock { listeners in
    var registration = listeners[key] ?? Registration(bridged: BridgedUSBAccessoryListener(listener))
    registration.pending += 1
    listeners[key] = registration
    return registration.bridged
  }
}

/// Ends a registration of `bridged` in flight, forgetting it once none are
/// and none has succeeded. One unregistered since is no longer kept: the Rust
/// listener `key` names may have been kept again, for a new registration.
func bridging(registeredListener bridged: BridgedUSBAccessoryListener, key: UInt, succeeded: Bool) {
  listeners.withLock { listeners in
    guard var registration = listeners[key], registration.bridged === bridged else {
      return
    }

    registration.pending -= 1
    registration.registered = registration.registered || succeeded
    listeners[key] = registration.pending == 0 && !registration.registered ? nil : registration
  }
}

/// The `BridgedUSBAccessoryListener` for the Rust listener `key` names, no
/// longer kept, for unregistering: the one registered, or a new one for a
/// listener never registered, as AccessoryAccess would be given in Swift. A
/// registration from here on is a new one, even while this unregister runs.
func bridging(unregisteringListener listener: AAUSBAccessoryListenerBridge, key: UInt)
  -> BridgedUSBAccessoryListener
{
  listeners.withLock { $0.removeValue(forKey: key) }?.bridged ?? BridgedUSBAccessoryListener(listener)
}

extension AAUSBAccessoryListenerBridge: @unchecked Sendable {}
