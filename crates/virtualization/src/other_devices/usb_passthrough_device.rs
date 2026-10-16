use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, USBDevice, USBPassthroughDeviceConfiguration, foundation};

/// [`VZUSBPassthroughDevice`](https://developer.apple.com/documentation/virtualization/vzusbpassthroughdevice)
pub struct USBPassthroughDevice {
  pub(crate) inner: ffi::VZUSBPassthroughDevice,
}

impl USBPassthroughDevice {
  /// [`init(configuration:)`](https://developer.apple.com/documentation/virtualization/vzusbpassthroughdevice/init(configuration:))
  pub fn new(configuration: &USBPassthroughDeviceConfiguration) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_passthrough_device_init_configuration(&configuration.inner).map_err(foundation::thrown)?,
    })
  }
}

impl Clone for USBPassthroughDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBPassthroughDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBPassthroughDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_passthrough_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl USBDevice for USBPassthroughDevice {}
