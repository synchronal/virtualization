//===----------------------------------------------------------------------===//
// `VZCustomVirtioDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_custom_virtio_device_configuration_init() -> VZCustomVirtioDeviceConfiguration {
  VZCustomVirtioDeviceConfiguration()
}

/// An element of an array of them, which Rust holds.
func vz_custom_virtio_device_configuration_bridge_from(object: UnsafeRawPointer) -> VZCustomVirtioDeviceConfiguration {
  bridging(borrowed: object)
}

func vz_custom_virtio_device_configuration_maximum_allowed_shared_memory_region_count() -> Int {
  VZCustomVirtioDeviceConfiguration.maximumAllowedSharedMemoryRegionCount
}

/// Any `VZCustomVirtioDeviceProvider`, by address: `0` for `nil`.
func vz_custom_virtio_device_configuration_provider(this: VZCustomVirtioDeviceConfiguration) -> UInt {
  bridging(retainedAddress: this.provider)
}

/// `provider` is any `VZCustomVirtioDeviceProvider`, by address: `0` for
/// `nil`.
func vz_custom_virtio_device_configuration_set_provider(this: VZCustomVirtioDeviceConfiguration, provider: UInt) {
  this.provider = bridging(borrowedAddress: provider)
}

func vz_custom_virtio_device_configuration_shared_memory_regions(this: VZCustomVirtioDeviceConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.sharedMemoryRegions as NSArray)
}

/// `shared_memory_regions` is an array of
/// `VZVirtioSharedMemoryRegionConfiguration`.
func vz_custom_virtio_device_configuration_set_shared_memory_regions(
  this: VZCustomVirtioDeviceConfiguration,
  shared_memory_regions: UnsafeRawPointer
) {
  this.sharedMemoryRegions = bridging(borrowedArray: shared_memory_regions)
}

extension VZCustomVirtioDeviceConfiguration {
  func vz_device_id() -> UInt16 {
    deviceID
  }

  func vz_set_device_id(device_id: UInt16) {
    deviceID = device_id
  }

  func vz_pci_class_id() -> UInt8 {
    pciClassID
  }

  func vz_set_pci_class_id(pci_class_id: UInt8) {
    pciClassID = pci_class_id
  }

  func vz_pci_subclass_id() -> UInt8 {
    pciSubclassID
  }

  func vz_set_pci_subclass_id(pci_subclass_id: UInt8) {
    pciSubclassID = pci_subclass_id
  }

  func vz_virtio_queue_count() -> UInt16 {
    virtioQueueCount
  }

  func vz_set_virtio_queue_count(virtio_queue_count: UInt16) {
    virtioQueueCount = virtio_queue_count
  }

  func vz_mandatory_features() -> VZVirtioFeatureSet {
    mandatoryFeatures
  }

  func vz_optional_features() -> VZVirtioFeatureSet {
    optionalFeatures
  }

  func vz_device_specific_configuration() throws(BridgedNil) -> VZVirtioDeviceSpecificConfiguration {
    try bridging(deviceSpecificConfiguration)
  }

  func vz_set_device_specific_configuration(device_specific_configuration: VZVirtioDeviceSpecificConfiguration) {
    deviceSpecificConfiguration = device_specific_configuration
  }

  func bridge_set_device_specific_configuration_nil() {
    deviceSpecificConfiguration = nil
  }

  func vz_supports_save_restore() -> Bool {
    supportsSaveRestore
  }

  func vz_set_supports_save_restore(supports_save_restore: Bool) {
    supportsSaveRestore = supports_save_restore
  }
}
