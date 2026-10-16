use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, GenericMachineIdentifier, PlatformConfiguration};

/// [`VZGenericPlatformConfiguration`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration)
pub struct GenericPlatformConfiguration {
  inner: ffi::VZGenericPlatformConfiguration,
}

impl GenericPlatformConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_generic_platform_configuration_init(),
    })
  }

  /// [`isNestedVirtualizationSupported`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/isnestedvirtualizationsupported)
  pub fn is_nested_virtualization_supported() -> Result<bool, Error> {
    platform::supported()?;

    Ok(ffi::vz_generic_platform_configuration_is_nested_virtualization_supported())
  }

  /// [`machineIdentifier`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/machineidentifier)
  pub fn machine_identifier(&self) -> GenericMachineIdentifier {
    GenericMachineIdentifier {
      inner: self.inner.vz_machine_identifier(),
    }
  }

  /// [`machineIdentifier`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/machineidentifier)
  pub fn set_machine_identifier(&self, machine_identifier: &GenericMachineIdentifier) {
    self
      .inner
      .vz_set_machine_identifier(machine_identifier.inner.bridge_retained());
  }

  /// [`isNestedVirtualizationEnabled`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/isnestedvirtualizationenabled)
  pub fn is_nested_virtualization_enabled(&self) -> bool {
    self.inner.vz_is_nested_virtualization_enabled()
  }

  /// [`isNestedVirtualizationEnabled`](https://developer.apple.com/documentation/virtualization/vzgenericplatformconfiguration/isnestedvirtualizationenabled)
  pub fn set_nested_virtualization_enabled(&self, nested_virtualization_enabled: bool) {
    self
      .inner
      .vz_set_nested_virtualization_enabled(nested_virtualization_enabled);
  }
}

impl Clone for GenericPlatformConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for GenericPlatformConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for GenericPlatformConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_generic_platform_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl PlatformConfiguration for GenericPlatformConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(GenericPlatformConfiguration::new(), Err(Error::Unsupported)));
    assert!(matches!(
      GenericPlatformConfiguration::is_nested_virtualization_supported(),
      Err(Error::Unsupported)
    ));
  }
}
