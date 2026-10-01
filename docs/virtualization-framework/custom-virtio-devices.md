# Custom Virtio Devices

Define a Virtio device the guest discovers and your code implements: configure it, receive it when the virtual machine is created, then service its virtqueues, shared memory regions, and lifecycle events. Everything here is macOS 27+.

## VZCustomVirtioDeviceConfiguration

`class VZCustomVirtioDeviceConfiguration: NSObject, NSCopying`

Defines a custom Virtio device. The framework creates a `VZCustomVirtioDevice` from it when the `VZVirtualMachine` is created. Set `deviceID`, `pciClassID`, `pciSubclassID`, and `virtioQueueCount` so the guest can discover the device.

- `var deviceID: UInt16 { get set }`: the Virtio device ID.
- `var pciClassID: UInt8 { get set }`
- `var pciSubclassID: UInt8 { get set }`
- `var virtioQueueCount: UInt16 { get set }`
- `var mandatoryFeatures: VZVirtioFeatureSet { get }`: features offered that the driver must accept, or the device fails to initialize. `VIRTIO_F_VERSION_1` is always set.
- `var optionalFeatures: VZVirtioFeatureSet { get }`: features the guest may decline; see `VZCustomVirtioDevice.negotiatedFeatures`. A few bits are always set for performance.
- `var deviceSpecificConfiguration: VZVirtioDeviceSpecificConfiguration? { get set }`
- `var provider: VZCustomVirtioDeviceProvider? { get set }`: how the device is implemented.
- `class var maximumAllowedSharedMemoryRegionCount: Int { get }`
- `var sharedMemoryRegions: [VZVirtioSharedMemoryRegionConfiguration] { get set }`: empty by default.
- `var supportsSaveRestore: Bool { get set }`: `false` by default. Setting `true` when the device's delegate doesn't implement the save and restore methods raises an exception.

## VZCustomVirtioDeviceProvider

`class VZCustomVirtioDeviceProvider: NSObject`

Base class for how a custom device is implemented. Don't create it; use a subclass such as `VZCustomVirtioDeviceDelegateProvider`.

## VZCustomVirtioDeviceDelegateProvider

`class VZCustomVirtioDeviceDelegateProvider: VZCustomVirtioDeviceProvider`

Provides devices implemented by a `VZCustomVirtioDeviceConfigurationDelegate`, which runs in the same process as the guest's `VZVirtualMachine`.

- `init(deviceQueue: dispatch_queue_t, delegate: any VZCustomVirtioDeviceConfigurationDelegate)`: the framework synchronizes all device operations on `deviceQueue`.
- `var deviceQueue: dispatch_queue_t { get }`
- `weak var delegate: (any VZCustomVirtioDeviceConfigurationDelegate)? { get }`

## VZCustomVirtioDeviceConfigurationDelegate

`protocol VZCustomVirtioDeviceConfigurationDelegate : NSObjectProtocol`

Tracks the creation of a custom Virtio device from its configuration.

- `optional func customVirtioConfiguration(_ deviceConfiguration: VZCustomVirtioDeviceConfiguration, didCreateDevice device: VZCustomVirtioDevice)`: called on the virtual machine's serial queue during `VZVirtualMachine` initialization. Set the device's `delegate` here to miss no calls.

## VZCustomVirtioDevice

`class VZCustomVirtioDevice: NSObject`

Implemented by you; the framework creates it and passes it to the configuration delegate. All operations on the device and its delegate run on `deviceQueue`.

- `weak var delegate: (any VZCustomVirtioDeviceDelegate)? { get set }`
- `var deviceQueue: dispatch_queue_t { get }`: a serial queue; the virtual machine's `queue` if not set.
- `var negotiatedFeatures: VZNegotiatedVirtioFeatureSet? { get }`: `nil` before negotiation; valid only after `customVirtioDeviceDidAcceptDriverOk(_:)`.
- `var sharedMemoryRegions: [VZVirtioSharedMemoryRegion] { get }`: exposed to the guest.
- `func guestMemoryMapping(atPhysicalAddress physicalAddress: UInt64, length: Int) -> VZGuestMemoryMapping?`: `nil` if the address and length don't reference a valid guest RAM region.
- `func queue(at index: UInt16) -> VZVirtioQueue?`: queues are set up when the driver sets `DRIVER_OK`, so valid only after `customVirtioDeviceDidAcceptDriverOk(_:)`. `nil` if the index is invalid or the guest disabled the queue.
- `func requestReset()`: sets `DEVICE_NEEDS_RESET`; the guest may ignore it.
- `func update(_ newConfiguration: VZVirtioDeviceSpecificConfiguration, completionHandler: @escaping @Sendable ((any Error)?) -> Void)`: the new data must match the old size, or an exception is raised.
- `func update(_ newConfiguration: VZVirtioDeviceSpecificConfiguration) async throws`

## VZCustomVirtioDeviceDelegate

`protocol VZCustomVirtioDeviceDelegate : NSObjectProtocol`

Receives a custom device's lifecycle events and queue notifications. The device is ready once the driver sets `DRIVER_OK`.

