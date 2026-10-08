//===----------------------------------------------------------------------===//
// `VZVirtioConsolePortConfigurationArray`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZVirtioConsolePortConfigurationArray {
  func vz_maximum_port_count() -> UInt32 {
    maximumPortCount
  }

  func vz_set_maximum_port_count(maximum_port_count: UInt32) {
    maximumPortCount = maximum_port_count
  }

  func vz_subscript(port_index: Int) throws(BridgedNil) -> VZVirtioConsolePortConfiguration {
    try bridging(self[port_index])
  }

  func vz_set_subscript(port_index: Int, value: VZVirtioConsolePortConfiguration) {
    self[port_index] = value
  }

  func bridge_set_subscript_nil(port_index: Int) {
    self[port_index] = nil
  }
}
