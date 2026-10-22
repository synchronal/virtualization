//===----------------------------------------------------------------------===//
// `VZGuestMemoryMapping`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZGuestMemoryMapping {
  func vz_mutable_bytes() -> UnsafeMutableRawPointer {
    mutableBytes
  }

  func vz_physical_address() -> UInt64 {
    physicalAddress
  }

  func vz_length() -> Int {
    length
  }
}
