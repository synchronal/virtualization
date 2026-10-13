//===----------------------------------------------------------------------===//
// `VZVirtioSocketListener` and `VZVirtioSocketListenerDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_socket_listener_init() -> VZVirtioSocketListener {
  VZVirtioSocketListener()
}

/// Rust's delegate, which `vz_virtio_socket_listener_set_delegate` set, by the
/// address Rust takes it from: `0` for none.
func vz_virtio_socket_listener_delegate(this: VZVirtioSocketListener) -> UInt {
  (this.delegate as? BridgedVirtioSocketListenerDelegate)?.delegate.bridge_retained() ?? 0
}

/// `delegate` is weak, so the listener keeps the
/// `BridgedVirtioSocketListenerDelegate` it is given. Rust's delegate is weak
/// in turn.
func vz_virtio_socket_listener_set_delegate(
  this: VZVirtioSocketListener,
  delegate: VZVirtioSocketListenerDelegateBridge
) {
  let delegate = BridgedVirtioSocketListenerDelegate(delegate)

  objc_setAssociatedObject(this, &delegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
  this.delegate = delegate
}

extension VZVirtioSocketListener {
  func bridge_set_delegate_nil() {
    objc_setAssociatedObject(self, &delegateKey, nil, .OBJC_ASSOCIATION_RETAIN)
    delegate = nil
  }
}

/// Where a listener keeps its `BridgedVirtioSocketListenerDelegate`. Only its
/// address is used.
nonisolated(unsafe) private var delegateKey: UInt8 = 0

/// A `VZVirtioSocketListenerDelegate` that forwards to Rust's.
final class BridgedVirtioSocketListenerDelegate: NSObject, VZVirtioSocketListenerDelegate {
  let delegate: VZVirtioSocketListenerDelegateBridge

  init(_ delegate: VZVirtioSocketListenerDelegateBridge) {
    self.delegate = delegate
  }

  func listener(
    _ listener: VZVirtioSocketListener,
    shouldAcceptNewConnection connection: VZVirtioSocketConnection,
    from socketDevice: VZVirtioSocketDevice
  ) -> Bool {
    delegate.listener_should_accept_new_connection_from(listener, connection, socketDevice)
  }
}
