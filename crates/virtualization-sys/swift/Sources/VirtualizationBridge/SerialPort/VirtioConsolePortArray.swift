//===----------------------------------------------------------------------===//
// `VZVirtioConsolePortArray`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZVirtioConsolePortArray {
  func vz_maximum_port_count() -> UInt32 {
    maximumPortCount
  }

  func vz_subscript(port_index: Int) throws(BridgedNil) -> VZVirtioConsolePort {
    try bridging(self[port_index])
  }
}
