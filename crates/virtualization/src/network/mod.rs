mod network_device;

pub use self::network_device::NetworkDevice;
use crate::sealed::Sealed;

/// [`VZNetworkDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration)
pub trait NetworkDeviceConfiguration: Sealed {}

/// [`VZNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceattachment)
pub trait NetworkDeviceAttachment: Sealed {}
