use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, SerialPortAttachment};

/// [`VZSpiceAgentPortAttachment`](https://developer.apple.com/documentation/virtualization/vzspiceagentportattachment)
pub struct SpiceAgentPortAttachment {
  inner: ffi::VZSpiceAgentPortAttachment,
}

impl SpiceAgentPortAttachment {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzspiceagentportattachment/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_spice_agent_port_attachment_init(),
    })
  }

  /// [`spiceAgentPortName`](https://developer.apple.com/documentation/virtualization/vzspiceagentportattachment/spiceagentportname)
  pub fn spice_agent_port_name() -> Result<String, Error> {
    platform::supported()?;

    Ok(ffi::vz_spice_agent_port_attachment_spice_agent_port_name())
  }

  /// [`sharesClipboard`](https://developer.apple.com/documentation/virtualization/vzspiceagentportattachment/sharesclipboard)
  pub fn shares_clipboard(&self) -> bool {
    self.inner.vz_shares_clipboard()
  }

  /// [`sharesClipboard`](https://developer.apple.com/documentation/virtualization/vzspiceagentportattachment/sharesclipboard)
  pub fn set_shares_clipboard(&self, shares_clipboard: bool) {
    self.inner.vz_set_shares_clipboard(shares_clipboard);
  }
}

impl Clone for SpiceAgentPortAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for SpiceAgentPortAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for SpiceAgentPortAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_spice_agent_port_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SerialPortAttachment for SpiceAgentPortAttachment {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(SpiceAgentPortAttachment::new(), Err(Error::Unsupported)));
    assert!(matches!(
      SpiceAgentPortAttachment::spice_agent_port_name(),
      Err(Error::Unsupported)
    ));
  }
}
