//===----------------------------------------------------------------------===//
// `VZEFIBootLoader`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_efi_boot_loader_init() -> VZEFIBootLoader {
  VZEFIBootLoader()
}

/// `object as? VZEFIBootLoader`, for an abstract class's object.
func vz_efi_boot_loader_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZEFIBootLoader {
  try bridging(bridging(borrowed: object) as NSObject as? VZEFIBootLoader)
}

extension VZEFIBootLoader {
  func vz_variable_store() throws(BridgedNil) -> VZEFIVariableStore {
    try bridging(variableStore)
  }

  func vz_set_variable_store(variable_store: VZEFIVariableStore) {
    variableStore = variable_store
  }

  func bridge_set_variable_store_nil() {
    variableStore = nil
  }
}
