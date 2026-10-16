//===----------------------------------------------------------------------===//
// `VZVirtioSoundDeviceOutputStreamConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_sound_device_output_stream_configuration_init() -> VZVirtioSoundDeviceOutputStreamConfiguration {
  VZVirtioSoundDeviceOutputStreamConfiguration()
}

/// `object as? VZVirtioSoundDeviceOutputStreamConfiguration`, for an abstract
/// class's object.
func vz_virtio_sound_device_output_stream_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioSoundDeviceOutputStreamConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioSoundDeviceOutputStreamConfiguration)
}

func vz_virtio_sound_device_output_stream_configuration_sink(
  this: VZVirtioSoundDeviceOutputStreamConfiguration
) -> UInt {
  bridging(retainedAddress: this.sink)
}

/// `sink` is any `VZAudioOutputStreamSink`, or `0` for `nil`.
func vz_virtio_sound_device_output_stream_configuration_set_sink(
  this: VZVirtioSoundDeviceOutputStreamConfiguration,
  sink: UInt
) {
  this.sink = bridging(borrowedAddress: sink)
}
