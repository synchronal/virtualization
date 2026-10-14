mod bridged_network_device_attachment;
mod bridged_network_interface;
mod file_handle_network_device_attachment;
mod mac_address;
mod nat_network_device_attachment;
mod network_device;
mod virtio_network_device_configuration;

pub use self::bridged_network_device_attachment::BridgedNetworkDeviceAttachment;
pub use self::bridged_network_interface::BridgedNetworkInterface;
pub use self::file_handle_network_device_attachment::FileHandleNetworkDeviceAttachment;
pub use self::mac_address::MACAddress;
pub use self::nat_network_device_attachment::NATNetworkDeviceAttachment;
pub use self::network_device::NetworkDevice;
pub use self::virtio_network_device_configuration::VirtioNetworkDeviceConfiguration;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZNetworkDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration)
pub trait NetworkDeviceConfiguration: Sealed {
  /// [`macAddress`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration/macaddress)
  fn mac_address(&self) -> MACAddress {
    MACAddress::from_inner(ffi::vz_network_device_configuration_mac_address(self.bridge_object()))
  }

  /// [`macAddress`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration/macaddress)
  fn set_mac_address(&self, mac_address: &MACAddress) {
    ffi::vz_network_device_configuration_set_mac_address(self.bridge_object(), &mac_address.inner);
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration/attachment),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  fn attachment<T: NetworkDeviceAttachment>(&self) -> Option<T>
  where
    Self: Sized,
  {
    T::bridge_cast(&*ffi::vz_network_device_configuration_attachment(self.bridge_object())?)
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceconfiguration/attachment)
  fn set_attachment(&self, attachment: Option<&dyn NetworkDeviceAttachment>) {
    ffi::vz_network_device_configuration_set_attachment(
      self.bridge_object(),
      attachment.map(|attachment| attachment.bridge_object()),
    );
  }
}

/// [`VZNetworkDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vznetworkdeviceattachment)
pub trait NetworkDeviceAttachment: Sealed {}
