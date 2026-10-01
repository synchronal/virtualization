mod disk_image_storage_device_attachment;

pub use self::disk_image_storage_device_attachment::{
  DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode,
};
use crate::sealed::Sealed;

/// [`VZStorageDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vzstoragedeviceattachment)
pub trait StorageDeviceAttachment: Sealed {}
