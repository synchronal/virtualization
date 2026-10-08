//===----------------------------------------------------------------------===//
// `VZVirtioFileSystemDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioFileSystemDevice`, for an abstract class's object.
func vz_virtio_file_system_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioFileSystemDevice
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioFileSystemDevice)
}

func vz_virtio_file_system_device_share(this: VZVirtioFileSystemDevice) -> UInt {
  bridging(retainedAddress: this.share)
}

/// `share` is any `VZDirectoryShare`, or `0` for `nil`.
func vz_virtio_file_system_device_set_share(this: VZVirtioFileSystemDevice, share: UInt) {
  this.share = bridging(borrowedAddress: share)
}

extension VZVirtioFileSystemDevice {
  func vz_tag() -> RustString {
    tag.intoRustString()
  }
}
