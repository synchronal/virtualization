//===----------------------------------------------------------------------===//
// `VZSharedDirectory`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_shared_directory_init(url: UnsafeRawPointer, read_only: Bool) -> VZSharedDirectory {
  VZSharedDirectory(url: bridging(borrowed: url) as NSURL as URL, readOnly: read_only)
}

/// A value of a dictionary of them, which Rust holds.
func vz_shared_directory_bridge_from(object: UnsafeRawPointer) -> VZSharedDirectory {
  bridging(borrowed: object)
}

func vz_shared_directory_url(this: VZSharedDirectory) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

extension VZSharedDirectory {
  func vz_is_read_only() -> Bool {
    isReadOnly
  }
}
