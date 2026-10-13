mod efi_boot_loader;
mod efi_signature_database_configuration;
mod efi_signature_list;
mod efi_variable_store;
mod generic_machine_identifier;
mod generic_platform_configuration;
mod linux_boot_loader;

pub use self::efi_boot_loader::EFIBootLoader;
pub use self::efi_signature_database_configuration::EFISignatureDatabaseConfiguration;
pub use self::efi_signature_list::{EFISignature, EFISignatureList};
pub use self::efi_variable_store::{EFIVariableStore, EFIVariableStoreInitializationOptions};
pub use self::generic_machine_identifier::GenericMachineIdentifier;
pub use self::generic_platform_configuration::GenericPlatformConfiguration;
pub use self::linux_boot_loader::LinuxBootLoader;
use crate::sealed::Sealed;

/// [`VZBootLoader`](https://developer.apple.com/documentation/virtualization/vzbootloader)
pub trait BootLoader: Sealed {}

/// [`VZPlatformConfiguration`](https://developer.apple.com/documentation/virtualization/vzplatformconfiguration)
pub trait PlatformConfiguration: Sealed {}
