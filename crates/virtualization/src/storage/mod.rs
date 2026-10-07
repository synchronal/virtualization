mod disk_image_storage_device_attachment;
mod virtio_block_device_configuration;

pub use self::disk_image_storage_device_attachment::{
  DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode,
};
pub use self::virtio_block_device_configuration::VirtioBlockDeviceConfiguration;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZStorageDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzstoragedeviceattachment)
pub trait StorageDeviceAttachment: Sealed {}

/// [`VZStorageDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzstoragedeviceconfiguration)
pub trait StorageDeviceConfiguration: Sealed {
  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzstoragedeviceconfiguration/attachment),
  /// as Swift's `as? T`: `None` when the attachment isn't a `T`.
  fn attachment<T: StorageDeviceAttachment>(&self) -> Option<T>
  where
    Self: Sized,
  {
    T::bridge_cast(&ffi::vz_storage_device_configuration_attachment(self.bridge_object()))
  }
}
