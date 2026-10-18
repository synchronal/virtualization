//===----------------------------------------------------------------------===//
// `VZMacOSConfigurationRequirements`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZMacOSConfigurationRequirements {
  func vz_hardware_model() -> VZMacHardwareModel {
    hardwareModel
  }

  func vz_minimum_supported_cpu_count() -> Int {
    minimumSupportedCPUCount
  }

  func vz_minimum_supported_memory_size() -> UInt64 {
    minimumSupportedMemorySize
  }
}
