use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDevice, USBDevice, USBMassStorageDeviceConfiguration};

/// [`VZUSBMassStorageDevice`](https://developer.apple.com/documentation/virtualization/vzusbmassstoragedevice)
pub struct USBMassStorageDevice {
  inner: ffi::VZUSBMassStorageDevice,
}

impl USBMassStorageDevice {
  /// [`init(configuration:)`](https://developer.apple.com/documentation/virtualization/vzusbmassstoragedevice/init(configuration:))
  pub fn new(configuration: &USBMassStorageDeviceConfiguration) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_mass_storage_device_init(&configuration.inner),
    })
  }
}

impl Clone for USBMassStorageDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBMassStorageDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBMassStorageDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_mass_storage_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDevice for USBMassStorageDevice {}

impl USBDevice for USBMassStorageDevice {}
