//===----------------------------------------------------------------------===//
// `VZEFIVariableStore`. `InitializationOptions` crosses as its raw value.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_efi_variable_store_init(url: UnsafeRawPointer) -> VZEFIVariableStore {
  VZEFIVariableStore(url: bridging(borrowed: url) as NSURL as URL)
}

func vz_efi_variable_store_init_creating_variable_store_at_options(url: UnsafeRawPointer, options: UInt)
  throws(BridgedThrown) -> VZEFIVariableStore
{
  try bridging {
    try VZEFIVariableStore(
      creatingVariableStoreAt: bridging(borrowed: url) as NSURL as URL,
      options: VZEFIVariableStore.InitializationOptions(rawValue: options))
  }
}

func vz_efi_variable_store_url(this: VZEFIVariableStore) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

func vz_efi_variable_store_enable_secure_boot(this: VZEFIVariableStore, platform_key: UnsafeRawPointer)
  throws(BridgedThrown)
{
  try bridging {
    try this.enableSecureBoot(platformKey: bridging(borrowed: platform_key))
  }
}

extension VZEFIVariableStore {
  func vz_enable_secure_boot_using_default_platform_key() throws(BridgedThrown) {
    try bridging {
      try enableSecureBootUsingDefaultPlatformKey()
    }
  }

  func vz_disable_secure_boot() throws(BridgedThrown) {
    try bridging {
      try disableSecureBoot()
    }
  }

  func vz_reset_secure_boot() throws(BridgedThrown) {
    try bridging {
      try resetSecureBoot()
    }
  }

  func vz_is_secure_boot_enabled() throws(BridgedThrown) -> Bool {
    try bridging {
      try isSecureBootEnabled
    }
  }

  func vz_enroll_default_secure_boot_signatures() throws(BridgedThrown) {
    try bridging {
      try enrollDefaultSecureBootSignatures()
    }
  }

  func vz_enroll_secure_boot_signatures(signatures: VZEFISignatureDatabaseConfiguration) throws(BridgedThrown) {
    try bridging {
      try enrollSecureBootSignatures(signatures)
    }
  }

  func vz_enrolled_secure_boot_signatures() throws(BridgedThrown) -> VZEFISignatureDatabaseConfiguration {
    try bridging {
      try enrolledSecureBootSignatures
    }
  }
}
