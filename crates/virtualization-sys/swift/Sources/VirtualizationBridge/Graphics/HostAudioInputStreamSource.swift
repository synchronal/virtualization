//===----------------------------------------------------------------------===//
// `VZHostAudioInputStreamSource`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_host_audio_input_stream_source_init() -> VZHostAudioInputStreamSource {
  VZHostAudioInputStreamSource()
}

/// `object as? VZHostAudioInputStreamSource`, for an abstract class's object.
func vz_host_audio_input_stream_source_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZHostAudioInputStreamSource
{
  try bridging(bridging(borrowed: object) as NSObject as? VZHostAudioInputStreamSource)
}
