//===----------------------------------------------------------------------===//
// `VZMacPlatformConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_platform_configuration_init() -> VZMacPlatformConfiguration {
  VZMacPlatformConfiguration()
}

/// `object as? VZMacPlatformConfiguration`, for an abstract class's object.
func vz_mac_platform_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacPlatformConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacPlatformConfiguration)
}

extension VZMacPlatformConfiguration {
  func vz_hardware_model() -> VZMacHardwareModel {
    hardwareModel
  }

  func vz_set_hardware_model(hardware_model: VZMacHardwareModel) {
    hardwareModel = hardware_model
  }

  func vz_machine_identifier() -> VZMacMachineIdentifier {
    machineIdentifier
  }

  func vz_set_machine_identifier(machine_identifier: VZMacMachineIdentifier) {
    machineIdentifier = machine_identifier
  }

  func vz_auxiliary_storage() throws(BridgedNil) -> VZMacAuxiliaryStorage {
    try bridging(auxiliaryStorage)
  }

  func vz_set_auxiliary_storage(auxiliary_storage: VZMacAuxiliaryStorage) {
    auxiliaryStorage = auxiliary_storage
  }

  func bridge_set_auxiliary_storage_nil() {
    auxiliaryStorage = nil
  }
}
