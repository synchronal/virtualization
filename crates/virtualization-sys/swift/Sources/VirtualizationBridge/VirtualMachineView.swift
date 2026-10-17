//===----------------------------------------------------------------------===//
// `VZVirtualMachineView` and `VZVirtualMachineViewAdaptor`.
//
// The view is AppKit's, so `@MainActor`, while swift-bridge's calls are
// nonisolated: each runs on the main actor with `MainActor.assumeIsolated`.
// Rust makes a view only with a `MainThreadMarker`, and it isn't `Send`, so
// every call is on the main thread.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AppKit
import Virtualization

func vz_virtual_machine_view_init() -> VZVirtualMachineView {
  MainActor.assumeIsolated {
    VZVirtualMachineView()
  }
}

extension VZVirtualMachineView {
  /// `virtualMachine` isn't `Sendable`, so leaves the main actor unchecked:
  /// Rust holds the view only on the main thread.
  nonisolated func vz_virtual_machine() throws(BridgedNil) -> VZVirtualMachine {
    nonisolated(unsafe) var virtualMachine: VZVirtualMachine?

    MainActor.assumeIsolated {
      virtualMachine = self.virtualMachine
    }

    return try bridging(virtualMachine)
  }

  nonisolated func vz_set_virtual_machine(virtual_machine: VZVirtualMachine) {
    nonisolated(unsafe) let virtualMachine = virtual_machine

    MainActor.assumeIsolated {
      self.virtualMachine = virtualMachine
    }
  }

  nonisolated func bridge_set_virtual_machine_nil() {
    MainActor.assumeIsolated {
      virtualMachine = nil
    }
  }

  nonisolated func vz_captures_system_keys() -> Bool {
    MainActor.assumeIsolated {
      capturesSystemKeys
    }
  }

  nonisolated func vz_set_captures_system_keys(captures_system_keys: Bool) {
    MainActor.assumeIsolated {
      capturesSystemKeys = captures_system_keys
    }
  }

  nonisolated func vz_automatically_reconfigures_display() -> Bool {
    MainActor.assumeIsolated {
      automaticallyReconfiguresDisplay
    }
  }

  nonisolated func vz_set_automatically_reconfigures_display(automatically_reconfigures_display: Bool) {
    MainActor.assumeIsolated {
      automaticallyReconfiguresDisplay = automatically_reconfigures_display
    }
  }

  nonisolated func vz_adaptor() throws(BridgedNil) -> BridgedVZVirtualMachineViewAdaptor {
    try bridging(
      MainActor.assumeIsolated {
        adaptor.map { BridgedVZVirtualMachineViewAdaptor($0) }
      }
    )
  }

  nonisolated func vz_set_adaptor(adaptor: BridgedVZVirtualMachineViewAdaptor) {
    MainActor.assumeIsolated {
      self.adaptor = adaptor.value
    }
  }

  nonisolated func bridge_set_adaptor_nil() {
    MainActor.assumeIsolated {
      adaptor = nil
    }
  }
}

/// `VZVirtualMachineViewAdaptor`, a struct, in a class swift-bridge can carry.
final class BridgedVZVirtualMachineViewAdaptor: Sendable {
  let value: VZVirtualMachineViewAdaptor

  init(_ value: VZVirtualMachineViewAdaptor) {
    self.value = value
  }

  func bridge_retained() -> BridgedVZVirtualMachineViewAdaptor {
    self
  }

  func bridge_description() -> RustString {
    String(describing: value).intoRustString()
  }
}

func vz_virtual_machine_view_adaptor_init_virtual_machine(
  virtual_machine: VZVirtualMachine
) -> BridgedVZVirtualMachineViewAdaptor {
  BridgedVZVirtualMachineViewAdaptor(VZVirtualMachineViewAdaptor(virtualMachine: virtual_machine))
}
