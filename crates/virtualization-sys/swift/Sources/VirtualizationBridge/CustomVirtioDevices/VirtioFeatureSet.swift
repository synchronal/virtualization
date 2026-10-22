//===----------------------------------------------------------------------===//
// `VZVirtioFeatureSet`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZVirtioFeatureSet {
  func vz_subset0() -> UInt32 {
    subset0
  }

  func vz_set_subset0(subset0: UInt32) {
    self.subset0 = subset0
  }

  func vz_subset1() -> UInt32 {
    subset1
  }

  func vz_set_subset1(subset1: UInt32) {
    self.subset1 = subset1
  }
}
