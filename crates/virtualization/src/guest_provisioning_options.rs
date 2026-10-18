use crate::platform::ffi;
use crate::sealed::Sealed;
use crate::{Error, foundation};

/// [`VZGuestProvisioningOptions`](https://developer.apple.com/documentation/virtualization/vzguestprovisioningoptions)
pub trait GuestProvisioningOptions: Sealed {
  /// [`validate()`](https://developer.apple.com/documentation/virtualization/vzguestprovisioningoptions/validate())
  fn validate(&self) -> Result<(), Error> {
    ffi::vz_guest_provisioning_options_validate(self.bridge_object()).map_err(foundation::thrown)
  }
}
