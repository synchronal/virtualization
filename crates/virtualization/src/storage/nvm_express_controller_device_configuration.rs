use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDeviceAttachment, StorageDeviceConfiguration};

/// [`VZNVMExpressControllerDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vznvmexpresscontrollerdeviceconfiguration)
pub struct NVMExpressControllerDeviceConfiguration {
  inner: ffi::VZNVMExpressControllerDeviceConfiguration,
}

impl NVMExpressControllerDeviceConfiguration {
  /// [`init(attachment:)`](https://developer.apple.com/documentation/virtualization/vznvmexpresscontrollerdeviceconfiguration/init(attachment:))
  pub fn new(attachment: &dyn StorageDeviceAttachment) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_nvm_express_controller_device_configuration_init(attachment.bridge_object()),
    })
  }
}

impl Clone for NVMExpressControllerDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NVMExpressControllerDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for NVMExpressControllerDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_nvm_express_controller_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDeviceConfiguration for NVMExpressControllerDeviceConfiguration {}
