use super::{BridgedThrown, GUARDED, NSObject, NSURL, Never, Retained};

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

pub(crate) fn vz_storage_device_configuration_attachment(this: &NSObject) -> Retained<NSObject> {
  this.never()
}

pub(crate) struct VZVirtioBlockDeviceConfiguration {
  never: Never,
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
