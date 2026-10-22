use std::fmt;

use crate::platform::{self, ffi};
use crate::{
  CustomVirtioDeviceProvider, Error, VirtioDeviceSpecificConfiguration, VirtioFeatureSet,
  VirtioSharedMemoryRegionConfiguration,
};

/// [`VZCustomVirtioDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration)
pub struct CustomVirtioDeviceConfiguration {
  pub(crate) inner: ffi::VZCustomVirtioDeviceConfiguration,
}

impl CustomVirtioDeviceConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/objectivec/nsobject-swift.class/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_custom_virtio_device_configuration_init(),
    })
  }

  /// [`maximumAllowedSharedMemoryRegionCount`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/maximumallowedsharedmemoryregioncount)
  pub fn maximum_allowed_shared_memory_region_count() -> Result<isize, Error> {
    platform::supported()?;

    Ok(ffi::vz_custom_virtio_device_configuration_maximum_allowed_shared_memory_region_count())
  }

  /// [`deviceID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/deviceid)
  pub fn device_id(&self) -> u16 {
    self.inner.vz_device_id()
  }

  /// [`deviceID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/deviceid)
  pub fn set_device_id(&self, device_id: u16) {
    self.inner.vz_set_device_id(device_id);
  }

  /// [`pciClassID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/pciclassid)
  pub fn pci_class_id(&self) -> u8 {
    self.inner.vz_pci_class_id()
  }

  /// [`pciClassID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/pciclassid)
  pub fn set_pci_class_id(&self, pci_class_id: u8) {
    self.inner.vz_set_pci_class_id(pci_class_id);
  }

  /// [`pciSubclassID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/pcisubclassid)
  pub fn pci_subclass_id(&self) -> u8 {
    self.inner.vz_pci_subclass_id()
  }

  /// [`pciSubclassID`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/pcisubclassid)
  pub fn set_pci_subclass_id(&self, pci_subclass_id: u8) {
    self.inner.vz_set_pci_subclass_id(pci_subclass_id);
  }

  /// [`virtioQueueCount`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/virtioqueuecount)
  pub fn virtio_queue_count(&self) -> u16 {
    self.inner.vz_virtio_queue_count()
  }

  /// [`virtioQueueCount`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/virtioqueuecount)
  pub fn set_virtio_queue_count(&self, virtio_queue_count: u16) {
    self.inner.vz_set_virtio_queue_count(virtio_queue_count);
  }

  /// [`mandatoryFeatures`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/mandatoryfeatures)
  pub fn mandatory_features(&self) -> VirtioFeatureSet {
    VirtioFeatureSet {
      inner: self.inner.vz_mandatory_features(),
    }
  }

  /// [`optionalFeatures`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/optionalfeatures)
  pub fn optional_features(&self) -> VirtioFeatureSet {
    VirtioFeatureSet {
      inner: self.inner.vz_optional_features(),
    }
  }

  /// [`deviceSpecificConfiguration`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/devicespecificconfiguration)
  pub fn device_specific_configuration(&self) -> Option<VirtioDeviceSpecificConfiguration> {
    self
      .inner
      .vz_device_specific_configuration()
      .ok()
      .map(|inner| VirtioDeviceSpecificConfiguration { inner })
  }

  /// [`deviceSpecificConfiguration`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/devicespecificconfiguration)
  pub fn set_device_specific_configuration(
    &self,
    device_specific_configuration: Option<&VirtioDeviceSpecificConfiguration>,
  ) {
    match device_specific_configuration {
      Some(device_specific_configuration) => self
        .inner
        .vz_set_device_specific_configuration(device_specific_configuration.inner.bridge_retained()),
      None => self.inner.bridge_set_device_specific_configuration_nil(),
    }
  }

  /// [`provider`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/provider),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn provider<T: CustomVirtioDeviceProvider>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_provider()?)
  }

  /// [`provider`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/provider)
  pub fn set_provider(&self, provider: Option<&dyn CustomVirtioDeviceProvider>) {
    self
      .inner
      .vz_set_provider(provider.map(|provider| provider.bridge_object()));
  }

  /// [`sharedMemoryRegions`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/sharedmemoryregions)
  pub fn shared_memory_regions(&self) -> Vec<VirtioSharedMemoryRegionConfiguration> {
    self
      .inner
      .vz_shared_memory_regions()
      .into_iter()
      .map(|inner| VirtioSharedMemoryRegionConfiguration { inner })
      .collect()
  }

  /// [`sharedMemoryRegions`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/sharedmemoryregions)
  pub fn set_shared_memory_regions(&self, shared_memory_regions: &[&VirtioSharedMemoryRegionConfiguration]) {
    let shared_memory_regions: Vec<_> = shared_memory_regions
      .iter()
      .map(|region| &region.inner)
      .collect();

    self
      .inner
      .vz_set_shared_memory_regions(&shared_memory_regions);
  }

  /// [`supportsSaveRestore`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/supportssaverestore)
  pub fn supports_save_restore(&self) -> bool {
    self.inner.vz_supports_save_restore()
  }

  /// [`supportsSaveRestore`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfiguration/supportssaverestore)
  pub fn set_supports_save_restore(&self, supports_save_restore: bool) {
    self
      .inner
      .vz_set_supports_save_restore(supports_save_restore);
  }
}

impl Clone for CustomVirtioDeviceConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for CustomVirtioDeviceConfiguration {
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
      CustomVirtioDeviceConfiguration::new(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      CustomVirtioDeviceConfiguration::maximum_allowed_shared_memory_region_count(),
      Err(Error::Unsupported)
    ));
  }
}
