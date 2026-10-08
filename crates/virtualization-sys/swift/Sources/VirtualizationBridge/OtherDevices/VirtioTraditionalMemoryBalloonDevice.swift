//===----------------------------------------------------------------------===//
// `VZVirtioTraditionalMemoryBalloonDevice`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object as? VZVirtioTraditionalMemoryBalloonDevice`, for an abstract
/// class's object.
func vz_virtio_traditional_memory_balloon_device_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioTraditionalMemoryBalloonDevice
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioTraditionalMemoryBalloonDevice)
}

extension VZVirtioTraditionalMemoryBalloonDevice {
  func vz_target_virtual_machine_memory_size() -> UInt64 {
    targetVirtualMachineMemorySize
  }

  func vz_set_target_virtual_machine_memory_size(target_virtual_machine_memory_size: UInt64) {
    targetVirtualMachineMemorySize = target_virtual_machine_memory_size
  }
}
