//===----------------------------------------------------------------------===//
// `VZVirtualMachine`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtual_machine_is_supported() -> Bool {
  VZVirtualMachine.isSupported
}
