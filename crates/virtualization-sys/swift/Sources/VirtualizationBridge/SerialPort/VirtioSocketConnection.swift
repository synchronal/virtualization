//===----------------------------------------------------------------------===//
// `VZVirtioSocketConnection`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// What a completion handler was called with, which Rust holds.
func vz_virtio_socket_connection_bridge_from(object: UnsafeRawPointer) -> VZVirtioSocketConnection {
  bridging(borrowed: object)
}

extension VZVirtioSocketConnection {
  func vz_destination_port() -> UInt32 {
    destinationPort
  }

  func vz_source_port() -> UInt32 {
    sourcePort
  }

  func vz_file_descriptor() -> Int32 {
    fileDescriptor
  }

  func vz_close() {
    close()
  }
}
