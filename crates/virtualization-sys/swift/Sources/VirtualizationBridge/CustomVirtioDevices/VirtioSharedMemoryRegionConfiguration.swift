//===----------------------------------------------------------------------===//
// `VZVirtioSharedMemoryRegionConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_shared_memory_region_configuration_init(region_id: UInt8, size: UInt64)
  -> VZVirtioSharedMemoryRegionConfiguration
{
  VZVirtioSharedMemoryRegionConfiguration(regionID: region_id, size: size)
}

/// An element of an array of them, which Rust holds.
func vz_virtio_shared_memory_region_configuration_bridge_from(object: UnsafeRawPointer)
  -> VZVirtioSharedMemoryRegionConfiguration
{
  bridging(borrowed: object)
}

extension VZVirtioSharedMemoryRegionConfiguration {
  func vz_region_id() -> UInt8 {
    regionID
  }

  func vz_size() -> UInt64 {
    size
  }
}
