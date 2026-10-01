# Essentials

Core types: the virtual machine, its configuration, lifecycle delegate, display view, error codes, and base types for guest provisioning and guest memory. Creating a virtual machine or a valid configuration requires the `com.apple.security.virtualization` entitlement.

## VZVirtualMachine

`class VZVirtualMachine: NSObject`

A single virtual machine's entire state, created from a configuration and operated on one serial queue.

- `convenience init(configuration: VZVirtualMachineConfiguration)`: Uses the main queue: every call must be made on it, and callbacks arrive on it. The configuration must be valid and is copied.
- `init(configuration: VZVirtualMachineConfiguration, queue: dispatch_queue_t)`: Every operation must happen on `queue`, and callbacks and delegate methods are invoked on it. `queue` must be serial, or behavior is undefined. The configuration must be valid and is copied.
- `nonisolated var queue: dispatch_queue_t { get }`: The creation queue, or the main queue if none was given. Readable from any queue or actor; everything else on the machine must be used on this queue.
- `class var isSupported: Bool { get }`: When false, no configuration validates; the validation error explains why.
- `var state: VZVirtualMachine.State { get }`
- `weak var delegate: (any VZVirtualMachineDelegate)? { get set }`
- `var canStart: Bool { get }`
- `var canStop: Bool { get }`
- `var canPause: Bool { get }`
- `var canResume: Bool { get }`
- `var canRequestStop: Bool { get }`: Whether the guest can be asked to stop.
- `var consoleDevices: [VZConsoleDevice] { get }`: Empty if none are configured.
- `var directorySharingDevices: [VZDirectorySharingDevice] { get }`: Empty if none are configured.
- `var graphicsDevices: [VZGraphicsDevice] { get }`: Empty if none are configured.
- `var memoryBalloonDevices: [VZMemoryBalloonDevice] { get }`: Empty if none are configured.
- `var networkDevices: [VZNetworkDevice] { get }`: Empty if none are configured.
- `var socketDevices: [VZSocketDevice] { get }`: Empty if none are configured.
- `var usbControllers: [VZUSBController] { get }`: Empty if none are configured. One per entry in the configuration's `usbControllers`.
- `func start(completionHandler: @escaping (Result<Void, any Error>) -> Void)`: Starts a machine that is stopped or in the error state.
- `func start() async throws`: Async form of `start(completionHandler:)`.
- `func start(options: VZVirtualMachineStartOptions, completionHandler: @escaping ((any Error)?) -> Void)`: As `start(completionHandler:)`, with options. The handler receives nil on success.
- `func start(options: VZVirtualMachineStartOptions) async throws`: Async form of `start(options:completionHandler:)`.
- `func stop(completionHandler: @escaping ((any Error)?) -> Void)`: Stops a running or paused machine without letting the guest stop cleanly. Fails with an invalid state transition error if the guest's state conflicts, such as the guest already stopped. An internal error moves the machine to the error state; inspect failures with `state` and the delegate.
- `func stop() async throws`: Async form of `stop(completionHandler:)`.
- `func pause(completionHandler: @escaping (Result<Void, any Error>) -> Void)`: Pauses a running machine. Fails with an invalid state transition error if the guest's state conflicts. An internal error moves the machine to the error state.
- `func pause() async throws`: Async form of `pause(completionHandler:)`.
- `func resume(completionHandler: @escaping (Result<Void, any Error>) -> Void)`: Resumes a paused machine.
- `func resume() async throws`: Async form of `resume(completionHandler:)`.
- `func requestStop() throws`: Asks the guest to turn itself off; the delegate's `guestDidStop(_:)` is invoked when it does.
- `func restoreMachineStateFrom(url saveFileURL: URL, completionHandler: @escaping ((any Error)?) -> Void)`: Apple silicon only. Restores a stopped machine from a file `saveMachineStateTo(url:completionHandler:)` wrote on the same host; on success the machine is paused. Fails with `VZError.Code.invalidVirtualMachineStateTransition` if the machine isn't stopped; with `VZError.Code.restore` for an unreadable or invalid file, a file from another host or newer software, or a configuration not matching the file; with `VZError.Code.internalError` on an internal failure to start. The configuration must pass `validateSaveRestoreSupport()`. On failure the state is unchanged. After a host software update the file may be rejected as an invalid argument; start the machine normally instead.
- `func restoreMachineStateFrom(url saveFileURL: URL) async throws`: Apple silicon only. Async form of `restoreMachineStateFrom(url:completionHandler:)`.
- `func saveMachineStateTo(url saveFileURL: URL, completionHandler: @escaping ((any Error)?) -> Void)`: Apple silicon only. Writes a paused machine's state to a restorable file, encrypted with a host-tied key. Fails with `VZError.Code.invalidVirtualMachineState` if the machine isn't paused, and `VZError.Code.save` for any other failure. The configuration must pass `validateSaveRestoreSupport()`. The machine's state is unchanged either way.
- `func saveMachineStateTo(url saveFileURL: URL) async throws`: Apple silicon only. Async form of `saveMachineStateTo(url:completionHandler:)`.

