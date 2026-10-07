use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, StorageDeviceAttachment, StorageDeviceConfiguration, foundation};

/// [`VZVirtioBlockDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtioblockdeviceconfiguration)
pub struct VirtioBlockDeviceConfiguration {
  inner: ffi::VZVirtioBlockDeviceConfiguration,
}

impl VirtioBlockDeviceConfiguration {
  /// [`init(attachment:)`](https://developer.apple.com/documentation/virtualization/vzvirtioblockdeviceconfiguration/init(attachment:))
  pub fn new(attachment: &dyn StorageDeviceAttachment) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_block_device_configuration_init(attachment.bridge_object()),
    })
  }

  /// [`validateBlockDeviceIdentifier(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtioblockdeviceconfiguration/validateblockdeviceidentifier(_:))
  pub fn validate_block_device_identifier(block_device_identifier: &str) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_virtio_block_device_configuration_validate_block_device_identifier(block_device_identifier)
      .map_err(foundation::thrown)
  }

  /// [`blockDeviceIdentifier`](https://developer.apple.com/documentation/virtualization/vzvirtioblockdeviceconfiguration/blockdeviceidentifier)
  pub fn block_device_identifier(&self) -> String {
    self.inner.vz_block_device_identifier()
  }

  /// [`blockDeviceIdentifier`](https://developer.apple.com/documentation/virtualization/vzvirtioblockdeviceconfiguration/blockdeviceidentifier)
  pub fn set_block_device_identifier(&self, block_device_identifier: &str) {
    self
      .inner
      .vz_set_block_device_identifier(block_device_identifier);
  }
}

impl Clone for VirtioBlockDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioBlockDeviceConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioBlockDeviceConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_block_device_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDeviceConfiguration for VirtioBlockDeviceConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      VirtioBlockDeviceConfiguration::validate_block_device_identifier("disk"),
      Err(Error::Unsupported)
    ));
  }
}
