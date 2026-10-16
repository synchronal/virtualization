use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, USBDeviceConfiguration};

/// [`VZUSBPassthroughDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbpassthroughdeviceconfiguration)
///
/// An `AAUSBAccessory` is [`accessory_access::USBAccessory`].
pub struct USBPassthroughDeviceConfiguration {
  pub(crate) inner: ffi::VZUSBPassthroughDeviceConfiguration,
}

impl USBPassthroughDeviceConfiguration {
  /// [`init(device:)`](https://developer.apple.com/documentation/virtualization/vzusbpassthroughdeviceconfiguration/init(device:))
  pub fn new(device: &accessory_access::USBAccessory) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_passthrough_device_configuration_init(accessory_access::bridge::usb_accessory(device)),
    })
  }
}

impl Clone for USBPassthroughDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBPassthroughDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBPassthroughDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_passthrough_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl USBDeviceConfiguration for USBPassthroughDeviceConfiguration {}
