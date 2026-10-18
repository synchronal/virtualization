//===----------------------------------------------------------------------===//
// `VZMacOSVirtualMachineStartOptions`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_macos_virtual_machine_start_options_init() -> VZMacOSVirtualMachineStartOptions {
  VZMacOSVirtualMachineStartOptions()
}

/// `object as? VZMacOSVirtualMachineStartOptions`, for an abstract class's
/// object.
func vz_macos_virtual_machine_start_options_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacOSVirtualMachineStartOptions
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacOSVirtualMachineStartOptions)
}

extension VZMacOSVirtualMachineStartOptions {
  func vz_start_up_from_macos_recovery() -> Bool {
    startUpFromMacOSRecovery
  }

  func vz_set_start_up_from_macos_recovery(start_up_from_macos_recovery: Bool) {
    startUpFromMacOSRecovery = start_up_from_macos_recovery
  }

  func vz_guest_provisioning_options() throws(BridgedNil) -> VZMacGuestProvisioningOptions {
    try bridging(guestProvisioningOptions)
  }

  func vz_set_guest_provisioning(guest_provisioning_options: VZMacGuestProvisioningOptions) throws(BridgedThrown) {
    try bridging {
      try setGuestProvisioning(guest_provisioning_options)
    }
  }

  func bridge_set_guest_provisioning_nil() throws(BridgedThrown) {
    try bridging {
      try setGuestProvisioning(nil)
    }
  }
}
