# AccessoryAccess

Apple's AccessoryAccess framework as of the macOS 27 SDK, in Swift. Macs only; every symbol is macOS 27+.

AccessoryAccess gives an app access to connected USB devices that use the IOUSBHost framework (`docs/iousbhost/`). A client registers one or more listeners with matching criteria and is told when matching accessories connect and disconnect. An `AAUSBAccessory` is what Virtualization's `VZUSBPassthroughDeviceConfiguration.init(device:)` captures for a virtual machine.

Entitlement: `com.apple.developer.accessory-access.usb` ("Claim USB Accessory", added in Xcode under Signing & Capabilities). Without it, every AccessoryAccess call fails with `AAError.Code.internalError` and the failure reason "Unable to communicate with service". Apple's pages don't say whether it needs a provisioning profile, and name no Info.plist keys.

`AAUSBAccessoryManager` presents UI for the app, so only an app with a UI (one in the Dock) can use it.

## AAUSBAccessoryManager

`class AAUSBAccessoryManager` (`NSObject`, `Sendable`)

Manages USB accessories and the listeners registered for them. Don't instantiate it; use `shared`. Notifications reach listeners on the manager's internal serial queue.

- `class var shared: AAUSBAccessoryManager { get }`: the process's manager.
- `func registerListener(_ listener: any AAUSBAccessoryListener, matchingCriteria: [AAUSBAccessoryMatchingCriteria], completionHandler: @escaping @Sendable ([AAUSBAccessory], (any Error)?) -> Void)`: the listener hears about accessories that satisfy any of the criteria; an empty array matches any accessory. The handler gets the already-connected matching accessories (empty if none) and `nil` on success; it runs on an arbitrary thread. Fails with `AAError.Code.accessoryListenerAlreadyRegistered` if the listener is already registered.
- `func registerListener(_ listener: any AAUSBAccessoryListener, matchingCriteria: [AAUSBAccessoryMatchingCriteria]) async throws -> [AAUSBAccessory]`
- `func unregisterListener(_ listener: any AAUSBAccessoryListener, completionHandler: @escaping @Sendable () -> Void)`: the handler runs on an arbitrary thread once the listener is unregistered.
- `func unregisterListener(_ listener: any AAUSBAccessoryListener) async`
- `func events(matching criteria: [AAUSBAccessoryMatchingCriteria]) async throws -> some AsyncSequence<AAUSBAccessory.Event, Never>`: connection and disconnection events for accessories matching the criteria.

## AAUSBAccessory

`class AAUSBAccessory` (`NSObject`, `NSSecureCoding`, `Sendable`)

