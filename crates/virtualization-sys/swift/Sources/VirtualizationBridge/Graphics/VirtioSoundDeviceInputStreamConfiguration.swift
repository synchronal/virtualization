//===----------------------------------------------------------------------===//
// `VZVirtioSoundDeviceInputStreamConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_sound_device_input_stream_configuration_init() -> VZVirtioSoundDeviceInputStreamConfiguration {
  VZVirtioSoundDeviceInputStreamConfiguration()
}

/// `object as? VZVirtioSoundDeviceInputStreamConfiguration`, for an abstract
/// class's object.
func vz_virtio_sound_device_input_stream_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioSoundDeviceInputStreamConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioSoundDeviceInputStreamConfiguration)
}

func vz_virtio_sound_device_input_stream_configuration_source(
  this: VZVirtioSoundDeviceInputStreamConfiguration
) -> UInt {
  bridging(retainedAddress: this.source)
}

/// `source` is any `VZAudioInputStreamSource`, or `0` for `nil`.
func vz_virtio_sound_device_input_stream_configuration_set_source(
  this: VZVirtioSoundDeviceInputStreamConfiguration,
  source: UInt
) {
  this.source = bridging(borrowedAddress: source)
}
