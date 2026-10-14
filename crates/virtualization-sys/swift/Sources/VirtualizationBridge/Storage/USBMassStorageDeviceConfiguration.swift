//===----------------------------------------------------------------------===//
// `VZUSBMassStorageDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `attachment` is any `VZStorageDeviceAttachment`, lent by address
/// (`bridge_borrowed()`).
func vz_usb_mass_storage_device_configuration_init(attachment: UnsafeRawPointer)
  -> VZUSBMassStorageDeviceConfiguration
{
  VZUSBMassStorageDeviceConfiguration(attachment: bridging(borrowed: attachment) as VZStorageDeviceAttachment)
}

/// `object as? VZUSBMassStorageDeviceConfiguration`, for an abstract class's
/// object.
func vz_usb_mass_storage_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZUSBMassStorageDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBMassStorageDeviceConfiguration)
}
