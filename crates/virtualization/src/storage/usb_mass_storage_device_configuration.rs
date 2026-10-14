use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDeviceAttachment, StorageDeviceConfiguration, USBDeviceConfiguration};

/// [`VZUSBMassStorageDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbmassstoragedeviceconfiguration)
pub struct USBMassStorageDeviceConfiguration {
  pub(crate) inner: ffi::VZUSBMassStorageDeviceConfiguration,
}

impl USBMassStorageDeviceConfiguration {
  /// [`init(attachment:)`](https://developer.apple.com/documentation/virtualization/vzusbmassstoragedeviceconfiguration/init(attachment:))
  pub fn new(attachment: &dyn StorageDeviceAttachment) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_usb_mass_storage_device_configuration_init(attachment.bridge_object()),
    })
  }
}

impl Clone for USBMassStorageDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBMassStorageDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for USBMassStorageDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_usb_mass_storage_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDeviceConfiguration for USBMassStorageDeviceConfiguration {}

impl USBDeviceConfiguration for USBMassStorageDeviceConfiguration {}
