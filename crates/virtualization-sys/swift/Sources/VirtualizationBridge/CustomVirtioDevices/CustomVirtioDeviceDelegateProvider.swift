//===----------------------------------------------------------------------===//
// `VZCustomVirtioDeviceDelegateProvider` and
// `VZCustomVirtioDeviceConfigurationDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `device_queue` is a `dispatch_queue_t`, lent for the call. `delegate` is
/// weak, so the provider keeps the
/// `BridgedCustomVirtioDeviceConfigurationDelegate` it is given. Rust's
/// delegate is weak in turn.
func vz_custom_virtio_device_delegate_provider_init(
  device_queue: UnsafeRawPointer,
  delegate: VZCustomVirtioDeviceConfigurationDelegateBridge
) -> VZCustomVirtioDeviceDelegateProvider {
  let delegate = BridgedCustomVirtioDeviceConfigurationDelegate(delegate)
  let provider = VZCustomVirtioDeviceDelegateProvider(deviceQueue: bridging(borrowed: device_queue), delegate: delegate)

  objc_setAssociatedObject(provider, delegateKey.address, delegate, .OBJC_ASSOCIATION_RETAIN)
  return provider
}

/// `object as? VZCustomVirtioDeviceDelegateProvider`, for an abstract class's
/// object.
func vz_custom_virtio_device_delegate_provider_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZCustomVirtioDeviceDelegateProvider
{
  try bridging(bridging(borrowed: object) as NSObject as? VZCustomVirtioDeviceDelegateProvider)
}

func vz_custom_virtio_device_delegate_provider_device_queue(this: VZCustomVirtioDeviceDelegateProvider)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.deviceQueue)
}

/// Rust's delegate, which `vz_custom_virtio_device_delegate_provider_init` set,
/// by the address Rust takes it from: `0` for none.
func vz_custom_virtio_device_delegate_provider_delegate(this: VZCustomVirtioDeviceDelegateProvider) -> UInt {
  (this.delegate as? BridgedCustomVirtioDeviceConfigurationDelegate)?.delegate.bridge_retained() ?? 0
}

/// Where a provider keeps its `BridgedCustomVirtioDeviceConfigurationDelegate`.
private let delegateKey = AssociationKey()

/// A `VZCustomVirtioDeviceConfigurationDelegate` that forwards to Rust's.
final class BridgedCustomVirtioDeviceConfigurationDelegate: NSObject, VZCustomVirtioDeviceConfigurationDelegate {
  let delegate: VZCustomVirtioDeviceConfigurationDelegateBridge

  init(_ delegate: VZCustomVirtioDeviceConfigurationDelegateBridge) {
    self.delegate = delegate
  }

  func customVirtioConfiguration(
    _ deviceConfiguration: VZCustomVirtioDeviceConfiguration,
    didCreateDevice device: VZCustomVirtioDevice
  ) {
    delegate.custom_virtio_configuration_did_create_device(deviceConfiguration, device)
  }
}
