//===----------------------------------------------------------------------===//
// `VZMacHardwareModel`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_hardware_model_init_data_representation(data_representation: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacHardwareModel
{
  try bridging(VZMacHardwareModel(dataRepresentation: bridging(borrowed: data_representation) as NSData as Data))
}

func vz_mac_hardware_model_data_representation(this: VZMacHardwareModel) -> UnsafeMutableRawPointer {
  bridging(retained: this.dataRepresentation as NSData)
}

extension VZMacHardwareModel {
  func vz_is_supported() -> Bool {
    isSupported
  }
}
