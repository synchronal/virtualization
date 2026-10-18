use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, MacGuestProvisioningOptions, VirtualMachineStartOptions, foundation};

/// [`VZMacOSVirtualMachineStartOptions`](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions)
pub struct MacOSVirtualMachineStartOptions {
  inner: ffi::VZMacOSVirtualMachineStartOptions,
}

impl MacOSVirtualMachineStartOptions {
  /// [`init()`](https://developer.apple.com/documentation/objectivec/nsobject-swift.class/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_macos_virtual_machine_start_options_init(),
    })
  }

  /// [`startUpFromMacOSRecovery`](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions/startupfrommacosrecovery)
  pub fn start_up_from_macos_recovery(&self) -> bool {
    self.inner.vz_start_up_from_macos_recovery()
  }

  /// [`startUpFromMacOSRecovery`](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions/startupfrommacosrecovery)
  pub fn set_start_up_from_macos_recovery(&self, start_up_from_macos_recovery: bool) {
    self
      .inner
      .vz_set_start_up_from_macos_recovery(start_up_from_macos_recovery);
  }

  /// [`guestProvisioningOptions`](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions/guestprovisioningoptions)
  pub fn guest_provisioning_options(&self) -> Option<MacGuestProvisioningOptions> {
    self
      .inner
      .vz_guest_provisioning_options()
      .ok()
      .map(|inner| MacGuestProvisioningOptions { inner })
  }

  /// [`setGuestProvisioning(_:)`](https://developer.apple.com/documentation/virtualization/vzmacosvirtualmachinestartoptions/setguestprovisioning(_:))
  pub fn set_guest_provisioning(
    &self,
    guest_provisioning_options: Option<&MacGuestProvisioningOptions>,
  ) -> Result<(), Error> {
    match guest_provisioning_options {
      Some(guest_provisioning_options) => self
        .inner
        .vz_set_guest_provisioning(guest_provisioning_options.inner.bridge_retained()),
      None => self.inner.bridge_set_guest_provisioning_nil(),
    }
    .map_err(foundation::thrown)
  }
}

impl Clone for MacOSVirtualMachineStartOptions {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacOSVirtualMachineStartOptions {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacOSVirtualMachineStartOptions {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_macos_virtual_machine_start_options_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl VirtualMachineStartOptions for MacOSVirtualMachineStartOptions {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      MacOSVirtualMachineStartOptions::new(),
      Err(Error::Unsupported)
    ));
  }
}
