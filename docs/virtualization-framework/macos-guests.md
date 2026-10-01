# macOS Guests

Types for booting and installing macOS on a virtual machine: boot loader, platform configuration, hardware model, machine identifier, auxiliary storage, restore images, the installer, and start options including guest provisioning. Everything in this file is Apple silicon only.

## VZMacOSBootLoader

`class VZMacOSBootLoader: VZBootLoader, NSCopying`

Must be used with a `VZMacPlatformConfiguration`; any other platform configuration is invalid.

- `init()`

## VZMacPlatformConfiguration

`class VZMacPlatformConfiguration: VZPlatformConfiguration, NSCopying`

For a new virtual machine, the hardware model and auxiliary storage depend on the restore image used to install macOS. Each virtual machine created from the same configuration should have its own auxiliary storage and machine identifier. When reloading a saved virtual machine, restore all three properties to their original values.

- `init()`
- `@NSCopying var hardwareModel: VZMacHardwareModel { get set }`: Take it from `VZMacOSRestoreImage.mostFeaturefulSupportedConfiguration`.
- `@NSCopying var machineIdentifier: VZMacMachineIdentifier { get set }`: Two virtual machines running at once with the same identifier give undefined behavior in the guest.
- `var auxiliaryStorage: VZMacAuxiliaryStorage? { get set }`: For a new virtual machine, its hardware model must match `hardwareModel`.

## VZMacHardwareModel

`class VZMacHardwareModel: NSObject, NSCopying`

Abstracts a set of virtualized hardware elements and configurations. A macOS version runs only on certain models, and the host offers only certain models depending on its macOS version and hardware. Keep the installation's model for later boots.

- `init?(dataRepresentation: Data)`: `nil` if the data isn't a valid representation.
- `var dataRepresentation: Data { get }`: Opaque, serializable; `init(dataRepresentation:)` recreates the same model.
- `var isSupported: Bool { get }`: Whether the host supports it. If not, no `VZVirtualMachineConfiguration` using it validates; the validation error explains why.

## VZMacMachineIdentifier

`class VZMacMachineIdentifier: NSObject, NSCopying`

macOS guests use it to tell virtual hardware apart. Two virtual machines running at once should not share one. Compare with `isEqual(_:)`.

- `init()`: A new unique identifier.
- `init?(dataRepresentation: Data)`: Identical to the one that produced the data; `nil` if the data is invalid.
- `var dataRepresentation: Data { get }`: Opaque, serializable; recreates the same identifier.

## VZMacAuxiliaryStorage

`class VZMacAuxiliaryStorage: NSObject`

Data used by the boot loader and the macOS guest; required to boot a macOS guest. The installer lays it out and the guest uses it on every later boot. Move or copy this file with the main disk image when moving or backing up a virtual machine. For a new virtual machine, `VZMacOSInstaller` can use a default-initialized auxiliary storage.

- `init(url URL: URL)`: Opens an existing file, such as the one used at installation. The platform's `hardwareModel` must match the file's; otherwise behavior is undefined.
- `init(creatingStorageAt URL: URL, hardwareModel: VZMacHardwareModel, options: VZMacAuxiliaryStorage.InitializationOptions = []) throws`: Writes empty storage to the file system; layout can differ per hardware model. The platform's `hardwareModel` must be identical to this one; otherwise behavior is undefined.
- `var url: URL { get }`: Location on the local file system.

## VZMacAuxiliaryStorage.InitializationOptions

`struct InitializationOptions`

- `init(rawValue: UInt)`
- `static var allowOverwrite: VZMacAuxiliaryStorage.InitializationOptions { get }`: Overwrite existing auxiliary storage. Raw value `1 << 0`.

## VZMacOSRestoreImage

`class VZMacOSRestoreImage: NSObject`

A macOS version to install, loaded from installation media. Loading requires the `com.apple.security.virtualization` entitlement.

