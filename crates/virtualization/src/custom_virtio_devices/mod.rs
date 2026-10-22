mod custom_virtio_device;
mod custom_virtio_device_configuration;
mod custom_virtio_device_delegate_provider;
mod negotiated_virtio_feature_set;
mod virtio_device_specific_configuration;
mod virtio_feature_set;
mod virtio_queue;
mod virtio_queue_element;
mod virtio_shared_memory_region;
mod virtio_shared_memory_region_configuration;

pub use self::custom_virtio_device::{CustomVirtioDevice, CustomVirtioDeviceDelegate};
pub use self::custom_virtio_device_configuration::CustomVirtioDeviceConfiguration;
pub use self::custom_virtio_device_delegate_provider::{
  CustomVirtioDeviceConfigurationDelegate, CustomVirtioDeviceDelegateProvider,
};
pub use self::negotiated_virtio_feature_set::NegotiatedVirtioFeatureSet;
pub use self::virtio_device_specific_configuration::VirtioDeviceSpecificConfiguration;
pub use self::virtio_feature_set::VirtioFeatureSet;
pub use self::virtio_queue::VirtioQueue;
pub use self::virtio_queue_element::VirtioQueueElement;
pub use self::virtio_shared_memory_region::VirtioSharedMemoryRegion;
pub use self::virtio_shared_memory_region_configuration::VirtioSharedMemoryRegionConfiguration;
use crate::sealed::Sealed;

/// [`VZCustomVirtioDeviceProvider`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceprovider)
pub trait CustomVirtioDeviceProvider: Sealed {}
