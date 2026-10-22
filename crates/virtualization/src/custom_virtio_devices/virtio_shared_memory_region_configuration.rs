use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// [`VZVirtioSharedMemoryRegionConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregionconfiguration)
pub struct VirtioSharedMemoryRegionConfiguration {
  pub(crate) inner: ffi::VZVirtioSharedMemoryRegionConfiguration,
}

impl VirtioSharedMemoryRegionConfiguration {
  /// [`init(regionID:size:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregionconfiguration/init(regionid:size:))
  pub fn new(region_id: u8, size: u64) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_shared_memory_region_configuration_init(region_id, size),
    })
  }

  /// [`regionID`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregionconfiguration/regionid)
  pub fn region_id(&self) -> u8 {
    self.inner.vz_region_id()
  }

  /// [`size`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregionconfiguration/size)
  pub fn size(&self) -> u64 {
    self.inner.vz_size()
  }
}

impl Clone for VirtioSharedMemoryRegionConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSharedMemoryRegionConfiguration {
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
      VirtioSharedMemoryRegionConfiguration::new(0, 0),
      Err(Error::Unsupported)
    ));
  }
}