- `class func image(from fileURL: URL) async throws -> VZMacOSRestoreImage`: Loads from a local file; a URL that isn't a local file raises an exception.
- `@preconcurrency class func load(from fileURL: URL, completionHandler: @escaping @Sendable (Result<VZMacOSRestoreImage, any Error>) -> Void)`: Completion-handler form of `image(from:)`; the handler runs on an arbitrary thread.
- `class var latestSupported: VZMacOSRestoreImage { get async throws }`: Fetches from the network the latest image the host supports. Its `url` is a network URL; download it locally (e.g. with `URLSession`) before giving it to `VZMacOSInstaller`.
- `@preconcurrency class func fetchLatestSupported(completionHandler: @escaping @Sendable (Result<VZMacOSRestoreImage, any Error>) -> Void)`: Completion-handler form of `latestSupported`; the handler runs on an arbitrary thread.
- `var isSupported: Bool { get }`: Whether the current host supports it.
- `var url: URL { get }`: A file URL if loaded from a file; a network URL if fetched.
- `var buildVersion: String { get }`
- `var operatingSystemVersion: OperatingSystemVersion { get }`
- `@NSCopying var mostFeaturefulSupportedConfiguration: VZMacOSConfigurationRequirements? { get }`: The most featureful configuration both the host and this image support; an image can hold several hardware models, some unsupported by the host. `nil` if none is supported; shouldn't be `nil` for an image from `latestSupported`.

## VZMacOSConfigurationRequirements

`class VZMacOSConfigurationRequirements: NSObject`

Parameter constraints of one macOS configuration of a specific restore image.

- `@NSCopying var hardwareModel: VZMacHardwareModel { get }`: For `VZMacPlatformConfiguration.hardwareModel` and creating `VZMacAuxiliaryStorage`.
- `var minimumSupportedCPUCount: Int { get }`: Installing or running with fewer CPUs gives undefined behavior.
- `var minimumSupportedMemorySize: UInt64 { get }`: Installing or running with less memory gives undefined behavior.

## VZMacOSInstaller

`class VZMacOSInstaller: NSObject`

Installs macOS on a virtual machine from a local restore image file.

- `init(virtualMachine: VZVirtualMachine, restoringFromImageAt restoreImageFileURL: URL)`: The virtual machine's platform must be macOS and the URL a file URL for a file on disk, or an exception is raised. Call on the virtual machine's queue.
- `func install() async throws`: The virtual machine must be stopped; pausing or stopping it during installation gives undefined behavior. Installing twice with the same installer raises an exception. Call on the virtual machine's queue.
- `@preconcurrency func install(completionHandler: @escaping @Sendable (Result<Void, any Error>) -> Void)`: Completion-handler form of `install()`; the handler runs on the virtual machine's queue.
- `var progress: Progress { get }`: Observe or cancel installation. Cancelling before it starts raises an exception.
- `var virtualMachine: VZVirtualMachine { get }`
- `var restoreImageURL: URL { get }`

## VZMacOSVirtualMachineStartOptions

`class VZMacOSVirtualMachineStartOptions: VZVirtualMachineStartOptions`

For a virtual machine using the macOS boot loader.

- `var startUpFromMacOSRecovery: Bool { get set }`
- `@NSCopying var guestProvisioningOptions: VZMacGuestProvisioningOptions? { get }`: macOS 27. Set with `setGuestProvisioning(_:)`.
- `func setGuestProvisioning(_ guestProvisioningOptions: VZMacGuestProvisioningOptions?) throws`: macOS 27. Validates before setting; on failure the current options stay unchanged. `nil` removes them.

## VZMacGuestProvisioningOptions

`class VZMacGuestProvisioningOptions: VZGuestProvisioningOptions, NSCopying, Sendable`

macOS 27. Automated first-boot setup of a macOS guest: a user account and initial setup, without manual interaction. The guest must run macOS 27 or later; earlier versions ignore these settings. macOS evaluates them only on the first boot after restore; they can't reconfigure an already provisioned guest. Changing properties after the virtual machine starts has no effect.

- `init()`: macOS 27.
- `var fullName: String { get set }`: macOS 27.
- `var username: String { get set }`: macOS 27. The login name.
- `var password: String { get set }`: macOS 27.
- `var logsInAutomatically: Bool { get set }`: macOS 27. At startup.
- `var enablesRemoteLogin: Bool { get set }`: macOS 27. Remote Login (SSH). Defaults to `false`.
