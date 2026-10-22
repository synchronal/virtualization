use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZVirtioDeviceSpecificConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiodevicespecificconfiguration)
pub struct VirtioDeviceSpecificConfiguration {
  pub(crate) inner: ffi::VZVirtioDeviceSpecificConfiguration,
}

impl VirtioDeviceSpecificConfiguration {
  /// [`init(configurationData:)`](https://developer.apple.com/documentation/virtualization/vzvirtiodevicespecificconfiguration/init(configurationdata:)):
  /// `Data` is its bytes.
  pub fn new(configuration_data: &[u8]) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_device_specific_configuration_init(configuration_data),
    })
  }

  /// [`configurationData`](https://developer.apple.com/documentation/virtualization/vzvirtiodevicespecificconfiguration/configurationdata):
  /// `Data` is its bytes.
  pub fn configuration_data(&self) -> Vec<u8> {
    self.inner.vz_configuration_data()
  }
}

impl Clone for VirtioDeviceSpecificConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioDeviceSpecificConfiguration {
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
      VirtioDeviceSpecificConfiguration::new(&[]),
      Err(Error::Unsupported)
    ));
  }
}