## VZVirtualMachine.State

`enum State`

A virtual machine's execution state. Raw values are `Int`.

- `case stopped`: 0. Initial state, before starting.
- `case running`: 1.
- `case paused`: 2. Reached only from `pausing`.
- `case error`: 3. After an internal error.
- `case starting`: 4. Configuring hardware and starting.
- `case pausing`: 5. Between running and paused.
- `case resuming`: 6. Between paused and running.
- `case stopping`: 7. Between running and stopped.
- `case saving`: 8. Between paused and paused.
- `case restoring`: 9. Between stopped and paused on success, or stopped on failure.
- `init?(rawValue: Int)`

## VZVirtualMachineConfiguration

`class VZVirtualMachineConfiguration: NSObject, NSCopying`

Defines a `VZVirtualMachine`: boot loader, CPU count, memory, platform, and devices. Each device is a configuration (the emulated hardware) plus an attachment (the host resource it exposes).

To run macOS, set a `VZMacPlatformConfiguration` as `platform` and a `VZMacOSBootLoader` as `bootLoader`; set `cpuCount` and `memorySize` from the guest's `VZMacOSConfigurationRequirements`; put the main storage device first in `storageDevices`; and set `keyboards`, `pointingDevices`, and `graphicsDevices`. To run Linux, set a `VZLinuxBootLoader`, `cpuCount`, and `memorySize`, then any devices needed.

- `var bootLoader: VZBootLoader? { get set }`
- `var memorySize: UInt64 { get set }`: Bytes; the guest's total physical memory, allocated on demand. Must be a multiple of 1 MiB between `minimumAllowedMemorySize` and `maximumAllowedMemorySize`.
- `var cpuCount: Int { get set }`: Must be between `minimumAllowedCPUCount` and `maximumAllowedCPUCount`.
- `var platform: VZPlatformConfiguration { get set }`: A `VZGenericPlatformConfiguration` (the default) or a `VZMacPlatformConfiguration`. When restoring saved state it must match the saved machine.
- `var audioDevices: [VZAudioDeviceConfiguration] { get set }`: Empty by default.
- `var consoleDevices: [VZConsoleDeviceConfiguration] { get set }`: Empty by default.
- `var customVirtioDevices: [VZCustomVirtioDeviceConfiguration] { get set }`: Empty by default. macOS 27.
- `var directorySharingDevices: [VZDirectorySharingDeviceConfiguration] { get set }`: Empty by default.
- `var entropyDevices: [VZEntropyDeviceConfiguration] { get set }`: Empty by default.
- `var memoryBalloonDevices: [VZMemoryBalloonDeviceConfiguration] { get set }`: Empty by default.
- `var networkDevices: [VZNetworkDeviceConfiguration] { get set }`: Empty by default.
- `var serialPorts: [VZSerialPortConfiguration] { get set }`: Empty by default.
- `var socketDevices: [VZSocketDeviceConfiguration] { get set }`: Empty by default.
- `var storageDevices: [VZStorageDeviceConfiguration] { get set }`: Empty by default.
- `var keyboards: [VZKeyboardConfiguration] { get set }`: Empty by default.
- `var pointingDevices: [VZPointingDeviceConfiguration] { get set }`: Empty by default.
- `var graphicsDevices: [VZGraphicsDeviceConfiguration] { get set }`: Empty by default.
- `var usbControllers: [VZUSBControllerConfiguration] { get set }`: Empty by default. Each entry produces a runtime object in `VZVirtualMachine.usbControllers`.
- `var label: String? { get set }`: Part of the machine's name in some system services. Must be non-empty, at most 64 characters, and contain at least one non-whitespace character. macOS 27.
- `func validate() throws`
- `func validateSaveRestoreSupport() throws`: Apple silicon only. Checks that a machine with this configuration can be saved and restored; if it throws, expect `saveMachineStateTo` to fail.
- `class var minimumAllowedMemorySize: UInt64 { get }`
- `class var maximumAllowedMemorySize: UInt64 { get }`
- `class var minimumAllowedCPUCount: Int { get }`
- `class var maximumAllowedCPUCount: Int { get }`

## VZVirtualMachineDelegate

`protocol VZVirtualMachineDelegate : NSObjectProtocol`

Optional callbacks for tracking a virtual machine's state, invoked on the machine's queue.

- `optional func guestDidStop(_ virtualMachine: VZVirtualMachine)`: The guest stopped the machine.
- `optional func virtualMachine(_ virtualMachine: VZVirtualMachine, didStopWithError error: any Error)`
- `optional func virtualMachine(_ virtualMachine: VZVirtualMachine, networkDevice: VZNetworkDevice, attachmentWasDisconnectedWithError error: any Error)`: Invoked each time the network interface fails to start, such as at boot, device reset, or reboot, so possibly several times per machine. The device's `attachment` is nil afterward.

## VZVirtualMachineStartOptions

`class VZVirtualMachineStartOptions: NSObject`

Base class for options passed to `VZVirtualMachine.start(options:completionHandler:)`; its subclass is `VZMacOSVirtualMachineStartOptions`.

