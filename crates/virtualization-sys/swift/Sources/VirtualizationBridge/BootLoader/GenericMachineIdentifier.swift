//===----------------------------------------------------------------------===//
// `VZGenericMachineIdentifier`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_generic_machine_identifier_init() -> VZGenericMachineIdentifier {
  VZGenericMachineIdentifier()
}

func vz_generic_machine_identifier_init_data_representation(data_representation: UnsafeRawPointer) throws(BridgedNil)
  -> VZGenericMachineIdentifier
{
  try bridging(
    VZGenericMachineIdentifier(dataRepresentation: bridging(borrowed: data_representation) as NSData as Data))
}

func vz_generic_machine_identifier_data_representation(this: VZGenericMachineIdentifier) -> UnsafeMutableRawPointer {
  bridging(retained: this.dataRepresentation as NSData)
}

extension VZGenericMachineIdentifier {
  func ns_is_equal(object: VZGenericMachineIdentifier) -> Bool {
    isEqual(object)
  }
}
