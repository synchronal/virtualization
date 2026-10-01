# Entropy, memory balloon, and USB

Entropy devices feed the guest's random number generator, memory balloon devices let the host reclaim and return guest memory, and USB controllers host hot-pluggable USB devices, including host passthrough devices.

## VZEntropyDeviceConfiguration

`class VZEntropyDeviceConfiguration: NSObject, NSCopying`

Base class; use `VZVirtioEntropyDeviceConfiguration`.

## VZVirtioEntropyDeviceConfiguration

`class VZVirtioEntropyDeviceConfiguration: VZEntropyDeviceConfiguration`

Exposes a source of entropy for the guest's random number generator.

- `init()`

## VZMemoryBalloonDevice

`class VZMemoryBalloonDevice: NSObject`

Base class for a runtime memory balloon device, in `VZVirtualMachine.memoryBalloonDevices`. Its real type matches the configuration: a `VZVirtioTraditionalMemoryBalloonDeviceConfiguration` produces a `VZVirtioTraditionalMemoryBalloonDevice`.

## VZMemoryBalloonDeviceConfiguration

`class VZMemoryBalloonDeviceConfiguration: NSObject, NSCopying`

Base class; use `VZVirtioTraditionalMemoryBalloonDeviceConfiguration`.

## VZVirtioTraditionalMemoryBalloonDeviceConfiguration

`class VZVirtioTraditionalMemoryBalloonDeviceConfiguration: VZMemoryBalloonDeviceConfiguration`

At most one per virtual machine.

- `init()`

## VZVirtioTraditionalMemoryBalloonDevice

`class VZVirtioTraditionalMemoryBalloonDevice: VZMemoryBalloonDevice`

A primitive device that transfers memory between host and guest per the Virtio specification, letting the guest adapt to changes in its available physical memory.

- `var targetVirtualMachineMemorySize: UInt64 { get set }`: bytes of physical memory available to the guest. Starts at `VZVirtualMachineConfiguration.memorySize`. Should be a multiple of 1 MiB between `VZVirtualMachineConfiguration.minimumAllowedMemorySize` and `memorySize`; other values are rounded down to a 1 MiB multiple and clamped to that range. Lowering it takes memory from the guest; raising it gives memory back. A write is only a request: it takes effect once the guest operating system handles it, if at all. For best results, compact guest memory first (e.g. `echo 1 > /proc/sys/vm/compact_memory`).

## VZUSBController

`class VZUSBController: NSObject`

A runtime USB controller, in `VZVirtualMachine.usbControllers`. Its real type matches the configuration: a `VZXHCIControllerConfiguration` produces a `VZXHCIController`.

- `func attach(device: any VZUSBDevice, completionHandler: @escaping ((any Error)?) -> Void)`: on success the device appears in `usbDevices` and its `usbController` is this controller. Fails with `VZErrorDeviceAlreadyAttached` if the device was ever attached to any controller, `VZErrorDeviceInitializationFailure` if it cannot be initialized. Call on the virtual machine's queue; the handler runs there.
- `func attach(device: any VZUSBDevice) async throws`
- `func detach(device: any VZUSBDevice, completionHandler: @escaping ((any Error)?) -> Void)`: on success the device leaves `usbDevices` and its `usbController` becomes nil. Fails with `VZErrorDeviceNotFound` if it was not attached to this controller. Call on the virtual machine's queue; the handler runs there.
- `func detach(device: any VZUSBDevice) async throws`
- `var usbDevices: [any VZUSBDevice] { get }`: attached devices; those in the controller configuration's `usbDevices` appear once the virtual machine starts.
- `weak var delegate: (any VZUSBController.Delegate)? { get set }`: macOS 27.

## VZUSBController.Delegate

`protocol Delegate : NSObjectProtocol`

Receives USB controller state-change callbacks. macOS 27.

- `optional func usbController(_ usbController: VZUSBController, usbPassthroughDeviceDidDisconnect device: VZUSBPassthroughDevice)`: called when the device's IOService terminates, after the framework has detached the device and removed it from `usbDevices`.

## VZUSBControllerConfiguration

`class VZUSBControllerConfiguration: NSObject, NSCopying`

Base class; use `VZXHCIControllerConfiguration`.

- `var usbDevices: [any VZUSBDeviceConfiguration] { get set }`: devices the virtual machine starts with; empty by default. Each produces a runtime object in `VZUSBController.usbDevices`.

## VZUSBDevice

`protocol VZUSBDevice : NSObjectProtocol`

A hot-pluggable USB device in a virtual machine. Only framework objects should conform.

- `weak var usbController: VZUSBController? { get }`: nil when detached.
- `var uuid: UUID { get }`: the UUID from the device's `VZUSBDeviceConfiguration`.

## VZUSBDeviceConfiguration

`protocol VZUSBDeviceConfiguration : NSObjectProtocol`

A hot-pluggable USB device configuration. Only framework objects should conform.

- `var uuid: UUID { get set }`: generated automatically. Before restoring a virtual machine, set it to the UUID the device had when the machine was saved.

## VZUSBPassthroughDevice

`class VZUSBPassthroughDevice: NSObject, VZUSBDevice`

Create directly and pass to `VZUSBController.attach(device:completionHandler:)`, or add a `VZUSBPassthroughDeviceConfiguration` to a `VZVirtualMachineConfiguration` to make it appear in `VZUSBController.usbDevices`. macOS 27.

- `init(configuration: VZUSBPassthroughDeviceConfiguration) throws`

## VZUSBPassthroughDeviceConfiguration

`class VZUSBPassthroughDeviceConfiguration: NSObject, NSCopying, VZUSBDeviceConfiguration`

A host USB device, made accessible to a virtual machine by capturing it: when the machine starts with this configuration in a `VZUSBControllerConfiguration`, or when `VZUSBController.attach(device:completionHandler:)` is called on a running machine. macOS 27.

- `init(device: AAUSBAccessory)`: the AccessoryAccess object to capture.

## VZXHCIController

`class VZXHCIController: VZUSBController`

Cannot be created directly: add a `VZXHCIControllerConfiguration` to `VZVirtualMachineConfiguration.usbControllers`.

## VZXHCIControllerConfiguration

`class VZXHCIControllerConfiguration: VZUSBControllerConfiguration`

- `init()`
