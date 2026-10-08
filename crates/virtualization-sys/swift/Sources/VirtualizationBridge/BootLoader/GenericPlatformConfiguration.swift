//===----------------------------------------------------------------------===//
// `VZGenericPlatformConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_generic_platform_configuration_init() -> VZGenericPlatformConfiguration {
  VZGenericPlatformConfiguration()
}

/// `object as? VZGenericPlatformConfiguration`, for an abstract class's
/// object.
func vz_generic_platform_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZGenericPlatformConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZGenericPlatformConfiguration)
}

func vz_generic_platform_configuration_is_nested_virtualization_supported() -> Bool {
  VZGenericPlatformConfiguration.isNestedVirtualizationSupported
}

extension VZGenericPlatformConfiguration {
  func vz_machine_identifier() -> VZGenericMachineIdentifier {
    machineIdentifier
  }

  func vz_set_machine_identifier(machine_identifier: VZGenericMachineIdentifier) {
    machineIdentifier = machine_identifier
  }

  func vz_is_nested_virtualization_enabled() -> Bool {
    isNestedVirtualizationEnabled
  }

  func vz_set_nested_virtualization_enabled(nested_virtualization_enabled: Bool) {
    isNestedVirtualizationEnabled = nested_virtualization_enabled
  }
}
