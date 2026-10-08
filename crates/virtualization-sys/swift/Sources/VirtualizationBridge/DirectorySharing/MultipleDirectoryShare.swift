//===----------------------------------------------------------------------===//
// `VZMultipleDirectoryShare`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_multiple_directory_share_init() -> VZMultipleDirectoryShare {
  VZMultipleDirectoryShare()
}

/// `directories` is an `NSDictionary` of names to `VZSharedDirectory`s.
func vz_multiple_directory_share_init_directories(directories: UnsafeRawPointer) -> VZMultipleDirectoryShare {
  VZMultipleDirectoryShare(directories: bridging(borrowedDictionary: directories))
}

/// `object as? VZMultipleDirectoryShare`, for an abstract class's object.
func vz_multiple_directory_share_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMultipleDirectoryShare
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMultipleDirectoryShare)
}

func vz_multiple_directory_share_validate_name(name: RustStr) throws(BridgedThrown) {
  try bridging {
    try VZMultipleDirectoryShare.validateName(name.toString())
  }
}

func vz_multiple_directory_share_canonicalized_name(name: RustStr) -> RustString? {
  VZMultipleDirectoryShare.canonicalizedName(from: name.toString())?.intoRustString()
}

func vz_multiple_directory_share_directories(this: VZMultipleDirectoryShare) -> UnsafeMutableRawPointer {
  bridging(retained: this.directories as NSDictionary)
}
