use super::{GUARDED, NSObject, Never, Retained};

pub(crate) struct VZMACAddress {
  never: Never,
}

pub(crate) fn vz_mac_address_init_ethernet_address(_ethernet_address: [u8; 6]) -> VZMACAddress {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_address_init_string(_string: &str) -> Option<VZMACAddress> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_address_random_locally_administered() -> VZMACAddress {
  unreachable!("{GUARDED}")
}

impl VZMACAddress {
  pub(crate) fn bridge_retained(&self) -> VZMACAddress {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_ethernet_address(&self) -> [u8; 6] {
    match self.never {}
  }

  pub(crate) fn vz_string(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_is_broadcast_address(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_is_multicast_address(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_is_unicast_address(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_is_locally_administered_address(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_is_universally_administered_address(&self) -> bool {
    match self.never {}
  }
}

pub(crate) struct VZNATNetworkDeviceAttachment {
  never: Never,
}

pub(crate) fn vz_nat_network_device_attachment_init() -> VZNATNetworkDeviceAttachment {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_nat_network_device_attachment_bridge_cast(object: &NSObject) -> Option<VZNATNetworkDeviceAttachment> {
  object.never()
}

impl VZNATNetworkDeviceAttachment {
  pub(crate) fn bridge_retained(&self) -> VZNATNetworkDeviceAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZNetworkDevice {
  never: Never,
}

impl VZNetworkDevice {
  pub(crate) fn bridge_retained(&self) -> VZNetworkDevice {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_attachment(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_attachment(&self, _attachment: Option<&NSObject>) {
    match self.never {}
  }
}

pub(crate) fn vz_network_device_configuration_mac_address(this: &NSObject) -> VZMACAddress {
  this.never()
}

pub(crate) fn vz_network_device_configuration_set_mac_address(this: &NSObject, _mac_address: &VZMACAddress) {
  this.never()
}

pub(crate) fn vz_network_device_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_network_device_configuration_set_attachment(this: &NSObject, _attachment: Option<&NSObject>) {
  this.never()
}

pub(crate) struct VZVirtioNetworkDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_network_device_configuration_init() -> VZVirtioNetworkDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_network_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioNetworkDeviceConfiguration> {
  object.never()
}

impl VZVirtioNetworkDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioNetworkDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}
