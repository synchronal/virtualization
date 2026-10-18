use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, MacAuxiliaryStorage, MacHardwareModel, MacMachineIdentifier, PlatformConfiguration};

/// [`VZMacPlatformConfiguration`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration)
pub struct MacPlatformConfiguration {
  inner: ffi::VZMacPlatformConfiguration,
}

impl MacPlatformConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_platform_configuration_init(),
    })
  }

  /// [`hardwareModel`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/hardwaremodel)
  pub fn hardware_model(&self) -> MacHardwareModel {
    MacHardwareModel {
      inner: self.inner.vz_hardware_model(),
    }
  }

  /// [`hardwareModel`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/hardwaremodel)
  pub fn set_hardware_model(&self, hardware_model: &MacHardwareModel) {
    self
      .inner
      .vz_set_hardware_model(hardware_model.inner.bridge_retained());
  }

  /// [`machineIdentifier`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/machineidentifier)
  pub fn machine_identifier(&self) -> MacMachineIdentifier {
    MacMachineIdentifier {
      inner: self.inner.vz_machine_identifier(),
    }
  }

  /// [`machineIdentifier`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/machineidentifier)
  pub fn set_machine_identifier(&self, machine_identifier: &MacMachineIdentifier) {
    self
      .inner
      .vz_set_machine_identifier(machine_identifier.inner.bridge_retained());
  }

  /// [`auxiliaryStorage`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/auxiliarystorage)
  pub fn auxiliary_storage(&self) -> Option<MacAuxiliaryStorage> {
    self
      .inner
      .vz_auxiliary_storage()
      .ok()
      .map(|inner| MacAuxiliaryStorage { inner })
  }

  /// [`auxiliaryStorage`](https://developer.apple.com/documentation/virtualization/vzmacplatformconfiguration/auxiliarystorage)
  pub fn set_auxiliary_storage(&self, auxiliary_storage: Option<&MacAuxiliaryStorage>) {
    match auxiliary_storage {
      Some(auxiliary_storage) => self
        .inner
        .vz_set_auxiliary_storage(auxiliary_storage.inner.bridge_retained()),
      None => self.inner.bridge_set_auxiliary_storage_nil(),
    }
  }
}

impl Clone for MacPlatformConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacPlatformConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacPlatformConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_mac_platform_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl PlatformConfiguration for MacPlatformConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacPlatformConfiguration::new(), Err(Error::Unsupported)));
  }
}
