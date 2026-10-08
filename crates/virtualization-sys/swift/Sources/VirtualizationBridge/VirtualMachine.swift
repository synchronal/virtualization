//===----------------------------------------------------------------------===//
// `VZVirtualMachine`, `VZVirtualMachine.State`, and
// `VZVirtualMachineDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtual_machine_init(configuration: VZVirtualMachineConfiguration) -> VZVirtualMachine {
  VZVirtualMachine(configuration: configuration)
}

/// `queue` is a `dispatch_queue_t`, lent for the call.
func vz_virtual_machine_init_queue(configuration: VZVirtualMachineConfiguration, queue: UnsafeRawPointer)
  -> VZVirtualMachine
{
  VZVirtualMachine(configuration: configuration, queue: bridging(borrowed: queue))
}

func vz_virtual_machine_is_supported() -> Bool {
  VZVirtualMachine.isSupported
}

func vz_virtual_machine_queue(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.queue)
}

func vz_virtual_machine_console_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.consoleDevices as NSArray)
}

func vz_virtual_machine_directory_sharing_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.directorySharingDevices as NSArray)
}

func vz_virtual_machine_graphics_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.graphicsDevices as NSArray)
}

func vz_virtual_machine_memory_balloon_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.memoryBalloonDevices as NSArray)
}

func vz_virtual_machine_network_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.networkDevices as NSArray)
}

func vz_virtual_machine_socket_devices(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.socketDevices as NSArray)
}

func vz_virtual_machine_usb_controllers(this: VZVirtualMachine) -> UnsafeMutableRawPointer {
  bridging(retained: this.usbControllers as NSArray)
}

func vz_virtual_machine_start(this: VZVirtualMachine, completion_handler: @escaping (UInt) -> Void) {
  this.start { completion_handler(bridging(retainedError: $0)) }
}

/// `options` is any `VZVirtualMachineStartOptions`.
func vz_virtual_machine_start_options(
  this: VZVirtualMachine,
  options: UnsafeRawPointer,
  completion_handler: @escaping (UInt) -> Void
) {
  this.start(options: bridging(borrowed: options)) { completion_handler(bridging(retainedError: $0)) }
}

func vz_virtual_machine_stop(this: VZVirtualMachine, completion_handler: @escaping (UInt) -> Void) {
  this.stop { completion_handler(bridging(retainedError: $0)) }
}

func vz_virtual_machine_pause(this: VZVirtualMachine, completion_handler: @escaping (UInt) -> Void) {
  this.pause { completion_handler(bridging(retainedError: $0)) }
}

func vz_virtual_machine_resume(this: VZVirtualMachine, completion_handler: @escaping (UInt) -> Void) {
  this.resume { completion_handler(bridging(retainedError: $0)) }
}

func vz_virtual_machine_restore_machine_state_from(
  this: VZVirtualMachine,
  url: UnsafeRawPointer,
  completion_handler: @escaping (UInt) -> Void
) {
  this.restoreMachineStateFrom(url: bridging(borrowed: url) as NSURL as URL) {
    completion_handler(bridging(retainedError: $0))
  }
}

func vz_virtual_machine_save_machine_state_to(
  this: VZVirtualMachine,
  url: UnsafeRawPointer,
  completion_handler: @escaping (UInt) -> Void
) {
  this.saveMachineStateTo(url: bridging(borrowed: url) as NSURL as URL) {
    completion_handler(bridging(retainedError: $0))
  }
}

/// Rust's delegate, which `vz_virtual_machine_set_delegate` set, by the
/// address Rust takes it from: `0` for none.
func vz_virtual_machine_delegate(this: VZVirtualMachine) -> UInt {
  (this.delegate as? BridgedVirtualMachineDelegate)?.delegate.bridge_retained() ?? 0
}

/// `delegate` is weak, so the machine keeps the `BridgedVirtualMachineDelegate`
/// it is given. Rust's delegate is weak in turn.
func vz_virtual_machine_set_delegate(this: VZVirtualMachine, delegate: VZVirtualMachineDelegateBridge) {
  let delegate = BridgedVirtualMachineDelegate(delegate)

  objc_setAssociatedObject(this, &delegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
  this.delegate = delegate
}

extension VZVirtualMachine {
  func bridge_set_delegate_nil() {
    objc_setAssociatedObject(self, &delegateKey, nil, .OBJC_ASSOCIATION_RETAIN)
    delegate = nil
  }

  func vz_state() -> BridgedVZVirtualMachineState {
    BridgedVZVirtualMachineState(state)
  }

  func vz_can_start() -> Bool {
    canStart
  }

  func vz_can_stop() -> Bool {
    canStop
  }

  func vz_can_pause() -> Bool {
    canPause
  }

  func vz_can_resume() -> Bool {
    canResume
  }

  func vz_can_request_stop() -> Bool {
    canRequestStop
  }

  func vz_request_stop() throws(BridgedThrown) {
    try bridging { try requestStop() }
  }
}

/// Where a machine keeps its `BridgedVirtualMachineDelegate`. Only its address
/// is used.
nonisolated(unsafe) private var delegateKey: UInt8 = 0

/// A `VZVirtualMachineDelegate` that forwards to Rust's.
final class BridgedVirtualMachineDelegate: NSObject, VZVirtualMachineDelegate {
  let delegate: VZVirtualMachineDelegateBridge

  init(_ delegate: VZVirtualMachineDelegateBridge) {
    self.delegate = delegate
  }

  func guestDidStop(_ virtualMachine: VZVirtualMachine) {
    delegate.guest_did_stop(virtualMachine)
  }

  func virtualMachine(_ virtualMachine: VZVirtualMachine, didStopWithError error: any Error) {
    delegate.virtual_machine_did_stop_with_error(virtualMachine, bridging(retained: error as NSError))
  }

  func virtualMachine(
    _ virtualMachine: VZVirtualMachine,
    networkDevice: VZNetworkDevice,
    attachmentWasDisconnectedWithError error: any Error
  ) {
    delegate.virtual_machine_network_device_attachment_was_disconnected_with_error(
      virtualMachine,
      networkDevice,
      bridging(retained: error as NSError)
    )
  }
}

extension BridgedVZVirtualMachineState {
  init(_ state: VZVirtualMachine.State) {
    switch state {
    case .stopped: self = .Stopped
    case .running: self = .Running
    case .paused: self = .Paused
    case .error: self = .Error
    case .starting: self = .Starting
    case .pausing: self = .Pausing
    case .resuming: self = .Resuming
    case .stopping: self = .Stopping
    case .saving: self = .Saving
    case .restoring: self = .Restoring
    @unknown default: self = .Unknown(state.rawValue)
    }
  }
}
