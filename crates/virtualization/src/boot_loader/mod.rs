mod generic_machine_identifier;
mod generic_platform_configuration;
mod linux_boot_loader;

pub use self::generic_machine_identifier::GenericMachineIdentifier;
pub use self::generic_platform_configuration::GenericPlatformConfiguration;
pub use self::linux_boot_loader::LinuxBootLoader;
use crate::sealed::Sealed;

/// [`VZBootLoader`](https://developer.apple.com/documentation/virtualization/vzbootloader)
pub trait BootLoader: Sealed {}

/// [`VZPlatformConfiguration`](https://developer.apple.com/documentation/virtualization/vzplatformconfiguration)
pub trait PlatformConfiguration: Sealed {}
