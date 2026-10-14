//===----------------------------------------------------------------------===//
// `VZDiskBlockDeviceStorageDeviceAttachment` and `VZDiskSynchronizationMode`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `file_handle` is a `FileHandle`, lent by address.
func vz_disk_block_device_storage_device_attachment_init(
  file_handle: UnsafeRawPointer,
  read_only: Bool,
  synchronization_mode: BridgedVZDiskSynchronizationMode
) throws(BridgedThrown) -> VZDiskBlockDeviceStorageDeviceAttachment {
  try bridging {
    try VZDiskBlockDeviceStorageDeviceAttachment(
      fileHandle: bridging(borrowed: file_handle),
      readOnly: read_only,
      synchronizationMode: VZDiskSynchronizationMode(synchronization_mode)
    )
  }
}

/// `object as? VZDiskBlockDeviceStorageDeviceAttachment`, for an abstract
/// class's object.
func vz_disk_block_device_storage_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZDiskBlockDeviceStorageDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZDiskBlockDeviceStorageDeviceAttachment)
}

func vz_disk_block_device_storage_device_attachment_file_handle(this: VZDiskBlockDeviceStorageDeviceAttachment)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.fileHandle)
}

extension VZDiskBlockDeviceStorageDeviceAttachment {
  func vz_is_read_only() -> Bool {
    isReadOnly
  }

  func vz_synchronization_mode() -> BridgedVZDiskSynchronizationMode {
    BridgedVZDiskSynchronizationMode(synchronizationMode)
  }
}

// An Objective-C enum's `init(rawValue:)` accepts any value, so it never
// returns nil: this unwrap can't fail.

extension VZDiskSynchronizationMode {
  init(_ mode: BridgedVZDiskSynchronizationMode) {
    switch mode {
    case .Full: self = .full
    case .None: self = .none
    case .Unknown(let rawValue): self = VZDiskSynchronizationMode(rawValue: rawValue)!
    }
  }
}

extension BridgedVZDiskSynchronizationMode {
  init(_ mode: VZDiskSynchronizationMode) {
    switch mode {
    case .full: self = .Full
    case .none: self = .None
    @unknown default: self = .Unknown(mode.rawValue)
    }
  }
}
