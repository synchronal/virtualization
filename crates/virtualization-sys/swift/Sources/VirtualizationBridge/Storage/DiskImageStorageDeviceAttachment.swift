//===----------------------------------------------------------------------===//
// `VZDiskImageStorageDeviceAttachment`, `VZDiskImageCachingMode`, and
// `VZDiskImageSynchronizationMode`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import DiskImageKit
import Virtualization

func vz_disk_image_storage_device_attachment_init(url: UnsafeRawPointer, read_only: Bool) throws(BridgedThrown)
  -> VZDiskImageStorageDeviceAttachment
{
  try bridging {
    try VZDiskImageStorageDeviceAttachment(url: bridging(borrowed: url) as NSURL as URL, readOnly: read_only)
  }
}

func vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
  url: UnsafeRawPointer,
  read_only: Bool,
  caching_mode: BridgedVZDiskImageCachingMode,
  synchronization_mode: BridgedVZDiskImageSynchronizationMode
) throws(BridgedThrown) -> VZDiskImageStorageDeviceAttachment {
  try bridging {
    try VZDiskImageStorageDeviceAttachment(
      url: bridging(borrowed: url) as NSURL as URL,
      readOnly: read_only,
      cachingMode: VZDiskImageCachingMode(caching_mode),
      synchronizationMode: VZDiskImageSynchronizationMode(synchronization_mode)
    )
  }
}

/// `diskImage` is DiskImageKitBridge's, lent by address
/// (`DiskImage.bridge_borrowed()`).
func vz_disk_image_storage_device_attachment_init_disk_image_caching_mode_synchronization_mode(
  disk_image: UnsafeRawPointer,
  caching_mode: BridgedVZDiskImageCachingMode,
  synchronization_mode: BridgedVZDiskImageSynchronizationMode
) throws(BridgedThrown) -> VZDiskImageStorageDeviceAttachment {
  try bridging {
    try VZDiskImageStorageDeviceAttachment(
      diskImage: bridging(borrowed: disk_image) as DiskImage,
      cachingMode: VZDiskImageCachingMode(caching_mode),
      synchronizationMode: VZDiskImageSynchronizationMode(synchronization_mode)
    )
  }
}

/// `object as? VZDiskImageStorageDeviceAttachment`, for an abstract class's
/// object.
func vz_disk_image_storage_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZDiskImageStorageDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZDiskImageStorageDeviceAttachment)
}

func vz_disk_image_storage_device_attachment_url(this: VZDiskImageStorageDeviceAttachment) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

extension VZDiskImageStorageDeviceAttachment {
  func vz_is_read_only() -> Bool {
    isReadOnly
  }

  func vz_caching_mode() -> BridgedVZDiskImageCachingMode {
    BridgedVZDiskImageCachingMode(cachingMode)
  }

  func vz_synchronization_mode() -> BridgedVZDiskImageSynchronizationMode {
    BridgedVZDiskImageSynchronizationMode(synchronizationMode)
  }
}

// An Objective-C enum's `init(rawValue:)` accepts any value, so it never
// returns nil: these unwraps can't fail.

extension VZDiskImageCachingMode {
  init(_ mode: BridgedVZDiskImageCachingMode) {
    switch mode {
    case .Automatic: self = .automatic
    case .Uncached: self = .uncached
    case .Cached: self = .cached
    case .Unknown(let rawValue): self = VZDiskImageCachingMode(rawValue: rawValue)!
    }
  }
}

extension BridgedVZDiskImageCachingMode {
  init(_ mode: VZDiskImageCachingMode) {
    switch mode {
    case .automatic: self = .Automatic
    case .uncached: self = .Uncached
    case .cached: self = .Cached
    @unknown default: self = .Unknown(mode.rawValue)
    }
  }
}

extension VZDiskImageSynchronizationMode {
  init(_ mode: BridgedVZDiskImageSynchronizationMode) {
    switch mode {
    case .Full: self = .full
    case .Fsync: self = .fsync
    case .None: self = .none
    case .Unknown(let rawValue): self = VZDiskImageSynchronizationMode(rawValue: rawValue)!
    }
  }
}

extension BridgedVZDiskImageSynchronizationMode {
  init(_ mode: VZDiskImageSynchronizationMode) {
    switch mode {
    case .full: self = .Full
    case .fsync: self = .Fsync
    case .none: self = .None
    @unknown default: self = .Unknown(mode.rawValue)
    }
  }
}
