use crate::sealed::Sealed;

/// [`VZDirectorySharingDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzdirectorysharingdeviceconfiguration)
pub trait DirectorySharingDeviceConfiguration: Sealed {}

/// [`VZDirectorySharingDevice`](https://developer.apple.com/documentation/virtualization/vzdirectorysharingdevice)
pub trait DirectorySharingDevice: Sealed {}
