use std::any::Any;
use std::os::fd::{BorrowedFd, OwnedFd};
use std::sync::Arc;

use super::{BridgedThrown, GUARDED, Local, NSObject, NSURL, Never, Retained};

pub(crate) enum VZDiskSynchronizationMode {
  Full,
  None,
  Unknown(isize),
}

pub(crate) struct VZDiskBlockDeviceStorageDeviceAttachment {
  never: Never,
  local: Local,
}

pub(crate) fn vz_disk_block_device_storage_device_attachment_init(
  _file_handle: OwnedFd,
  _read_only: bool,
  _synchronization_mode: VZDiskSynchronizationMode,
) -> Result<VZDiskBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_disk_block_device_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZDiskBlockDeviceStorageDeviceAttachment> {
  object.never()
}

impl VZDiskBlockDeviceStorageDeviceAttachment {
  pub(crate) fn bridge_retained(&self) -> VZDiskBlockDeviceStorageDeviceAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_file_handle(&self) -> BorrowedFd<'_> {
    match self.never {}
  }

  pub(crate) fn vz_is_read_only(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_synchronization_mode(&self) -> VZDiskSynchronizationMode {
    match self.never {}
  }
}

pub(crate) enum VZDiskImageCachingMode {
  Automatic,
  Uncached,
  Cached,
  Unknown(isize),
}

pub(crate) enum VZDiskImageSynchronizationMode {
  Full,
  Fsync,
  None,
  Unknown(isize),
}

pub(crate) struct VZDiskImageStorageDeviceAttachment {
  never: Never,
  local: Local,
}

pub(crate) fn vz_disk_image_storage_device_attachment_init(
  _url: &NSURL,
  _read_only: bool,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
  _url: &NSURL,
  _read_only: bool,
  _caching_mode: VZDiskImageCachingMode,
  _synchronization_mode: VZDiskImageSynchronizationMode,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_disk_image_storage_device_attachment_init_disk_image_caching_mode_synchronization_mode(
  _disk_image: &disk_image_kit::DiskImage,
  _caching_mode: VZDiskImageCachingMode,
  _synchronization_mode: VZDiskImageSynchronizationMode,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_disk_image_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZDiskImageStorageDeviceAttachment> {
  object.never()
}

impl VZDiskImageStorageDeviceAttachment {
  pub(crate) fn bridge_retained(&self) -> VZDiskImageStorageDeviceAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_is_read_only(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_caching_mode(&self) -> VZDiskImageCachingMode {
    match self.never {}
  }

  pub(crate) fn vz_synchronization_mode(&self) -> VZDiskImageSynchronizationMode {
    match self.never {}
  }
}

pub(crate) struct VZNVMExpressControllerDeviceConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_nvm_express_controller_device_configuration_init(
  _attachment: &NSObject,
) -> VZNVMExpressControllerDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_nvm_express_controller_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZNVMExpressControllerDeviceConfiguration> {
  object.never()
}

impl VZNVMExpressControllerDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZNVMExpressControllerDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZNetworkBlockDeviceStorageDeviceAttachment {
  never: Never,
  local: Local,
}

pub(crate) fn vz_network_block_device_storage_device_attachment_init(
  _url: &NSURL,
) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
  _url: &NSURL,
  _timeout: f64,
  _is_forced_read_only: bool,
  _synchronization_mode: VZDiskSynchronizationMode,
) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_network_block_device_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZNetworkBlockDeviceStorageDeviceAttachment> {
  object.never()
}

pub(crate) fn vz_network_block_device_storage_device_attachment_validate(_url: &NSURL) -> Result<(), BridgedThrown> {
  unreachable!("{GUARDED}")
}

impl VZNetworkBlockDeviceStorageDeviceAttachment {
  pub(crate) fn bridge_retained(&self) -> VZNetworkBlockDeviceStorageDeviceAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_timeout(&self) -> f64 {
    match self.never {}
  }

  pub(crate) fn vz_is_forced_read_only(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_synchronization_mode(&self) -> VZDiskSynchronizationMode {
    match self.never {}
  }

  pub(crate) fn vz_delegate(&self) -> Option<Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>> {
    match self.never {}
  }

  pub(crate) fn vz_set_delegate(
    &self,
    _delegate: Option<Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>>,
  ) {
    match self.never {}
  }
}

pub(crate) trait VZNetworkBlockDeviceStorageDeviceAttachmentDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZNetworkBlockDeviceStorageDeviceAttachmentDelegate for T {}

pub(crate) fn vz_storage_device_configuration_attachment(this: &NSObject) -> Retained<NSObject> {
  this.never()
}

pub(crate) struct VZUSBMassStorageDevice {
  never: Never,
  local: Local,
}

pub(crate) fn vz_usb_mass_storage_device_init(
  _configuration: &VZUSBMassStorageDeviceConfiguration,
) -> VZUSBMassStorageDevice {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_mass_storage_device_bridge_cast(object: &NSObject) -> Option<VZUSBMassStorageDevice> {
  object.never()
}

impl VZUSBMassStorageDevice {
  pub(crate) fn bridge_retained(&self) -> VZUSBMassStorageDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZUSBMassStorageDeviceConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_usb_mass_storage_device_configuration_init(
  _attachment: &NSObject,
) -> VZUSBMassStorageDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_mass_storage_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZUSBMassStorageDeviceConfiguration> {
  object.never()
}

impl VZUSBMassStorageDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZUSBMassStorageDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioBlockDeviceConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_virtio_block_device_configuration_init(_attachment: &NSObject) -> VZVirtioBlockDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_block_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioBlockDeviceConfiguration> {
  object.never()
}

pub(crate) fn vz_virtio_block_device_configuration_validate_block_device_identifier(
  _block_device_identifier: &str,
) -> Result<(), BridgedThrown> {
  unreachable!("{GUARDED}")
}

impl VZVirtioBlockDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioBlockDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_block_device_identifier(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_block_device_identifier(&self, _block_device_identifier: &str) {
    match self.never {}
  }
}
