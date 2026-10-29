//===----------------------------------------------------------------------===//
// `VZVirtioConsoleDevice` and `VZVirtioConsoleDeviceDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioConsoleDevice`, for an abstract class's object.
func vz_virtio_console_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZVirtioConsoleDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioConsoleDevice)
}

/// Rust's delegate, which `vz_virtio_console_device_set_delegate` set, by the
/// address Rust takes it from: `0` for none.
func vz_virtio_console_device_delegate(this: VZVirtioConsoleDevice) -> UInt {
  (this.delegate as? BridgedVirtioConsoleDeviceDelegate)?.delegate.bridge_retained() ?? 0
}

/// `delegate` is weak, so the device keeps the
/// `BridgedVirtioConsoleDeviceDelegate` it is given. Rust's delegate is weak in
/// turn.
func vz_virtio_console_device_set_delegate(this: VZVirtioConsoleDevice, delegate: VZVirtioConsoleDeviceDelegateBridge) {
  let delegate = BridgedVirtioConsoleDeviceDelegate(delegate)

  objc_setAssociatedObject(this, delegateKey.address, delegate, .OBJC_ASSOCIATION_RETAIN)
  this.delegate = delegate
}

extension VZVirtioConsoleDevice {
  func bridge_set_delegate_nil() {
    objc_setAssociatedObject(self, delegateKey.address, nil, .OBJC_ASSOCIATION_RETAIN)
    delegate = nil
  }

  func vz_ports() -> VZVirtioConsolePortArray {
    ports
  }
}

/// Where a device keeps its `BridgedVirtioConsoleDeviceDelegate`.
private let delegateKey = AssociationKey()

/// A `VZVirtioConsoleDeviceDelegate` that forwards to Rust's.
final class BridgedVirtioConsoleDeviceDelegate: NSObject, VZVirtioConsoleDeviceDelegate {
  let delegate: VZVirtioConsoleDeviceDelegateBridge

  init(_ delegate: VZVirtioConsoleDeviceDelegateBridge) {
    self.delegate = delegate
  }

  func consoleDevice(_ consoleDevice: VZVirtioConsoleDevice, didOpen consolePort: VZVirtioConsolePort) {
    delegate.console_device_did_open(consoleDevice, consolePort)
  }

  func consoleDevice(_ consoleDevice: VZVirtioConsoleDevice, didClose consolePort: VZVirtioConsolePort) {
    delegate.console_device_did_close(consoleDevice, consolePort)
  }
}
