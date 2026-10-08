//===----------------------------------------------------------------------===//
// `VZSingleDirectoryShare`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_single_directory_share_init(directory: VZSharedDirectory) -> VZSingleDirectoryShare {
  VZSingleDirectoryShare(directory: directory)
}

/// `object as? VZSingleDirectoryShare`, for an abstract class's object.
func vz_single_directory_share_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZSingleDirectoryShare {
  try bridging(bridging(borrowed: object) as NSObject as? VZSingleDirectoryShare)
}

extension VZSingleDirectoryShare {
  func vz_directory() -> VZSharedDirectory {
    directory
  }
}
