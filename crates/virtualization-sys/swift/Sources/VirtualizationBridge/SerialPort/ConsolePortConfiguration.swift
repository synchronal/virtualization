//===----------------------------------------------------------------------===//
// `VZConsolePortConfiguration`, abstract: its object crosses by address.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_console_port_configuration_attachment(this: UnsafeRawPointer) -> UInt {
  bridging(retainedAddress: (bridging(borrowed: this) as VZConsolePortConfiguration).attachment)
}

/// `attachment` is any `VZSerialPortAttachment`, or `0` for `nil`.
func vz_console_port_configuration_set_attachment(this: UnsafeRawPointer, attachment: UInt) {
  (bridging(borrowed: this) as VZConsolePortConfiguration).attachment = bridging(borrowedAddress: attachment)
}