- `optional func customVirtioDevice(_ device: VZCustomVirtioDevice, didReceiveNotificationFor queue: VZVirtioQueue)`: the guest notified `queue`; call `nextElement()` until it returns `nil`.
- `optional func customVirtioDeviceDidAcceptDriverOk(_ device: VZCustomVirtioDevice)`: negotiation completed and the driver set `DRIVER_OK`; negotiated features and queues are now valid.
- `optional func customVirtioDeviceWillStop(_ device: VZCustomVirtioDevice)`: the virtual machine stopped, by guest shutdown or a stop request.
- `optional func customVirtioDeviceWillPause(_ device: VZCustomVirtioDevice)`: the virtual machine paused.
- `optional func customVirtioDeviceWillResume(_ device: VZCustomVirtioDevice)`: the virtual machine resumed.
- `optional func customVirtioDeviceWillReset(_ device: VZCustomVirtioDevice)`: a reset is due, whether you requested it or the guest did.
- `optional func customVirtioDeviceSaveState(forRestore device: VZCustomVirtioDevice) -> Data?`: the state to save, or empty `Data` if none; `nil` fails the save.
- `optional func customVirtioDeviceShouldRestore(_ device: VZCustomVirtioDevice, saveState: Data) -> Bool`: `saveState` is what the save method returned; return `false` if the restore failed.

## VZVirtioFeatureSet

`class VZVirtioFeatureSet: NSObject, NSCopying`

A 64-bit Virtio feature set split into two 32-bit subsets. Get it from the configuration's `mandatoryFeatures` and `optionalFeatures`; don't create it.

- `var subset0: UInt32 { get set }`: feature bits 0 through 31.
- `var subset1: UInt32 { get set }`: feature bits 32 through 63.

## VZNegotiatedVirtioFeatureSet

`class VZNegotiatedVirtioFeatureSet: NSObject`

Feature bits the device and driver negotiated. Get it from `VZCustomVirtioDevice.negotiatedFeatures`; don't create it.

- `var subset0: UInt32 { get }`: feature bits 0 through 31.
- `var subset1: UInt32 { get }`: feature bits 32 through 63.

## VZVirtioDeviceSpecificConfiguration

`class VZVirtioDeviceSpecificConfiguration: NSObject, NSCopying`

Device-specific configuration, serialized as `Data`. Set it on `VZCustomVirtioDeviceConfiguration.deviceSpecificConfiguration`.

- `init(configurationData: Data)`
- `var configurationData: Data { get }`

## VZVirtioSharedMemoryRegionConfiguration

`class VZVirtioSharedMemoryRegionConfiguration: NSObject, NSCopying`

A memory region continuously shared between the device implementation and the guest, which the framework advertises to the guest. The ID's meaning is device-specific.

- `init(regionID: UInt8, size: UInt64)`
- `var regionID: UInt8 { get }`
- `var size: UInt64 { get }`

## VZVirtioSharedMemoryRegion

`class VZVirtioSharedMemoryRegion: NSObject`

A running custom device's shared memory region, into which you map and unmap host memory. Get it from `VZCustomVirtioDevice.sharedMemoryRegions`; don't create it.

- `var regionID: UInt8 { get }`
- `var size: UInt64 { get }`
- `func mapMemory(_ memory: UnsafeMutableRawPointer, atOffset offset: UInt64, size: UInt64, completionHandler: @escaping @Sendable ((any Error)?) -> Void)`: `memory`, `offset`, and `size` must be host-page-aligned. The handler runs on `VZCustomVirtioDevice.deviceQueue`, with `nil` on success.
- `func mapMemory(_ memory: UnsafeMutableRawPointer, atOffset offset: UInt64, size: UInt64) async throws`
- `func unmapMemory(atOffset offset: UInt64, size: UInt64, completionHandler: @escaping @Sendable ((any Error)?) -> Void)`: `offset` and `size` must be host-page-aligned. The handler runs on `deviceQueue`, with `nil` on success.
- `func unmapMemory(atOffset offset: UInt64, size: UInt64) async throws`

## VZVirtioQueue

`class VZVirtioQueue: NSObject`

A virtqueue belonging to a custom device. Get it from `VZCustomVirtioDevice.queue(at:)` or the notification delegate method; don't create it.

- `func nextElement() -> VZVirtioQueueElement?`: `nil` if no element is available or the device is no longer live. Notifications stay disabled until every available element is consumed, so loop until `nil`. Return each element with `returnToQueue()`.
- `var queueIndex: UInt16 { get }`
- `var queueSize: UInt16 { get }`: the most elements the queue can hold.

## VZVirtioQueueElement

`class VZVirtioQueueElement: NSObject`

A unit of work on a virtqueue (a descriptor chain), exposing guest memory as host-read-only read buffers and host-write-only write buffers. Access each buffer only once: the guest can change its memory at any time, so repeated reads can differ and open time-of-check to time-of-use vulnerabilities. The read and write methods enforce single access and update the available byte counts. Get elements from `VZVirtioQueue.nextElement()`; don't create them.

- `var readBuffersByteCount: Int { get }`
- `var readBuffersAvailableByteCount: Int { get }`: starts equal to `readBuffersByteCount` and decreases as you read.
- `var writeBuffersByteCount: Int { get }`
- `var writeBuffersAvailableByteCount: Int { get }`: starts equal to `writeBuffersByteCount` and decreases as you write.
- `var writtenByteCount: Int { get }`: starts at 0 and increases as you write.
- `func readBuffers() -> [Data]`: the remaining read memory as scatter-gather buffers referencing guest memory without copying; `readBuffersAvailableByteCount` becomes 0.
- `func peekIntoReadBuffers(withExactLength exactLength: Int) throws -> Data`: copies without consuming, leaving `readBuffersAvailableByteCount` unchanged. Allows repeated reads, with the risk above.
- `func readBytes(withExactLength exactLength: Int) throws -> Data`: copies and consumes the bytes.
- `func readBytes(intoBuffer buffer: UnsafeMutableRawPointer, exactLength: Int) throws`: consumes the bytes.
- `func write(_ data: Data) throws`
- `func writeBuffer(_ buffer: UnsafeMutableRawPointer, exactLength: Int) throws`
- `func returnToQueue()`: returns the element to the guest. The element must come from `nextElement()`; returning it twice raises an exception.
