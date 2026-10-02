use super::{BridgedThrown, GUARDED, NSURL, Never, Retained};

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

impl VZDiskImageStorageDeviceAttachment {
  pub(crate) fn bridge_retained(&self) -> VZDiskImageStorageDeviceAttachment {
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
