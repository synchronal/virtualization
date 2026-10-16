//===----------------------------------------------------------------------===//
// `VZHostAudioOutputStreamSink`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_host_audio_output_stream_sink_init() -> VZHostAudioOutputStreamSink {
  VZHostAudioOutputStreamSink()
}

/// `object as? VZHostAudioOutputStreamSink`, for an abstract class's object.
func vz_host_audio_output_stream_sink_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZHostAudioOutputStreamSink
{
  try bridging(bridging(borrowed: object) as NSObject as? VZHostAudioOutputStreamSink)
}
