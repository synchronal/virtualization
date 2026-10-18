//===----------------------------------------------------------------------===//
// `VZMacMachineIdentifier`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_machine_identifier_init() -> VZMacMachineIdentifier {
  VZMacMachineIdentifier()
}

func vz_mac_machine_identifier_init_data_representation(data_representation: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacMachineIdentifier
{
  try bridging(VZMacMachineIdentifier(dataRepresentation: bridging(borrowed: data_representation) as NSData as Data))
}

func vz_mac_machine_identifier_data_representation(this: VZMacMachineIdentifier) -> UnsafeMutableRawPointer {
  bridging(retained: this.dataRepresentation as NSData)
}

extension VZMacMachineIdentifier {
  func ns_is_equal(object: VZMacMachineIdentifier) -> Bool {
    isEqual(object)
  }
}
