use std::any::Any;
use std::sync::Arc;

use super::{BridgedThrown, GUARDED, NSError, NSObject, NSUUID, Never, Retained};

pub(crate) fn vz_usb_controller_attach(
  this: &NSObject,
  _device: &NSObject,
  _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
) {
  this.never()
}

pub(crate) fn vz_usb_controller_detach(
  this: &NSObject,
  _device: &NSObject,
  _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
) {
  this.never()
}

pub(crate) fn vz_usb_controller_usb_devices(this: &NSObject) -> Vec<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_usb_controller_delegate(this: &NSObject) -> Option<Arc<dyn VZUSBControllerDelegate>> {
  this.never()
}

pub(crate) fn vz_usb_controller_set_delegate(this: &NSObject, _delegate: Option<Arc<dyn VZUSBControllerDelegate>>) {
  this.never()
}

pub(crate) trait VZUSBControllerDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZUSBControllerDelegate for T {}

pub(crate) fn vz_usb_controller_configuration_usb_devices(this: &NSObject) -> Vec<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_usb_controller_configuration_set_usb_devices(this: &NSObject, _usb_devices: &[&NSObject]) {
  this.never()
}

pub(crate) fn vz_usb_device_usb_controller(this: &NSObject) -> Option<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_usb_device_uuid(this: &NSObject) -> Retained<NSUUID> {
  this.never()
}

pub(crate) fn vz_usb_device_configuration_uuid(this: &NSObject) -> Retained<NSUUID> {
  this.never()
}

pub(crate) fn vz_usb_device_configuration_set_uuid(this: &NSObject, _uuid: &NSUUID) {
  this.never()
}

pub(crate) struct VZUSBPassthroughDevice {
  never: Never,
}

pub(crate) fn vz_usb_passthrough_device_init_configuration(
  _configuration: &VZUSBPassthroughDeviceConfiguration,
) -> Result<VZUSBPassthroughDevice, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_passthrough_device_bridge_cast(object: &NSObject) -> Option<VZUSBPassthroughDevice> {
  object.never()
}

impl VZUSBPassthroughDevice {
  pub(crate) fn bridge_retained(&self) -> VZUSBPassthroughDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZUSBPassthroughDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_usb_passthrough_device_configuration_init(
  _device: &accessory_access::USBAccessory,
) -> VZUSBPassthroughDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_usb_passthrough_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZUSBPassthroughDeviceConfiguration> {
  object.never()
}

impl VZUSBPassthroughDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZUSBPassthroughDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZXHCIController {
  never: Never,
}

pub(crate) fn vz_xhci_controller_bridge_cast(object: &NSObject) -> Option<VZXHCIController> {
  object.never()
}

impl VZXHCIController {
  pub(crate) fn bridge_retained(&self) -> VZXHCIController {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZXHCIControllerConfiguration {
  never: Never,
}

pub(crate) fn vz_xhci_controller_configuration_init() -> VZXHCIControllerConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_xhci_controller_configuration_bridge_cast(object: &NSObject) -> Option<VZXHCIControllerConfiguration> {
  object.never()
}

impl VZXHCIControllerConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZXHCIControllerConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

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
