//===----------------------------------------------------------------------===//
// `ASIFCreationConfiguration`, `ASIFLayerCreationConfiguration`, and
// `RAWCreationConfiguration`: structs, each in a class swift-bridge can carry.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import DiskImageKit
import Foundation

final class BridgedASIFCreationConfiguration {
  let value: ASIFCreationConfiguration

  init(_ value: ASIFCreationConfiguration) {
    self.value = value
  }

  func bridge_retained() -> BridgedASIFCreationConfiguration {
    self
  }

  func bridge_description() -> RustString {
    String(describing: value).intoRustString()
  }
}

func dik_asif_creation_configuration_asif(
  url: UnsafeRawPointer,
  block_count: Int,
  block_size: BridgedDiskImageBlockSize
) -> BridgedASIFCreationConfiguration {
  .init(.asif(url: bridging(borrowed: url) as NSURL as URL, blockCount: block_count, blockSize: .init(block_size)))
}

func dik_asif_creation_configuration_layer(url: UnsafeRawPointer, layer_type: BridgedDiskImageLayerType)
  -> BridgedASIFLayerCreationConfiguration
{
  .init(ASIFCreationConfiguration.layer(url: bridging(borrowed: url) as NSURL as URL, type: layer_type.value))
}

final class BridgedASIFLayerCreationConfiguration {
  let value: ASIFLayerCreationConfiguration

  init(_ value: ASIFLayerCreationConfiguration) {
    self.value = value
  }

  func bridge_retained() -> BridgedASIFLayerCreationConfiguration {
    self
  }

  func bridge_description() -> RustString {
    String(describing: value).intoRustString()
  }
}

func dik_asif_layer_creation_configuration_asif_layer(url: UnsafeRawPointer, layer_type: BridgedDiskImageLayerType)
  -> BridgedASIFLayerCreationConfiguration
{
  .init(.asifLayer(url: bridging(borrowed: url) as NSURL as URL, type: layer_type.value))
}

final class BridgedRAWCreationConfiguration {
  let value: RAWCreationConfiguration

  init(_ value: RAWCreationConfiguration) {
    self.value = value
  }

  func bridge_retained() -> BridgedRAWCreationConfiguration {
    self
  }

  func bridge_description() -> RustString {
    String(describing: value).intoRustString()
  }
}

func dik_raw_creation_configuration_raw(url: UnsafeRawPointer, block_count: Int) -> BridgedRAWCreationConfiguration {
  .init(.raw(url: bridging(borrowed: url) as NSURL as URL, blockCount: block_count))
}
