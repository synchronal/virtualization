//===----------------------------------------------------------------------===//
// `VZVirtioFileSystemDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_file_system_device_configuration_init(tag: RustStr) -> VZVirtioFileSystemDeviceConfiguration {
  VZVirtioFileSystemDeviceConfiguration(tag: tag.toString())
}

/// `object as? VZVirtioFileSystemDeviceConfiguration`, for an abstract
/// class's object.
func vz_virtio_file_system_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioFileSystemDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioFileSystemDeviceConfiguration)
}

func vz_virtio_file_system_device_configuration_validate_tag(tag: RustStr) throws(BridgedThrown) {
  try bridging {
    try VZVirtioFileSystemDeviceConfiguration.validateTag(tag.toString())
  }
}

func vz_virtio_file_system_device_configuration_macos_guest_automount_tag() -> RustString {
  VZVirtioFileSystemDeviceConfiguration.macOSGuestAutomountTag.intoRustString()
}

func vz_virtio_file_system_device_configuration_share(this: VZVirtioFileSystemDeviceConfiguration) -> UInt {
  bridging(retainedAddress: this.share)
}

/// `share` is any `VZDirectoryShare`, or `0` for `nil`.
func vz_virtio_file_system_device_configuration_set_share(this: VZVirtioFileSystemDeviceConfiguration, share: UInt) {
  this.share = bridging(borrowedAddress: share)
}

extension VZVirtioFileSystemDeviceConfiguration {
  func vz_tag() -> RustString {
    tag.intoRustString()
  }

  func vz_set_tag(tag: RustStr) {
    self.tag = tag.toString()
  }
}
