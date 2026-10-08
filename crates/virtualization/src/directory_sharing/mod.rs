mod multiple_directory_share;
mod shared_directory;
mod single_directory_share;
mod virtio_file_system_device;
mod virtio_file_system_device_configuration;

pub use self::multiple_directory_share::MultipleDirectoryShare;
pub use self::shared_directory::SharedDirectory;
pub use self::single_directory_share::SingleDirectoryShare;
pub use self::virtio_file_system_device::VirtioFileSystemDevice;
pub use self::virtio_file_system_device_configuration::VirtioFileSystemDeviceConfiguration;
use crate::sealed::Sealed;

/// [`VZDirectoryShare`](https://developer.apple.com/documentation/virtualization/vzdirectoryshare)
pub trait DirectoryShare: Sealed {}

/// [`VZDirectorySharingDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzdirectorysharingdeviceconfiguration)
pub trait DirectorySharingDeviceConfiguration: Sealed {}

/// [`VZDirectorySharingDevice`](https://developer.apple.com/documentation/virtualization/vzdirectorysharingdevice)
pub trait DirectorySharingDevice: Sealed {}
