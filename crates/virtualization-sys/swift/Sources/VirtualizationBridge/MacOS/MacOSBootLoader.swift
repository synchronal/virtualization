//===----------------------------------------------------------------------===//
// `VZMacOSBootLoader`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_macos_boot_loader_init() -> VZMacOSBootLoader {
  VZMacOSBootLoader()
}

/// `object as? VZMacOSBootLoader`, for an abstract class's object.
func vz_macos_boot_loader_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZMacOSBootLoader {
  try bridging(bridging(borrowed: object) as NSObject as? VZMacOSBootLoader)
}