A USB accessory. Obtained from `AAUSBAccessoryListener.usbAccessoryDidConnect(_:)` (or `registerListener`'s handler), or decoded from an XPC representation. It may arrive unconfigured: open it, then select a configuration with `IOUSBHostDevice.configureWithValue:error:`.

- `init?(xpcRepresentation: xpc_object_t)`: nil if the object isn't a valid `AAUSBAccessory` representation.
- `init?(XPCRepresentation xpcRepresentation: xpc_object_t)`: the same initializer, Objective-C-named; unavailable in Swift.
- `init?(coder: NSCoder)`
- `var configurationDescriptorData: Data? { get }`: the selected configuration descriptor, bytes of an `IOUSBConfigurationDescriptor`; nil if the accessory isn't configured.
- `var deviceDescriptorData: Data { get }`: the device descriptor, bytes of an `IOUSBDeviceDescriptor`.
- `var registryID: UInt64 { get }`: the accessory's IORegistry ID.
- `func open(serviceQueue: dispatch_queue_t?, completionHandler: @escaping @Sendable (IOUSBHostDevice, (any Error)?) -> Void)`: opens the accessory for this process's exclusive access and gives its `IOUSBHostDevice`. `serviceQueue` is a serial queue servicing the asynchronous requests the app submits to the default control endpoint; nil makes the framework create an internal one. Calling it again gives the same `IOUSBHostDevice`. The handler's error is nil on success; it runs on an arbitrary thread.
- `func open(serviceQueue: dispatch_queue_t?) async throws -> IOUSBHostDevice`
- `func close(completionHandler: @escaping @Sendable ((any Error)?) -> Void)`: closes all of this process's connections to the accessory and invalidates every `IOUSBHostDevice` it returned, the same as `IOUSBHostDevice.destroy()`. Afterwards this or another worker process of the app can open it again. The handler's error is nil on success; it runs on an arbitrary thread. Calling `IOUSBHostDevice.destroy()` blocks, and deadlocks if called from a completion handler.
- `func close() async throws`
- `func createXPCRepresentation() -> xpc_object_t`: an encoding to pass to an XPC service over an XPC connection.

## AAUSBAccessory.Event

`enum Event` (`Sendable`)

An accessory connecting or disconnecting, as `events(matching:)` yields.

- `case didConnect(AAUSBAccessory)`
- `case didDisconnect(AAUSBAccessory)`

## AAUSBAccessoryListener

`protocol AAUSBAccessoryListener : NSObjectProtocol, Sendable`

Told when matching accessories connect and disconnect. The overview says the methods run on the manager's internal serial queue; each method's page says an arbitrary thread.

- `optional func usbAccessoryDidConnect(_ usbAccessory: AAUSBAccessory)`: every time an accessory matching the listener's registered criteria connects.
- `optional func usbAccessoryDidDisconnect(_ usbAccessory: AAUSBAccessory)`: every time an accessory the listener was told connected disconnects.

## AAUSBAccessoryMatchingCriteria

`class AAUSBAccessoryMatchingCriteria` (`NSObject`, `NSCopying`, `Sendable`)

What a listener registers for. Dictionaries are made with `IOUSBHostDevice.createMatchingDictionaryWithVendorID:…` and `IOUSBHostInterface.createMatchingDictionaryWithVendorID:…`; their keys are in IOKit's `USBSpec.h`.

- `init?(deviceMatchingDictionary dictionary: [String : any Sendable])`: a USB device matching dictionary.
- `init?(deviceMatchingDictionary: [String : any Sendable]?, interfaceMatchingDictionaries: [[String : any Sendable]], interfaceMatchingOption: AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption)`: matches an accessory that matches the non-nil device dictionary and, in its current configuration, has interfaces matching all (`matchAll`) or any (`matchAny`) of the interface dictionaries.
- `convenience init(deviceMatchingCriteria: AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria)`
- `convenience init(deviceMatchingCriteria: AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria?, interfaceMatchingCriteria: [AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria], interfaceMatchingOption: AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption)`: as the dictionary form.

## AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria

`struct DeviceMatchingCriteria` (`Sendable`)

The device characteristics to match.

- `init(vendorID: Int? = nil, productID: Int? = nil, deviceClass: Int? = nil, deviceSubClass: Int? = nil, deviceProtocol: Int? = nil, speed: Int? = nil)`
- `var vendorID: Int?`
- `var productID: Int?`
- `var deviceClass: Int?`
- `var deviceSubClass: Int?`
- `var deviceProtocol: Int?`
- `var speed: Int?`

## AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria

`struct InterfaceMatchingCriteria` (`Sendable`)

The interface characteristics to match.

- `init(vendorID: Int? = nil, productID: Int? = nil, bcdDevice: Int? = nil, interfaceNumber: Int? = nil, configurationValue: Int? = nil, interfaceClass: Int? = nil, interfaceSubClass: Int? = nil, interfaceProtocol: Int? = nil, speed: Int? = nil)`
- `var vendorID: Int?`
- `var productID: Int?`
- `var bcdDevice: Int?`: the manufacturer's 2-byte BCD device revision.
- `var interfaceNumber: Int?`
- `var configurationValue: Int?`
- `var interfaceClass: Int?`
- `var interfaceSubClass: Int?`
- `var interfaceProtocol: Int?`
- `var speed: Int?`

## AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption

`enum InterfaceMatchingOption` (`RawRepresentable`, `Sendable`)

How interface criteria combine.

- `init?(rawValue: Int)`
- `case matchAll`: an accessory has interfaces matching every criterion.
- `case matchAny`: an accessory has an interface matching at least one criterion.

## AAError

`struct AAError` (`CustomNSError`, `Hashable`, `Sendable`)

An error AccessoryAccess returns. Errors from lower-level components can arrive in other domains.

- `static var errorDomain: String { get }`
- `static var accessoryListenerAlreadyRegistered: AAError.Code { get }`
- `static var accessoryNotAccessible: AAError.Code { get }`
- `static var internalError: AAError.Code { get }`
- `static var invalidAccessoryState: AAError.Code { get }`

## AAError.Code

`enum Code` (`RawRepresentable`, `Sendable`)

Codes in `AAErrorDomain`.

- `init?(rawValue: Int)`
- `case accessoryListenerAlreadyRegistered`: the listener is already registered.
- `case accessoryNotAccessible`: the accessory isn't accessible; it may already be in use.
- `case internalError`: also what every call fails with when the entitlement is missing.
- `case invalidAccessoryState`: the accessory isn't in the right state for the operation.

## AAErrorDomain

`let AAErrorDomain: String`

AccessoryAccess's `NSError` domain.
