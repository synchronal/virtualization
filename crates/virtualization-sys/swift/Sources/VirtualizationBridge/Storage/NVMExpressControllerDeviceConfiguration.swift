//===----------------------------------------------------------------------===//
// `VZNVMExpressControllerDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `attachment` is any `VZStorageDeviceAttachment`, lent by address
/// (`bridge_borrowed()`).
func vz_nvm_express_controller_device_configuration_init(attachment: UnsafeRawPointer)
  -> VZNVMExpressControllerDeviceConfiguration
{
  VZNVMExpressControllerDeviceConfiguration(attachment: bridging(borrowed: attachment) as VZStorageDeviceAttachment)
}

/// `object as? VZNVMExpressControllerDeviceConfiguration`, for an abstract
/// class's object.
func vz_nvm_express_controller_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZNVMExpressControllerDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZNVMExpressControllerDeviceConfiguration)
}
