//===----------------------------------------------------------------------===//
// `VZNetworkBlockDeviceStorageDeviceAttachment` and
// `VZNetworkBlockDeviceStorageDeviceAttachmentDelegate`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_network_block_device_storage_device_attachment_init(url: UnsafeRawPointer) throws(BridgedThrown)
  -> VZNetworkBlockDeviceStorageDeviceAttachment
{
  try bridging {
    try VZNetworkBlockDeviceStorageDeviceAttachment(url: bridging(borrowed: url) as NSURL as URL)
  }
}

func vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
  url: UnsafeRawPointer,
  timeout: Double,
  is_forced_read_only: Bool,
  synchronization_mode: BridgedVZDiskSynchronizationMode
) throws(BridgedThrown) -> VZNetworkBlockDeviceStorageDeviceAttachment {
  try bridging {
    try VZNetworkBlockDeviceStorageDeviceAttachment(
      url: bridging(borrowed: url) as NSURL as URL,
      timeout: timeout,
      isForcedReadOnly: is_forced_read_only,
      synchronizationMode: VZDiskSynchronizationMode(synchronization_mode)
    )
  }
}

/// `object as? VZNetworkBlockDeviceStorageDeviceAttachment`, for an abstract
/// class's object.
func vz_network_block_device_storage_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZNetworkBlockDeviceStorageDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZNetworkBlockDeviceStorageDeviceAttachment)
}

func vz_network_block_device_storage_device_attachment_validate(url: UnsafeRawPointer) throws(BridgedThrown) {
  try bridging {
    try VZNetworkBlockDeviceStorageDeviceAttachment.validate(bridging(borrowed: url) as NSURL as URL)
  }
}

func vz_network_block_device_storage_device_attachment_url(this: VZNetworkBlockDeviceStorageDeviceAttachment)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.url as NSURL)
}

/// Rust's delegate, which
/// `vz_network_block_device_storage_device_attachment_set_delegate` set, by
/// the address Rust takes it from: `0` for none.
func vz_network_block_device_storage_device_attachment_delegate(this: VZNetworkBlockDeviceStorageDeviceAttachment)
  -> UInt
{
  (this.delegate as? BridgedNetworkBlockDeviceStorageDeviceAttachmentDelegate)?.delegate.bridge_retained() ?? 0
}

/// `delegate` is weak, so the attachment keeps the
/// `BridgedNetworkBlockDeviceStorageDeviceAttachmentDelegate` it is given.
/// Rust's delegate is weak in turn.
func vz_network_block_device_storage_device_attachment_set_delegate(
  this: VZNetworkBlockDeviceStorageDeviceAttachment,
  delegate: VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge
) {
  let delegate = BridgedNetworkBlockDeviceStorageDeviceAttachmentDelegate(delegate)

  objc_setAssociatedObject(this, delegateKey.address, delegate, .OBJC_ASSOCIATION_RETAIN)
  this.delegate = delegate
}

extension VZNetworkBlockDeviceStorageDeviceAttachment {
  func vz_timeout() -> Double {
    timeout
  }

  func vz_is_forced_read_only() -> Bool {
    isForcedReadOnly
  }

  func vz_synchronization_mode() -> BridgedVZDiskSynchronizationMode {
    BridgedVZDiskSynchronizationMode(synchronizationMode)
  }

  func bridge_set_delegate_nil() {
    objc_setAssociatedObject(self, delegateKey.address, nil, .OBJC_ASSOCIATION_RETAIN)
    delegate = nil
  }
}

/// Where an attachment keeps its
/// `BridgedNetworkBlockDeviceStorageDeviceAttachmentDelegate`.
private let delegateKey = AssociationKey()

/// A `VZNetworkBlockDeviceStorageDeviceAttachmentDelegate` that forwards to
/// Rust's.
final class BridgedNetworkBlockDeviceStorageDeviceAttachmentDelegate: NSObject,
  VZNetworkBlockDeviceStorageDeviceAttachmentDelegate
{
  let delegate: VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge

  init(_ delegate: VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge) {
    self.delegate = delegate
  }

  func attachmentWasConnected(_ attachment: VZNetworkBlockDeviceStorageDeviceAttachment) {
    delegate.attachment_was_connected(attachment)
  }

  func attachment(_ attachment: VZNetworkBlockDeviceStorageDeviceAttachment, didEncounterError error: any Error) {
    delegate.attachment_did_encounter_error(attachment, bridging(retained: error as NSError))
  }
}
