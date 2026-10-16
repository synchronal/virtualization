use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZMACAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress)
///
/// `ether_addr_t` is its six octets.
pub struct MACAddress {
  pub(crate) inner: ffi::VZMACAddress,
}

impl MACAddress {
  /// [`init(ethernetAddress:)`](https://developer.apple.com/documentation/virtualization/vzmacaddress/init(ethernetaddress:))
  pub fn new(ethernet_address: [u8; 6]) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_address_init_ethernet_address(ethernet_address),
    })
  }

  /// [`init(string:)`](https://developer.apple.com/documentation/virtualization/vzmacaddress/init(string:))
  pub fn with_string(string: &str) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::vz_mac_address_init_string(string).map(|inner| Self { inner }))
  }

  /// [`randomLocallyAdministered()`](https://developer.apple.com/documentation/virtualization/vzmacaddress/randomlocallyadministered())
  pub fn random_locally_administered() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_mac_address_random_locally_administered(),
    })
  }

  /// [`ethernetAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/ethernetaddress)
  pub fn ethernet_address(&self) -> [u8; 6] {
    self.inner.vz_ethernet_address()
  }

  /// [`string`](https://developer.apple.com/documentation/virtualization/vzmacaddress/string)
  pub fn string(&self) -> String {
    self.inner.vz_string()
  }

  /// [`isBroadcastAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/isbroadcastaddress)
  pub fn is_broadcast_address(&self) -> bool {
    self.inner.vz_is_broadcast_address()
  }

  /// [`isMulticastAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/ismulticastaddress)
  pub fn is_multicast_address(&self) -> bool {
    self.inner.vz_is_multicast_address()
  }

  /// [`isUnicastAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/isunicastaddress)
  pub fn is_unicast_address(&self) -> bool {
    self.inner.vz_is_unicast_address()
  }

  /// [`isLocallyAdministeredAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/islocallyadministeredaddress)
  pub fn is_locally_administered_address(&self) -> bool {
    self.inner.vz_is_locally_administered_address()
  }

  /// [`isUniversallyAdministeredAddress`](https://developer.apple.com/documentation/virtualization/vzmacaddress/isuniversallyadministeredaddress)
  pub fn is_universally_administered_address(&self) -> bool {
    self.inner.vz_is_universally_administered_address()
  }
}

impl Clone for MACAddress {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MACAddress {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MACAddress::new([0; 6]), Err(Error::Unsupported)));
    assert!(matches!(MACAddress::with_string(""), Err(Error::Unsupported)));
    assert!(matches!(
      MACAddress::random_locally_administered(),
      Err(Error::Unsupported)
    ));
  }
}
