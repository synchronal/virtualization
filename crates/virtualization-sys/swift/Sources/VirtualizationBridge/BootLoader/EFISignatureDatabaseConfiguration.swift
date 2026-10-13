//===----------------------------------------------------------------------===//
// `VZEFISignatureDatabaseConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_efi_signature_database_configuration_init(
  key_exchange_keys: UnsafeRawPointer, db_signatures: UnsafeRawPointer, dbx_signatures: UnsafeRawPointer
) -> VZEFISignatureDatabaseConfiguration {
  VZEFISignatureDatabaseConfiguration(
    keyExchangeKeys: bridging(borrowedArray: key_exchange_keys),
    dbSignatures: bridging(borrowedArray: db_signatures),
    dbxSignatures: bridging(borrowedArray: dbx_signatures))
}

func vz_efi_signature_database_configuration_key_exchange_keys(this: VZEFISignatureDatabaseConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.keyExchangeKeys as NSArray)
}

func vz_efi_signature_database_configuration_db_signatures(this: VZEFISignatureDatabaseConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.dbSignatures as NSArray)
}

func vz_efi_signature_database_configuration_dbx_signatures(this: VZEFISignatureDatabaseConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.dbxSignatures as NSArray)
}
