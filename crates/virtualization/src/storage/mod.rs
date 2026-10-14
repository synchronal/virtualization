mod disk_block_device_storage_device_attachment;
mod disk_image_storage_device_attachment;
mod network_block_device_storage_device_attachment;
mod nvm_express_controller_device_configuration;
mod usb_mass_storage_device;
mod usb_mass_storage_device_configuration;
mod virtio_block_device_configuration;

pub use self::disk_block_device_storage_device_attachment::{
  DiskBlockDeviceStorageDeviceAttachment, DiskSynchronizationMode,
};
pub use self::disk_image_storage_device_attachment::{
  DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode,
};
pub use self::network_block_device_storage_device_attachment::{
  NetworkBlockDeviceStorageDeviceAttachment, NetworkBlockDeviceStorageDeviceAttachmentDelegate,
};
pub use self::nvm_express_controller_device_configuration::NVMExpressControllerDeviceConfiguration;
pub use self::usb_mass_storage_device::USBMassStorageDevice;
pub use self::usb_mass_storage_device_configuration::USBMassStorageDeviceConfiguration;
pub use self::virtio_block_device_configuration::VirtioBlockDeviceConfiguration;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZStorageDevice`](https://developer.apple.com/documentation/virtualization/vzstoragedevice)
pub trait StorageDevice: Sealed {}

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
