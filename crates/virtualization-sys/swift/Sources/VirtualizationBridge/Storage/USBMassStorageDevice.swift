//===----------------------------------------------------------------------===//
// `VZUSBMassStorageDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_usb_mass_storage_device_init(configuration: VZUSBMassStorageDeviceConfiguration) -> VZUSBMassStorageDevice {
  VZUSBMassStorageDevice(configuration: configuration)
}

/// `object as? VZUSBMassStorageDevice`, for an abstract class's object.
func vz_usb_mass_storage_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZUSBMassStorageDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZUSBMassStorageDevice)
}
