//===----------------------------------------------------------------------===//
// `VZUSBController`, abstract: its object crosses by address. Its delegate,
// `VZUSBController.Delegate`, is Rust's.
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

/// Rust's delegate, which `vz_usb_controller_set_delegate` set, by the
/// address Rust takes it from: `0` for none.
func vz_usb_controller_delegate(this: UnsafeRawPointer) -> UInt {
  ((bridging(borrowed: this) as VZUSBController).delegate as? BridgedUSBControllerDelegate)?.delegate
    .bridge_retained() ?? 0
}

/// `delegate` is weak, so the controller keeps the
/// `BridgedUSBControllerDelegate` it is given. Rust's delegate is weak in
/// turn.
func vz_usb_controller_set_delegate(this: UnsafeRawPointer, delegate: VZUSBControllerDelegateBridge) {
  let controller: VZUSBController = bridging(borrowed: this)
  let delegate = BridgedUSBControllerDelegate(delegate)

  objc_setAssociatedObject(controller, &delegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
  controller.delegate = delegate
}

func vz_usb_controller_set_delegate_nil(this: UnsafeRawPointer) {
  let controller: VZUSBController = bridging(borrowed: this)

  objc_setAssociatedObject(controller, &delegateKey, nil, .OBJC_ASSOCIATION_RETAIN)
  controller.delegate = nil
}

/// Where a controller keeps its `BridgedUSBControllerDelegate`. Only its
/// address is used.
nonisolated(unsafe) private var delegateKey: UInt8 = 0

/// A `VZUSBController.Delegate` that forwards to Rust's.
final class BridgedUSBControllerDelegate: NSObject, VZUSBController.Delegate {
  let delegate: VZUSBControllerDelegateBridge

  init(_ delegate: VZUSBControllerDelegateBridge) {
    self.delegate = delegate
  }

  /// `usbController` crosses by address, retained for Rust.
  func usbController(
    _ usbController: VZUSBController,
    usbPassthroughDeviceDidDisconnect device: VZUSBPassthroughDevice
  ) {
    delegate.usb_controller_usb_passthrough_device_did_disconnect(
      UInt(bitPattern: bridging(retained: usbController)), device)
  }
}
