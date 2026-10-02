# DiskImageKit

Apple's DiskImageKit framework as of the macOS 27 SDK, in Swift. Everything here is macOS 27+, and macOS only.

DiskImageKit creates, opens, resizes, and stacks disk images. A `DiskImage` can back a virtual machine's storage through Virtualization's `VZDiskImageStorageDeviceAttachment.init(diskImage:cachingMode:synchronizationMode:)`.

The framework is Swift only: it has no Objective-C headers, and none of its types are `NSObject`s.

## DiskImage

`class DiskImage`

An open disk image, either standalone or part of a stack (see `StackedImage`). Encrypted disk images are not supported.

- `convenience init(creating configuration: some CreationConfiguration) throws`: creates a new, empty standalone image, or the base image of a stack. `configuration` must not be a `StackableLayer`; append layers with `appending(_:)`. An existing file at the URL is overwritten. Throws `InvalidBlockCountError` if the block count is zero or negative, or `POSIXError` if the image can't be created.
- `convenience init(opening configuration: some OpenConfigurationProtocol) throws`: opens an existing image, standalone or a layer. Throws `CorruptedImageError` if the image holds invalid data, `UnsupportedFormatError` if its format isn't supported, or `POSIXError` for file system errors such as a missing file or denied permission.
- `final let url: URL`: for a standalone image, the URL it was created or opened with; for a `StackedImage`, the top layer's URL.
- `var blockCount: Int { get }`
- `var blockSize: DiskImage.BlockSize { get }`
- `var size: Int { get }`: the logical size in bytes, `blockCount` × `blockSize`.
- `var format: DiskImage.Format { get }`
- `var openMode: OpenConfiguration.Mode { get }`: read-only or read-write.
- `var layerType: DiskImage.LayerType? { get }`: `.cache` or `.overlay` for those layers of a stack; `nil` for a standalone image, a stack's base layer, and the stacked image itself.
- `var layerUUID: UUID? { get }`: checked against the `parentUUID` of the layer above it. Every image has one except RAW images. It changes when the framework first writes to the image; in a stack only the top image can be written. A cache layer's never changes.
- `var parentUUID: UUID? { get }`: must equal the `layerUUID` of the layer beneath it. Set on every image in a stack whose layer beneath has a `layerUUID`.
- `consuming func appending(_ layer: consuming DiskImage) throws -> any StackedImage`: appends an existing image as the new top layer, creating a stack from a base image or extending one. Throws `IncompatibleStackingError` if the layer isn't compatible with the stack.
- `consuming func appending(_ configuration: any DiskImage.CreationConfiguration & DiskImage.StackableLayer) throws -> any StackedImage`: creates a new layer and appends it as the top layer. Throws `IncompatibleStackingError` if it isn't compatible with the stack, `InvalidBlockCountError` if the block count is zero or negative, or `POSIXError` if the layer can't be created.
- `func truncate(blockCount: Int) throws`: shrinks or extends the image, not its contents such as file systems and partition maps; shrinking below a file system's size can lose data. `blockCount` must be greater than zero. Not for a cache image. On a stack, truncates the top layer, whose size is the stack's. Throws `InvalidBlockCountError` if the block count is zero or negative, or `POSIXError` if it fails.

A stack may have only one cache layer.

## DiskImage.Format

`enum Format: Equatable, Hashable`

- `case asif`: Apple sparse image format (ASIF).
- `case raw`
- `case stack`: a stacked disk image.

## DiskImage.BlockSize

`enum BlockSize: Int, CaseIterable, Sendable, Equatable, Hashable, RawRepresentable`

- `case bytes512`
- `case bytes4096`: not supported for RAW images.
- `init?(rawValue: Int)`
- `var rawValue: Int { get }`
- `static var allCases: [DiskImage.BlockSize] { get }`

## DiskImage.LayerType

`struct LayerType: Equatable, Sendable`

The type of a layer in a stack.

- `static let cache: DiskImage.LayerType`: stores every block read from the layers beneath it, e.g. when the base image is on a network mount.
- `static let overlay: DiskImage.LayerType`: stores every block changed from the layers beneath it, and takes its size from the layer beneath.
- `static func overlay(blockCount: Int) -> DiskImage.LayerType`: an overlay with its own block count, which resizes the stack, since the top layer sets the stack's size. `blockCount` must be greater than zero.

## DiskImage.CreationConfiguration

`protocol CreationConfiguration`

Marks configurations for creating an image. Made with:

- `static func asif(url: URL, blockCount: Int, blockSize: DiskImage.BlockSize) -> ASIFCreationConfiguration`: a standalone or base ASIF image, for `init(creating:)`.
- `static func asifLayer(url: URL, type: DiskImage.LayerType) -> ASIFLayerCreationConfiguration`: an ASIF cache or overlay layer, for `appending(_:)`. Calls `ASIFCreationConfiguration.layer(url:type:)`.
- `static func raw(url: URL, blockCount: Int) -> RAWCreationConfiguration`: a standalone or base RAW image, for `init(creating:)`. RAW images use 512-byte blocks.

## DiskImage.StackableLayer

`protocol StackableLayer`

Marks layer configurations that `appending(_:)` accepts.

## ASIFCreationConfiguration

`struct ASIFCreationConfiguration: DiskImage.CreationConfiguration`

Creates an ASIF image.

- `static func layer(url: URL, type: DiskImage.LayerType) -> ASIFLayerCreationConfiguration`: a cache or overlay layer, only for appending to a stack.

## ASIFLayerCreationConfiguration

`struct ASIFLayerCreationConfiguration: DiskImage.CreationConfiguration, DiskImage.StackableLayer`

Creates an ASIF layer of a stack; only for `appending(_:)`.

## RAWCreationConfiguration

`struct RAWCreationConfiguration: DiskImage.CreationConfiguration`

Creates a RAW image.

## OpenConfigurationProtocol

`protocol OpenConfigurationProtocol`

What `DiskImage.init(opening:)` takes.

- `var url: URL { get }`
- `var mode: OpenConfiguration.Mode { get }`
- `static func open(url: URL, mode: OpenConfiguration.Mode = .automatic) -> OpenConfiguration`

## OpenConfiguration

`struct OpenConfiguration: OpenConfigurationProtocol`

- `init(url: URL, mode: OpenConfiguration.Mode = .automatic)`
- `let url: URL`
- `let mode: OpenConfiguration.Mode`

## OpenConfiguration.Mode

`enum Mode: Equatable, Hashable`

- `case automatic`: read-write, falling back to read-only without write access.
- `case readOnly`: use for a stack's base image.
- `case readWrite`: fails without write permission.

## StackedImage

`protocol StackedImage: DiskImage`

A stack of layers read as one disk image. The bottom layer is the base; each layer above it is a cache or overlay layer over the layers beneath.

- `var layers: [DiskImage] { get }`: from the base (index 0) to the top.

## Errors

Each is a `struct` conforming to `Error` and `LocalizedError`, with `errorDescription`.

- `IncompatibleStackingError`: the appended layer isn't compatible with the stack. `let reason: String` says why.
- `InvalidBlockCountError`: the block count is zero or negative.
- `CorruptedImageError`: the image is corrupted or holds invalid data. `let underlyingError: any Error` has diagnostic details.
- `UnsupportedFormatError`: the image's format isn't supported. `let underlyingError: any Error` has diagnostic details.
