//===----------------------------------------------------------------------===//
// `VZMacAuxiliaryStorage`. `InitializationOptions` crosses as its raw value.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_auxiliary_storage_init(url: UnsafeRawPointer) -> VZMacAuxiliaryStorage {
  VZMacAuxiliaryStorage(url: bridging(borrowed: url) as NSURL as URL)
}

func vz_mac_auxiliary_storage_init_creating_storage_at_hardware_model_options(
  url: UnsafeRawPointer,
  hardware_model: VZMacHardwareModel,
  options: UInt
) throws(BridgedThrown) -> VZMacAuxiliaryStorage {
  try bridging {
    try VZMacAuxiliaryStorage(
      creatingStorageAt: bridging(borrowed: url) as NSURL as URL,
      hardwareModel: hardware_model,
      options: VZMacAuxiliaryStorage.InitializationOptions(rawValue: options))
  }
}

func vz_mac_auxiliary_storage_initialization_options_allow_overwrite() -> UInt {
  VZMacAuxiliaryStorage.InitializationOptions.allowOverwrite.rawValue
}

func vz_mac_auxiliary_storage_url(this: VZMacAuxiliaryStorage) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}
