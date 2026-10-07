//===----------------------------------------------------------------------===//
// `VZStorageDeviceConfiguration`, abstract: its object crosses by address.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_storage_device_configuration_attachment(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: (bridging(borrowed: this) as VZStorageDeviceConfiguration).attachment)
}
