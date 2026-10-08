mod virtio_entropy_device_configuration;
mod virtio_traditional_memory_balloon_device;
mod virtio_traditional_memory_balloon_device_configuration;

pub use self::virtio_entropy_device_configuration::VirtioEntropyDeviceConfiguration;
pub use self::virtio_traditional_memory_balloon_device::VirtioTraditionalMemoryBalloonDevice;
pub use self::virtio_traditional_memory_balloon_device_configuration::VirtioTraditionalMemoryBalloonDeviceConfiguration;
use crate::sealed::Sealed;

/// [`VZEntropyDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzentropydeviceconfiguration)
pub trait EntropyDeviceConfiguration: Sealed {}

/// [`VZMemoryBalloonDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzmemoryballoondeviceconfiguration)
pub trait MemoryBalloonDeviceConfiguration: Sealed {}

/// [`VZUSBControllerConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbcontrollerconfiguration)
pub trait USBControllerConfiguration: Sealed {}

/// [`VZMemoryBalloonDevice`](https://developer.apple.com/documentation/virtualization/vzmemoryballoondevice)
pub trait MemoryBalloonDevice: Sealed {}

/// [`VZUSBController`](https://developer.apple.com/documentation/virtualization/vzusbcontroller)
pub trait USBController: Sealed {}
