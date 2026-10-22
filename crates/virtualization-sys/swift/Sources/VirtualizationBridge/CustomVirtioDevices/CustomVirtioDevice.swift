//===----------------------------------------------------------------------===//
// `VZCustomVirtioDevice` and `VZCustomVirtioDeviceDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// Rust's delegate, which `vz_custom_virtio_device_set_delegate` set, by the
/// address Rust takes it from: `0` for none.
func vz_custom_virtio_device_delegate(this: VZCustomVirtioDevice) -> UInt {
  (this.delegate as? BridgedCustomVirtioDeviceDelegate)?.delegate.bridge_retained() ?? 0
}

/// `delegate` is weak, so the device keeps the
/// `BridgedCustomVirtioDeviceDelegate` it is given. Rust's delegate is weak in
/// turn.
func vz_custom_virtio_device_set_delegate(this: VZCustomVirtioDevice, delegate: VZCustomVirtioDeviceDelegateBridge) {
  let delegate = BridgedCustomVirtioDeviceDelegate(delegate)

  objc_setAssociatedObject(this, &delegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
  this.delegate = delegate
}

func vz_custom_virtio_device_device_queue(this: VZCustomVirtioDevice) -> UnsafeMutableRawPointer {
  bridging(retained: this.deviceQueue)
}

func vz_custom_virtio_device_shared_memory_regions(this: VZCustomVirtioDevice) -> UnsafeMutableRawPointer {
  bridging(retained: this.sharedMemoryRegions as NSArray)
}

func vz_custom_virtio_device_update(
  this: VZCustomVirtioDevice,
  new_configuration: VZVirtioDeviceSpecificConfiguration,
  completion_handler: @escaping (UInt) -> Void
) {
  let completionHandler = Bridged(value: completion_handler)

  this.update(new_configuration) { completionHandler.value(bridging(retainedError: $0)) }
}

extension VZCustomVirtioDevice {
  func bridge_set_delegate_nil() {
    objc_setAssociatedObject(self, &delegateKey, nil, .OBJC_ASSOCIATION_RETAIN)
    delegate = nil
  }

  func vz_negotiated_features() throws(BridgedNil) -> VZNegotiatedVirtioFeatureSet {
    try bridging(negotiatedFeatures)
  }

  func vz_guest_memory_mapping(physical_address: UInt64, length: Int) throws(BridgedNil) -> VZGuestMemoryMapping {
    try bridging(guestMemoryMapping(atPhysicalAddress: physical_address, length: length))
  }

  func vz_queue(index: UInt16) throws(BridgedNil) -> VZVirtioQueue {
    try bridging(queue(at: index))
  }

  func vz_request_reset() {
    requestReset()
  }
}

/// Where a device keeps its `BridgedCustomVirtioDeviceDelegate`. Only its
/// address is used.
nonisolated(unsafe) private var delegateKey: UInt8 = 0

/// A `VZCustomVirtioDeviceDelegate` that forwards to Rust's.
final class BridgedCustomVirtioDeviceDelegate: NSObject, VZCustomVirtioDeviceDelegate {
  let delegate: VZCustomVirtioDeviceDelegateBridge

  init(_ delegate: VZCustomVirtioDeviceDelegateBridge) {
    self.delegate = delegate
  }

  func customVirtioDevice(_ device: VZCustomVirtioDevice, didReceiveNotificationFor queue: VZVirtioQueue) {
    delegate.custom_virtio_device_did_receive_notification_for(device, queue)
  }

  func customVirtioDeviceDidAcceptDriverOk(_ device: VZCustomVirtioDevice) {
    delegate.custom_virtio_device_did_accept_driver_ok(device)
  }

  func customVirtioDeviceWillStop(_ device: VZCustomVirtioDevice) {
    delegate.custom_virtio_device_will_stop(device)
  }

  func customVirtioDeviceWillPause(_ device: VZCustomVirtioDevice) {
    delegate.custom_virtio_device_will_pause(device)
  }

  func customVirtioDeviceWillResume(_ device: VZCustomVirtioDevice) {
    delegate.custom_virtio_device_will_resume(device)
  }

  func customVirtioDeviceWillReset(_ device: VZCustomVirtioDevice) {
    delegate.custom_virtio_device_will_reset(device)
  }

  /// Rust hands over the `NSData` by address, `0` for `nil`.
  func customVirtioDeviceSaveState(forRestore device: VZCustomVirtioDevice) -> Data? {
    (bridging(takenAddress: delegate.custom_virtio_device_save_state_for_restore(device)) as NSData?)
      .map { $0 as Data }
  }

  func customVirtioDeviceShouldRestore(_ device: VZCustomVirtioDevice, saveState: Data) -> Bool {
    delegate.custom_virtio_device_should_restore_save_state(device, bridging(retained: saveState as NSData))
  }
}
