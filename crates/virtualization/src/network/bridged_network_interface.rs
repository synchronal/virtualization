use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZBridgedNetworkInterface`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkinterface)
pub struct BridgedNetworkInterface {
  pub(crate) inner: ffi::VZBridgedNetworkInterface,
}

impl BridgedNetworkInterface {
  /// [`networkInterfaces`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkinterface/networkinterfaces)
  pub fn network_interfaces() -> Result<Vec<Self>, Error> {
    platform::supported()?;

    Ok(
      ffi::vz_bridged_network_interface_network_interfaces()
        .into_iter()
        .map(Self::from_inner)
        .collect(),
    )
  }

  pub(crate) fn from_inner(inner: ffi::VZBridgedNetworkInterface) -> Self {
    Self { inner }
  }

  /// [`identifier`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkinterface/identifier)
  pub fn identifier(&self) -> String {
    self.inner.vz_identifier()
  }

  /// [`localizedDisplayName`](https://developer.apple.com/documentation/virtualization/vzbridgednetworkinterface/localizeddisplayname)
  pub fn localized_display_name(&self) -> Option<String> {
    self.inner.vz_localized_display_name()
  }
}

impl Clone for BridgedNetworkInterface {
  fn clone(&self) -> Self {
    Self::from_inner(self.inner.bridge_retained())
  }
}

/// The object's `description`.
impl fmt::Debug for BridgedNetworkInterface {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      BridgedNetworkInterface::network_interfaces(),
      Err(Error::Unsupported)
    ));
  }
}
