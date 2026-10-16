//===----------------------------------------------------------------------===//
// `DiskImage`, `DiskImage.Format`, `DiskImage.BlockSize`,
// `DiskImage.LayerType`, and `StackedImage`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import DiskImageKit
import Foundation

func dik_disk_image_init_creating_asif(configuration: BridgedASIFCreationConfiguration) throws(DiskImageKitThrown)
  -> DiskImage
{
  try bridging { try DiskImage(creating: configuration.value) }
}

func dik_disk_image_init_creating_asif_layer(configuration: BridgedASIFLayerCreationConfiguration)
  throws(DiskImageKitThrown) -> DiskImage
{
  try bridging { try DiskImage(creating: configuration.value) }
}

func dik_disk_image_init_creating_raw(configuration: BridgedRAWCreationConfiguration) throws(DiskImageKitThrown)
  -> DiskImage
{
  try bridging { try DiskImage(creating: configuration.value) }
}

/// `init(opening:)`, with the `OpenConfiguration` made from its `url` and
/// `mode`: all `OpenConfigurationProtocol` has.
func dik_disk_image_init_opening(url: UnsafeRawPointer, mode: BridgedOpenConfigurationMode)
  throws(DiskImageKitThrown) -> DiskImage
{
  try bridging {
    try DiskImage(opening: OpenConfiguration(url: bridging(borrowed: url) as NSURL as URL, mode: .init(mode)))
  }
}

func dik_disk_image_url(this: DiskImage) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

func dik_disk_image_layer_uuid(this: DiskImage) -> UInt {
  bridging(retainedAddress: this.layerUUID as NSUUID?)
}

func dik_disk_image_parent_uuid(this: DiskImage) -> UInt {
  bridging(retainedAddress: this.parentUUID as NSUUID?)
}

func dik_disk_image_appending_disk_image(this: DiskImage, layer: DiskImage) throws(DiskImageKitThrown) -> DiskImage {
  try bridging { try this.appending(layer) }
}

func dik_disk_image_appending_asif_layer(this: DiskImage, configuration: BridgedASIFLayerCreationConfiguration)
  throws(DiskImageKitThrown) -> DiskImage
{
  try bridging { try this.appending(configuration.value) }
}

func dik_disk_image_truncate(this: DiskImage, block_count: Int) throws(DiskImageKitThrown) {
  try bridging { try this.truncate(blockCount: block_count) }
}

extension DiskImage {
  func bridge_retained() -> DiskImage {
    self
  }

  /// `String(describing:)`: `DiskImage` isn't an `NSObject`.
  func bridge_description() -> RustString {
    String(describing: self).intoRustString()
  }

  /// For another bridge to take with `bridging(borrowed:)`, while Rust holds
  /// this one.
  func bridge_borrowed() -> UnsafeRawPointer {
    UnsafeRawPointer(Unmanaged.passUnretained(self).toOpaque())
  }

  func dik_block_count() -> Int {
    blockCount
  }

  func dik_block_size() -> BridgedDiskImageBlockSize {
    .init(blockSize)
  }

  func dik_size() -> Int {
    size
  }

  func dik_layer_type() throws(DiskImageKitNil) -> BridgedDiskImageLayerType {
    try bridging(layerType.map(BridgedDiskImageLayerType.init))
  }

  func dik_format() -> BridgedDiskImageFormat {
    .init(format)
  }

  func dik_open_mode() -> BridgedOpenConfigurationMode {
    .init(openMode)
  }

  /// `StackedImage.layers`, by index. Rust calls these only on what
  /// `appending(_:)` returned, which is a `StackedImage`.
  func bridge_layers_count() -> UInt {
    UInt((self as! any StackedImage).layers.count)
  }

  func bridge_layer_at(index: UInt) -> DiskImage {
    (self as! any StackedImage).layers[Int(index)]
  }
}

/// `DiskImage.LayerType`, a struct, in a class swift-bridge can carry.
/// `Sendable`, as `LayerType` is.
final class BridgedDiskImageLayerType: Sendable {
  let value: DiskImage.LayerType

  init(_ value: DiskImage.LayerType) {
    self.value = value
  }

  func bridge_retained() -> BridgedDiskImageLayerType {
    self
  }

  func bridge_description() -> RustString {
    String(describing: value).intoRustString()
  }

  func bridge_eq(other: BridgedDiskImageLayerType) -> Bool {
    value == other.value
  }
}

func dik_disk_image_layer_type_cache() -> BridgedDiskImageLayerType {
  .init(.cache)
}

func dik_disk_image_layer_type_overlay() -> BridgedDiskImageLayerType {
  .init(.overlay)
}

func dik_disk_image_layer_type_overlay_block_count(block_count: Int) -> BridgedDiskImageLayerType {
  .init(.overlay(blockCount: block_count))
}

// `Format` only comes out of DiskImageKit, so it maps one way.

extension BridgedDiskImageFormat {
  init(_ format: DiskImage.Format) {
    switch format {
    case .asif: self = .Asif
    case .raw: self = .Raw
    case .stack: self = .Stack
    @unknown default: self = .Unknown
    }
  }
}

// `BlockSize` has `Int` raw values, but unlike an Objective-C enum's,
// `init(rawValue:)` rejects one with no case: passing that stops the process.

extension DiskImage.BlockSize {
  init(_ blockSize: BridgedDiskImageBlockSize) {
    switch blockSize {
    case .Bytes512: self = .bytes512
    case .Bytes4096: self = .bytes4096
    case .Unknown(let rawValue):
      guard let blockSize = DiskImage.BlockSize(rawValue: rawValue) else {
        fatalError("DiskImage.BlockSize has no case with raw value \(rawValue)")
      }
      self = blockSize
    }
  }
}

extension BridgedDiskImageBlockSize {
  init(_ blockSize: DiskImage.BlockSize) {
    switch blockSize {
    case .bytes512: self = .Bytes512
    case .bytes4096: self = .Bytes4096
    @unknown default: self = .Unknown(blockSize.rawValue)
    }
  }
}
