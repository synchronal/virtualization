//===----------------------------------------------------------------------===//
// `VZVirtioBlockDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `attachment` is any `VZStorageDeviceAttachment`, lent by address
/// (`bridge_borrowed()`).
func vz_virtio_block_device_configuration_init(attachment: UnsafeRawPointer) -> VZVirtioBlockDeviceConfiguration {
  VZVirtioBlockDeviceConfiguration(attachment: bridging(borrowed: attachment) as VZStorageDeviceAttachment)
}

/// `object as? VZVirtioBlockDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_block_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioBlockDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioBlockDeviceConfiguration)
}

func vz_virtio_block_device_configuration_validate_block_device_identifier(block_device_identifier: RustStr)
  throws(BridgedThrown)
{
  try bridging {
    try VZVirtioBlockDeviceConfiguration.validateBlockDeviceIdentifier(block_device_identifier.toString())
  }
}

extension VZVirtioBlockDeviceConfiguration {
  func vz_block_device_identifier() -> RustString {
    blockDeviceIdentifier.intoRustString()
  }

  func vz_set_block_device_identifier(block_device_identifier: RustStr) {
    blockDeviceIdentifier = block_device_identifier.toString()
  }
}
