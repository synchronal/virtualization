mod linux_boot_loader;

pub use self::linux_boot_loader::LinuxBootLoader;
use crate::sealed::Sealed;

/// [`VZBootLoader`](https://developer.apple.com/documentation/virtualization/vzbootloader)
pub trait BootLoader: Sealed {}
