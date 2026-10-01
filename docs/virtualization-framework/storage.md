# Storage

Storage devices in a virtual machine, the configurations that create them, and the host-side attachments backing them. A configuration (Virtio block, NVMe, USB mass storage) wraps an attachment (disk image, block device, or network block device).

## VZStorageDevice

`class VZStorageDevice: NSObject`: Abstract runtime storage device; use a subclass such as `VZUSBMassStorageDevice`.

## VZStorageDeviceConfiguration

`class VZStorageDeviceConfiguration: NSObject, NSCopying`: Abstract; use `VZVirtioBlockDeviceConfiguration`, `VZNVMExpressControllerDeviceConfiguration`, or `VZUSBMassStorageDeviceConfiguration`.

- `var attachment: VZStorageDeviceAttachment { get }`: The local resource exposed to the guest as a disk.

## VZStorageDeviceAttachment

`class VZStorageDeviceAttachment: NSObject`: Abstract; defines how a storage device interfaces with the host. Use a subclass such as `VZDiskImageStorageDeviceAttachment`.

## VZVirtioBlockDeviceConfiguration

`class VZVirtioBlockDeviceConfiguration: VZStorageDeviceConfiguration, NSCopying`: Paravirtualized, per the Virtio block device specification.

- `init(attachment: VZStorageDeviceAttachment)`
- `class func validateBlockDeviceIdentifier(_ blockDeviceIdentifier: String) throws`: Throws if the identifier is not ASCII-encodable or is longer than 20 bytes.
- `var blockDeviceIdentifier: String { get set }`: Identifies the device to the guest (read via a `VIRTIO_BLK_T_GET_ID` request). Empty by default. ASCII, at most 20 bytes.

## VZNVMExpressControllerDeviceConfiguration

`class VZNVMExpressControllerDeviceConfiguration: VZStorageDeviceConfiguration, NSCopying`: Conforms to NVMe specification revision 1.1b. Valid only with `VZGenericPlatformConfiguration`.

- `init(attachment: VZStorageDeviceAttachment)`

## VZUSBMassStorageDeviceConfiguration

`class VZUSBMassStorageDeviceConfiguration: VZStorageDeviceConfiguration, VZUSBDeviceConfiguration, NSCopying`: Conforms to the USB Mass Storage specification.

- `init(attachment: VZStorageDeviceAttachment)`

## VZUSBMassStorageDevice

`class VZUSBMassStorageDevice: VZStorageDevice, VZUSBDevice`: Hot-pluggable.

- `init(configuration: VZUSBMassStorageDeviceConfiguration)`: Created directly, can be passed to `VZUSBController`'s attach method; created through a `VZUSBMassStorageDeviceConfiguration` in a `VZVirtualMachineConfiguration`, appears in `VZUSBController.usbDevices`.

## VZDiskImageStorageDeviceAttachment

`class VZDiskImageStorageDeviceAttachment: VZStorageDeviceAttachment`: A disk image on the host file system as the device's drive. RAW images only; file size must be a multiple of the 512-byte block size. Create an empty image with `FileDescriptor.resize(to:retryOnInterrupt:)`, `ftruncate()`, or the `truncate` command.

- `init(url: URL, readOnly: Bool) throws`: `url` must be a local file URL. If `readOnly` is false, the device can write to the image.
- `init(url: URL, readOnly: Bool, cachingMode: VZDiskImageCachingMode, synchronizationMode: VZDiskImageSynchronizationMode) throws`
- `convenience init(diskImage: DiskImage, cachingMode: VZDiskImageCachingMode = .automatic, synchronizationMode: VZDiskImageSynchronizationMode = .full) throws`: macOS 27.
- `var url: URL { get }`: For an attachment created from a `DiskImage`, the URL of the top image in the stack.
- `var isReadOnly: Bool { get }`
- `var cachingMode: VZDiskImageCachingMode { get }`
- `var synchronizationMode: VZDiskImageSynchronizationMode { get }`

## VZDiskImageCachingMode

`enum VZDiskImageCachingMode: Sendable`: Whether the host caches disk image data.

- `case automatic`: Raw value 0. Virtualization decides.
- `case uncached`: Raw value 1. Disabled.
- `case cached`: Raw value 2. Enabled.
- `init?(rawValue: Int)`

