use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, GuestProvisioningOptions};

/// [`VZMacGuestProvisioningOptions`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions)
///
/// `Send` and `Sync`, as Apple declares it `Sendable`.
pub struct MacGuestProvisioningOptions {
  pub(crate) inner: ffi::VZMacGuestProvisioningOptions,
}

impl MacGuestProvisioningOptions {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_guest_provisioning_options_init(),
    })
  }

  /// [`fullName`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/fullname)
  pub fn full_name(&self) -> String {
    self.inner.vz_full_name()
  }

  /// [`fullName`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/fullname)
  pub fn set_full_name(&self, full_name: &str) {
    self.inner.vz_set_full_name(full_name);
  }

  /// [`username`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/username)
  pub fn username(&self) -> String {
    self.inner.vz_username()
  }

  /// [`username`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/username)
  pub fn set_username(&self, username: &str) {
    self.inner.vz_set_username(username);
  }

  /// [`password`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/password)
  pub fn password(&self) -> String {
    self.inner.vz_password()
  }

  /// [`password`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/password)
  pub fn set_password(&self, password: &str) {
    self.inner.vz_set_password(password);
  }

  /// [`logsInAutomatically`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/logsinautomatically)
  pub fn logs_in_automatically(&self) -> bool {
    self.inner.vz_logs_in_automatically()
  }

  /// [`logsInAutomatically`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/logsinautomatically)
  pub fn set_logs_in_automatically(&self, logs_in_automatically: bool) {
    self
      .inner
      .vz_set_logs_in_automatically(logs_in_automatically);
  }

  /// [`enablesRemoteLogin`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/enablesremotelogin)
  pub fn enables_remote_login(&self) -> bool {
    self.inner.vz_enables_remote_login()
  }

  /// [`enablesRemoteLogin`](https://developer.apple.com/documentation/virtualization/vzmacguestprovisioningoptions/enablesremotelogin)
  pub fn set_enables_remote_login(&self, enables_remote_login: bool) {
    self.inner.vz_set_enables_remote_login(enables_remote_login);
  }
}

impl Clone for MacGuestProvisioningOptions {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacGuestProvisioningOptions {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MacGuestProvisioningOptions {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_mac_guest_provisioning_options_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl GuestProvisioningOptions for MacGuestProvisioningOptions {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MacGuestProvisioningOptions::new(), Err(Error::Unsupported)));
  }
}
