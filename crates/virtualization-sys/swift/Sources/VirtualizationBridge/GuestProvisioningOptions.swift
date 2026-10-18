//===----------------------------------------------------------------------===//
// `VZGuestProvisioningOptions`, an abstract class: its members serve every
// subclass, whose object Rust lends.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_guest_provisioning_options_validate(this: UnsafeRawPointer) throws(BridgedThrown) {
  try bridging {
    try (bridging(borrowed: this) as VZGuestProvisioningOptions).validate()
  }
}