## VZDiskImageSynchronizationMode

`enum VZDiskImageSynchronizationMode: Sendable`: How a disk image syncs with the underlying storage when the guest flushes.

- `case full`: Raw value 1. Data reaches the permanent storage holding the image; no synchronized data is lost on panic or power loss.
- `case fsync`: Raw value 2. Data reaches the drive, not necessarily moving from its internal cache to permanent storage. Best effort, same guarantees as `fsync()`.
- `case none`: Raw value 3. No synchronization; data integrity not guaranteed after errors such as a full host disk, panic, or power loss. Suits a VM that runs once to completion or failure; may perform better.
- `init?(rawValue: Int)`

## VZDiskBlockDeviceStorageDeviceAttachment

`class VZDiskBlockDeviceStorageDeviceAttachment: VZStorageDeviceAttachment`: Runs I/O directly on a disk (e.g. `/dev/rdisk42`), not through a file system. If the disk has a file system, the guest can destroy its data unrecoverably. Disk file handles are usually `root`-only; rather than running the VM as root, open the handle in a separate privileged process and pass the file descriptor to a less privileged process running Virtualization.

- `init(fileHandle: FileHandle, readOnly: Bool, synchronizationMode: VZDiskSynchronizationMode) throws`: The attachment retains the handle, which must be open when the VM starts. `readOnly` sets how the guest sees the disk; for a read-only disk, also open the handle read-only. If the handle disallows writes, the device cannot write.
- `var fileHandle: FileHandle { get }`
- `var isReadOnly: Bool { get }`
- `var synchronizationMode: VZDiskSynchronizationMode { get }`

## VZNetworkBlockDeviceStorageDeviceAttachment

`class VZNetworkBlockDeviceStorageDeviceAttachment: VZStorageDeviceAttachment`: An NBD client forwarding guest I/O to an NBD server, identified by an NBD URI given as a URL. The server runs outside Virtualization's control. The client connects when the VM starts, not at attachment creation; reconnects after recoverable errors such as timeouts and unexpected connection errors; and disconnects when the VM shuts down. Requires the `com.apple.security.network.client` entitlement. NBD URI format: https://github.com/NetworkBlockDevice/nbd/blob/master/doc/uri.md

- `init(url URL: URL, timeout: TimeInterval, isForcedReadOnly forcedReadOnly: Bool, synchronizationMode: VZDiskSynchronizationMode) throws`: `timeout` is in seconds; on expiry the client tries to reconnect. The server advertises read-only status in the NBD handshake; `forcedReadOnly` presents the disk to the guest as read-only regardless.
- `convenience init(url URL: URL) throws`: Optimized defaults for timeout, forced read-only, and synchronization mode.
- `class func validate(_ URL: URL) throws`: Throws if not a valid NBD URL. Checks well-formedness only; does not access the URL.
- `var url: URL { get }`
- `var timeout: TimeInterval { get }`
- `var isForcedReadOnly: Bool { get }`
- `var synchronizationMode: VZDiskSynchronizationMode { get }`
- `weak var delegate: (any VZNetworkBlockDeviceStorageDeviceAttachmentDelegate)? { get set }`

## VZNetworkBlockDeviceStorageDeviceAttachmentDelegate

`protocol VZNetworkBlockDeviceStorageDeviceAttachmentDelegate : NSObjectProtocol`: Tracks an NBD attachment's state.

- `optional func attachmentWasConnected(_ attachment: VZNetworkBlockDeviceStorageDeviceAttachment)`: The client connected: on first start and after each reconnection, so possibly many times over the VM's life. Reconnections are transparent to the guest.
- `optional func attachment(_ attachment: VZNetworkBlockDeviceStorageDeviceAttachment, didEncounterError error: any Error)`: Non-recoverable error; the client is non-functional afterwards.

## VZDiskSynchronizationMode

`enum VZDiskSynchronizationMode: Sendable`: How a block device or NBD client syncs data when the guest flushes.

- `case full`: Raw value 0. Guest flush and barrier commands pass to the disk implementation as their counterpart synchronization commands.
- `case none`: Raw value 1. No synchronization; data integrity not guaranteed after errors such as a full host disk, panic, or power loss, and block state and order on disk are undefined after a failure. Suits a VM that runs once to completion or failure; may perform better.
- `init?(rawValue: Int)`
