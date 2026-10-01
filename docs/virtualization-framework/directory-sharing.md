# Directory Sharing

Exposes host directories to a guest through a Virtio file system device. A device configuration holds a directory share (single, multiple, or Rosetta); the running device exposes the same share.

## VZDirectoryShare

`class VZDirectoryShare: NSObject`

Base class for exposing host directories to a guest. Not created directly; use `VZSingleDirectoryShare`, `VZMultipleDirectoryShare`, or `VZLinuxRosettaDirectoryShare`.

## VZSharedDirectory

`class VZSharedDirectory: NSObject`

A host directory that can be exposed to a guest.

- `init(url: URL, readOnly: Bool)`: `url` must be a local file URL pointing to an existing directory.
- `var url: URL { get }`
- `var isReadOnly: Bool { get }`: whether the guest sees it as read-only.

## VZSingleDirectoryShare

`class VZSingleDirectoryShare: VZDirectoryShare`

Exposes one host directory to the guest.

- `init(directory: VZSharedDirectory)`
- `var directory: VZSharedDirectory { get }`

## VZMultipleDirectoryShare

`class VZMultipleDirectoryShare: VZDirectoryShare`

Exposes multiple host directories to the guest, each under a name.

- `convenience init()`: starts with no directories.
- `init(directories: [String : VZSharedDirectory])`: keys are directory names; an invalid name raises an exception.
- `var directories: [String : VZSharedDirectory] { get }`: an invalid key raises an exception.
- `class func validateName(_ name: String) throws`: the name must not be empty, contain file-system-unsafe characters, or exceed `NAME_MAX`, among other restrictions. The error says why it is invalid.
- `class func canonicalizedName(from name: String) -> String?`: nil when no valid name can be produced; any non-nil result is valid.

## VZDirectorySharingDeviceConfiguration

`class VZDirectorySharingDeviceConfiguration: NSObject, NSCopying`

Base class for directory sharing device configurations. Not created directly; use `VZVirtioFileSystemDeviceConfiguration`.

## VZVirtioFileSystemDeviceConfiguration

`class VZVirtioFileSystemDeviceConfiguration: VZDirectorySharingDeviceConfiguration, NSCopying`

Configures a Virtio file system device that exposes host directories to a guest under a tag.

- `init(tag: String)`: the tag must be valid.
- `class func validateTag(_ tag: String) throws`: the tag must be non-empty and under 36 UTF-8 bytes. The error says why it is invalid.
- `var tag: String { get set }`: the label the guest mounts the device by. Must be valid.
- `var share: VZDirectoryShare? { get set }`: accepts single, multiple, and Rosetta shares.
- `class var macOSGuestAutomountTag: String { get }`: a device with this tag is mounted automatically in a macOS guest.

## VZDirectorySharingDevice

`class VZDirectorySharingDevice: NSObject`

Base class for a directory sharing device in a running virtual machine. Not created directly; devices come from `VZVirtualMachine.directorySharingDevices`, each typed to match its configuration (a `VZVirtioFileSystemDeviceConfiguration` produces a `VZVirtioFileSystemDevice`).

## VZVirtioFileSystemDevice

`class VZVirtioFileSystemDevice: VZDirectorySharingDevice`

Exposes host resources to the guest as a file system mount.

- `var tag: String { get }`: the label the guest mounts the device by.
- `var share: VZDirectoryShare? { get set }`: setting a `VZLinuxRosettaDirectoryShare` is unsupported and raises an exception.

## VZLinuxRosettaAvailability

`enum VZLinuxRosettaAvailability: Sendable`

Whether the host has Rosetta for Linux binaries. Apple silicon only.

- `init?(rawValue: Int)`
- `case notSupported` (0)
- `case notInstalled` (1)
- `case installed` (2)

## VZLinuxRosettaDirectoryShare

`class VZLinuxRosettaDirectoryShare: VZDirectoryShare`

Exposes Rosetta in a shared directory so a Linux guest can translate x86_64 binaries. Apple silicon only.

- `init() throws`: throws if Rosetta is unavailable for a directory share; install it with `installRosetta`.
- `class func installRosetta(completionHandler: @escaping @Sendable ((any Error)?) -> Void)`: downloads and installs Rosetta if needed, prompting the user. The error is nil on success. The handler is called on an arbitrary queue.
- `class func installRosetta() async throws`
- `class var availability: VZLinuxRosettaAvailability { get }`
- `var cachingOptions: VZLinuxRosettaDirectoryShare.CachingOptions? { get }`: translation caching and the socket type for reaching the Rosetta daemon.
- `func setCachingOptions(_ cachingOptions: VZLinuxRosettaDirectoryShare.CachingOptions?) throws`

## VZLinuxRosettaDirectoryShare.CachingOptions

`enum CachingOptions`

How Rosetta in the guest communicates with the Rosetta translation daemon. Apple silicon only.

- `case abstractSocket(String)`: an Abstract Socket name of at most `maximumNameLength` UTF-8 bytes.
- `case unixSocket(String)`: a Unix Domain Socket path of at most `maximumPathLength` UTF-8 bytes. Caching works only if the guest has a directory at the path.
- `static var defaultUnixSocket: VZLinuxRosettaDirectoryShare.CachingOptions { get }`: a Unix Domain Socket at `/run/rosettad/rosetta.sock`.
- `static var maximumNameLength: Int { get }`: the limit from Linux `sockaddr_un`.
- `static var maximumPathLength: Int { get }`: the limit from Linux `sockaddr_un`.
