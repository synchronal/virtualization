//===----------------------------------------------------------------------===//
// `VZVirtioSocketDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioSocketDevice`, for an abstract class's object.
func vz_virtio_socket_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZVirtioSocketDevice {
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioSocketDevice)
}

/// `completion_handler` takes the connection, which Rust takes over, then what
/// it failed with, each by address: one is `0`.
func vz_virtio_socket_device_connect(
  this: VZVirtioSocketDevice,
  port: UInt32,
  completion_handler: @escaping (UInt, UInt) -> Void
) {
  this.connect(toPort: port) { result in
    switch result {
    case .success(let connection): completion_handler(bridging(retainedAddress: connection), 0)
    case .failure(let error): completion_handler(0, bridging(retainedError: error))
    }
  }
}

extension VZVirtioSocketDevice {
  func vz_set_socket_listener(listener: VZVirtioSocketListener, port: UInt32) {
    setSocketListener(listener, forPort: port)
  }

  func vz_remove_socket_listener(port: UInt32) {
    removeSocketListener(forPort: port)
  }
}
