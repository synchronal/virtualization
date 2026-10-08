//===----------------------------------------------------------------------===//
// `VZVirtioConsolePort`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_console_port_attachment(this: VZVirtioConsolePort) -> UInt {
  bridging(retainedAddress: this.attachment)
}

/// `attachment` is any `VZSerialPortAttachment`, or `0` for `nil`.
func vz_virtio_console_port_set_attachment(this: VZVirtioConsolePort, attachment: UInt) {
  this.attachment = bridging(borrowedAddress: attachment)
}

extension VZVirtioConsolePort {
  func vz_name() -> RustString? {
    name?.intoRustString()
  }
}
