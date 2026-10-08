use super::{GUARDED, NSObject, Never};

pub(crate) struct VZVirtioEntropyDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_entropy_device_configuration_init() -> VZVirtioEntropyDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_entropy_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioEntropyDeviceConfiguration> {
  object.never()
}

impl VZVirtioEntropyDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioEntropyDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioTraditionalMemoryBalloonDevice {
  never: Never,
}

pub(crate) fn vz_virtio_traditional_memory_balloon_device_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioTraditionalMemoryBalloonDevice> {
  object.never()
}

impl VZVirtioTraditionalMemoryBalloonDevice {
  pub(crate) fn bridge_retained(&self) -> VZVirtioTraditionalMemoryBalloonDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_target_virtual_machine_memory_size(&self) -> u64 {
    match self.never {}
  }

  pub(crate) fn vz_set_target_virtual_machine_memory_size(&self, _target_virtual_machine_memory_size: u64) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_traditional_memory_balloon_device_configuration_init()
-> VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_traditional_memory_balloon_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioTraditionalMemoryBalloonDeviceConfiguration> {
  object.never()
}

impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}
