//===----------------------------------------------------------------------===//
// `VZVirtioSoundDeviceConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_sound_device_configuration_init() -> VZVirtioSoundDeviceConfiguration {
  VZVirtioSoundDeviceConfiguration()
}

/// `object as? VZVirtioSoundDeviceConfiguration`, for an abstract class's
/// object.
func vz_virtio_sound_device_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioSoundDeviceConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioSoundDeviceConfiguration)
}

func vz_virtio_sound_device_configuration_streams(this: VZVirtioSoundDeviceConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.streams as NSArray)
}

/// `streams` is an array of any `VZVirtioSoundDeviceStreamConfiguration`.
func vz_virtio_sound_device_configuration_set_streams(
  this: VZVirtioSoundDeviceConfiguration,
  streams: UnsafeRawPointer
) {
  this.streams = bridging(borrowedArray: streams)
}
