use super::{BridgedThrown, GUARDED, NSObject, NSURL, Never, Retained};

pub(crate) struct VZMultipleDirectoryShare {
  never: Never,
}

pub(crate) fn vz_multiple_directory_share_init() -> VZMultipleDirectoryShare {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_multiple_directory_share_init_directories(
  _directories: &[(&str, &VZSharedDirectory)],
) -> VZMultipleDirectoryShare {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_multiple_directory_share_bridge_cast(object: &NSObject) -> Option<VZMultipleDirectoryShare> {
  object.never()
}

pub(crate) fn vz_multiple_directory_share_validate_name(_name: &str) -> Result<(), BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_multiple_directory_share_canonicalized_name(_name: &str) -> Option<String> {
  unreachable!("{GUARDED}")
}

impl VZMultipleDirectoryShare {
  pub(crate) fn bridge_retained(&self) -> VZMultipleDirectoryShare {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_directories(&self) -> Vec<(String, VZSharedDirectory)> {
    match self.never {}
  }
}

pub(crate) struct VZSharedDirectory {
  never: Never,
}

pub(crate) fn vz_shared_directory_init(_url: &NSURL, _read_only: bool) -> VZSharedDirectory {
  unreachable!("{GUARDED}")
}

impl VZSharedDirectory {
  pub(crate) fn bridge_retained(&self) -> VZSharedDirectory {
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
}

pub(crate) struct VZSingleDirectoryShare {
  never: Never,
}

pub(crate) fn vz_single_directory_share_init(_directory: &VZSharedDirectory) -> VZSingleDirectoryShare {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_single_directory_share_bridge_cast(object: &NSObject) -> Option<VZSingleDirectoryShare> {
  object.never()
}

impl VZSingleDirectoryShare {
  pub(crate) fn bridge_retained(&self) -> VZSingleDirectoryShare {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_directory(&self) -> VZSharedDirectory {
    match self.never {}
  }
}

pub(crate) struct VZVirtioFileSystemDevice {
  never: Never,
}

pub(crate) fn vz_virtio_file_system_device_bridge_cast(object: &NSObject) -> Option<VZVirtioFileSystemDevice> {
  object.never()
}

impl VZVirtioFileSystemDevice {
  pub(crate) fn bridge_retained(&self) -> VZVirtioFileSystemDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_tag(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_share(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_share(&self, _share: Option<&NSObject>) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioFileSystemDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_file_system_device_configuration_init(_tag: &str) -> VZVirtioFileSystemDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_file_system_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioFileSystemDeviceConfiguration> {
  object.never()
}

pub(crate) fn vz_virtio_file_system_device_configuration_validate_tag(_tag: &str) -> Result<(), BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_file_system_device_configuration_macos_guest_automount_tag() -> String {
  unreachable!("{GUARDED}")
}

impl VZVirtioFileSystemDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioFileSystemDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_tag(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_tag(&self, _tag: &str) {
    match self.never {}
  }

  pub(crate) fn vz_share(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_share(&self, _share: Option<&NSObject>) {
    match self.never {}
  }
}