## VZVirtualMachineView

`class VZVirtualMachineView: NSView`

An AppKit view that shows the machine's framebuffer and, if the configuration has a keyboard and a pointing device, forwards keyboard and mouse events to them.

- `var virtualMachine: VZVirtualMachine? { get set }`: Setting it sets `adaptor` to nil.
- `@MainActor @preconcurrency var adaptor: VZVirtualMachineViewAdaptor? { get set }`: A Sendable alternative for assigning a machine across isolation boundaries in Swift 6. Setting it resets `virtualMachine` to nil. macOS 27.
- `var capturesSystemKeys: Bool { get set }`: Whether certain system hot keys go to the guest instead of the host. Defaults to false.
- `var automaticallyReconfiguresDisplay: Bool { get set }`: Reconfigures the graphics display to match the host display environment as the view changes, such as during live resize. Defaults to false. Only one view per `VZGraphicsDisplay` honors it; enabling it on a second view disables it on the first.

## VZVirtualMachineViewAdaptor

`struct VZVirtualMachineViewAdaptor: @unchecked Sendable`

Connects a `VZVirtualMachineView` to a machine from another isolation domain. Holds a strong reference to the machine, so it can be passed across actors and assigned to the view's `adaptor`. macOS 27.

- `init(virtualMachine: VZVirtualMachine)`: shows the first display of the machine's first graphics device. A machine with no graphics devices still makes an adaptor; the view then has no display.

## VZGuestMemoryMapping

`class VZGuestMemoryMapping: NSObject`

A chunk of the guest's DRAM that a `VZCustomVirtioDevice` can read and write. Obtained only from `VZCustomVirtioDevice.guestMemoryMapping(atPhysicalAddress:length:)`. It references all host memory allocated for guest DRAM, which a reboot or shutdown invalidates and remaps, so a mapping can't be used across one. macOS 27.

- `var mutableBytes: UnsafeMutableRawPointer { get }`
- `var physicalAddress: UInt64 { get }`: Guest physical base address.
- `var length: Int { get }`: Bytes.

## VZGuestProvisioningOptions

`class VZGuestProvisioningOptions: NSObject, NSCopying`

Base class for guest provisioning options; use a subclass such as `VZMacGuestProvisioningOptions`. macOS 27.

- `func validate() throws`: On failure the error is a `VZError` with a guest provisioning code.

## VZError

`struct VZError: Error, Sendable`

The framework's error type, in the domain `VZErrorDomain`. Errors from lower-level components can carry other domains.

- `let VZErrorDomain: String`
- `static var errorDomain: String { get }`
- `var code: Self.Code { get }`
- `var userInfo: [String : Any] { get }`
- `init(_ code: Self.Code, userInfo: [String : Any] = [:])`
- `enum Code`: Raw values are `Int`. Each code is also a static property of `VZError` with the same name, for example `static var internalError: VZError.Code { get }`.

## VZError.Code

`enum Code`

- `case internalError`: 1. Such as the machine unexpectedly stopping.
- `case invalidVirtualMachineConfiguration`: 2.
- `case invalidVirtualMachineState`: 3. Wrong state for the operation, such as interacting with a stopped or paused machine.
- `case invalidVirtualMachineStateTransition`: 4. Such as starting a machine whose `canStart` is false.
- `case invalidDiskImage`: 5. Unrecognized format, damaged, or invalid.
- `case virtualMachineLimitExceeded`: 6. Starting would exceed the system's limit on simultaneously running machines.
- `case networkError`: 7. Such as a failed connection.
- `case outOfDiskSpace`: 8. On the host, such as while installing Rosetta.
- `case operationCancelled`: 9. Cancelled by the framework.
- `case notSupported`: 10.
- `case save`: 11. Failed to save to the save file.
- `case restore`: 12.
- `case restoreImageCatalogLoadFailed`: 10001.
- `case invalidRestoreImageCatalog`: 10002.
- `case noSupportedRestoreImagesInCatalog`: 10003.
- `case restoreImageLoadFailed`: 10004.
- `case invalidRestoreImage`: 10005.
- `case installationRequiresUpdate`: 10006. A software update is needed to complete it.
- `case installationFailed`: 10007.
- `case networkBlockDeviceNegotiationFailed`: 20001. Connecting to or negotiating with the Network Block Device server failed.
- `case networkBlockDeviceDisconnected`: 20002. The Network Block Device client disconnected from the server.
- `case usbControllerNotFound`: 30001.
- `case deviceAlreadyAttached`: 30002.
- `case deviceInitializationFailure`: 30003.
- `case deviceNotFound`: 30004.
- `case guestProvisioningInvalidFullName`: 40001. macOS 27.
- `case guestProvisioningInvalidUsername`: 40002. macOS 27.
- `case guestProvisioningInvalidPassword`: 40003. macOS 27.
- `case efiSecureBootEnrollmentFailed`: 50001. Secure Boot signatures failed to enroll. macOS 27.
- `case efiVariableInaccessible`: 50002. The EFI variable store is inaccessible. macOS 27.
- `init?(rawValue: Int)`
