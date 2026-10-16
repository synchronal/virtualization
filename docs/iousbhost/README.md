# IOUSBHost

Apple's IOUSBHost framework as of the macOS 27 SDK: `IOUSBHostDevice`, its superclass `IOUSBHostObject`, and the types their members use. Classes are macOS 10.15+ unless noted.

IOUSBHost is a user-space host driver API for USB devices. AccessoryAccess (`docs/accessory-access/`) hands out an `IOUSBHostDevice` from `AAUSBAccessory.open(serviceQueue:completionHandler:)`.

IOUSBHost is Objective-C. For many members Apple's pages give only an Objective-C declaration; those are quoted in Objective-C below. On each, `error` receives an `NSError` holding an `IOReturn` on failure, and a `BOOL` result is `YES` on success. The descriptor structs and `tIOUSB…` enums are documented in Apple's Kernel reference, in C.

Entitlements: `IOUSBHostObjectInitOptions.deviceCapture` needs `com.apple.vm.device-access` (Boolean: the app captures USB devices and uses them in a guest operating system; "required to use the IOUSBHost APIs for USB device capture") plus `IOServiceAuthorize` authorization, or root. Nothing else on these pages names an entitlement.

## IOUSBHostObject

`class IOUSBHostObject` (`NSObject`)

Base class of `IOUSBHostDevice` and `IOUSBHostInterface`: device requests and descriptors on the default control endpoint.

- `- (instancetype) initWithIOService:(io_service_t) ioService options:(IOUSBHostObjectInitOptions) options queue:(dispatch_queue_t) queue error:(NSError **) error interestHandler:(IOUSBHostInterestHandler) interestHandler;`: Objective-C only on Apple's page. Takes exclusive ownership of the service, keeping a reference released by `destroy()`; nil on failure, including when the device or interface is already open for exclusive access. `options` defaults to `IOUSBHostObjectInitOptionsNone`; `queue` is a serial queue for asynchronous I/O, created for the client by default. Call `destroy()` when done.
- `- (instancetype) initWithIOService:(io_service_t) ioService queue:(dispatch_queue_t) queue error:(NSError **) error interestHandler:(IOUSBHostInterestHandler) interestHandler;`: as above with default options.
- `var ioService: io_service_t { get }`: the kernel object.
- `var queue: dispatch_queue_t { get }`: for asynchronous I/O requests only.
- `func destroy()`: tears down the kernel connection and deregisters interest on the service. Repeat calls do nothing.
- `func destroy(options: IOUSBHostObjectDestroyOptions = [])`: `deviceSurrender` gives up ownership of the kernel service, for answering `kUSBHostMessageDeviceIsRequestingClose`.
- `var deviceDescriptor: UnsafePointer<IOUSBDeviceDescriptor>? { get }`
- `var capabilityDescriptors: UnsafePointer<IOUSBBOSDescriptor>? { get }`
- `func configurationDescriptor(with index: Int) throws -> UnsafePointer<IOUSBConfigurationDescriptor>`
- `func configurationDescriptor(withConfigurationValue configurationValue: Int) throws -> UnsafePointer<IOUSBConfigurationDescriptor>`
- `- (const IOUSBDescriptor *) descriptorWithType:(tIOUSBDescriptorType) type length:(NSUInteger *) length index:(NSUInteger) index languageID:(NSUInteger) languageID requestType:(tIOUSBDeviceRequestTypeValue) requestType requestRecipient:(tIOUSBDeviceRequestRecipientValue) requestRecipient error:(NSError **) error;`: Objective-C only. From the descriptor cache, or a `GET_DESCRIPTOR` request whose result is cached; nil if none. `length` is updated with the descriptor's length. `index` and `languageID` default to 0, `requestType` to `kIOUSBDeviceRequestTypeValueStandard`, `requestRecipient` to `kIOUSBDeviceRequestRecipientValueDevice`.
- `- (const IOUSBDescriptor *) descriptorWithType:(tIOUSBDescriptorType) type length:(NSUInteger *) length error:(NSError **) error;`: Objective-C only; default arguments.
- `- (const IOUSBDescriptor *) descriptorWithType:(tIOUSBDescriptorType) type length:(NSUInteger *) length index:(NSUInteger) index languageID:(NSUInteger) languageID error:(NSError **) error;`: Objective-C only.
- `- (const NSString *) stringWithIndex:(NSUInteger) index languageID:(NSUInteger) languageID error:(NSError **) error;`: Objective-C only. The string from a string descriptor; `languageID` defaults to `kIOUSBLanguageIDEnglishUS`.
- `- (const NSString *) stringWithIndex:(NSUInteger) index error:(NSError **) error;`: Objective-C only; English.
- `func ioData(withCapacity capacity: Int) throws -> NSMutableData`: a kernel buffer optimized for the controller, so DMA doesn't bounce. Changing its length or capacity raises an exception.
- `func data(withCapacity capacity: Int, options: IOUSBHostObjectDataOptions = []) throws -> NSMutableData`: macOS 26.4. As `ioData(withCapacity:)`, for I/O or an isochronous frame list.
- `- (BOOL) sendDeviceRequest:(IOUSBDeviceRequest) request data:(NSMutableData *) data bytesTransferred:(NSUInteger *) bytesTransferred completionTimeout:(NSTimeInterval) completionTimeout error:(NSError **) error;`: Objective-C only. Synchronous: returns when the request completes or times out. `data` nil means no data phase; `bytesTransferred` is updated with the data phase's byte count; `completionTimeout` 0 never times out, default `IOUSBHostDefaultControlCompletionTimeout`.
- `- (BOOL) sendDeviceRequest:(IOUSBDeviceRequest) request data:(NSMutableData *) data bytesTransferred:(NSUInteger *) bytesTransferred error:(NSError **) error;`: Objective-C only; default timeout.
- `- (BOOL) sendDeviceRequest:(IOUSBDeviceRequest) request error:(NSError **) error;`: Objective-C only; no data phase, default timeout.
- `- (BOOL) enqueueDeviceRequest:(IOUSBDeviceRequest) request data:(NSMutableData *) data completionTimeout:(NSTimeInterval) completionTimeout error:(NSError **) error completionHandler:(IOUSBHostCompletionHandler) completionHandler;`: Objective-C only. Asynchronous; the handler runs when the request completes or times out, and not if the method fails. Arguments as `sendDeviceRequest`.
- `- (BOOL) enqueueDeviceRequest:(IOUSBDeviceRequest) request data:(NSMutableData *) data error:(NSError **) error completionHandler:(IOUSBHostCompletionHandler) completionHandler;`: Objective-C only; default timeout.
- `- (BOOL) enqueueDeviceRequest:(IOUSBDeviceRequest) request error:(NSError **) error completionHandler:(IOUSBHostCompletionHandler) completionHandler;`: Objective-C only; no data phase, default timeout.
- `- (BOOL) abortDeviceRequestsWithOption:(IOUSBHostAbortOption) option error:(NSError **) error;`: Objective-C only. Aborts requests made by `sendDeviceRequest` or `enqueueDeviceRequest`; `option` defaults to `synchronous`.
- `- (BOOL) abortDeviceRequestsWithError:(NSError **) error;`: Objective-C only; synchronous.
- `var deviceAddress: Int { get }`: the device's current bus address.
- `- (uint64_t) frameNumberWithTime:(IOUSBHostTime *) time;`: Objective-C only. The controller's current frame number, without the microframe; `time`, if not nil, gets the current system time. For scheduling isochronous requests on an `IOUSBHostPipe`.
- `- (uint64_t) currentMicroframeWithTime:(IOUSBHostTime *) time error:(NSError **) error;`: Objective-C only; not in the class page's topics. The current microframe number, 0 on failure; `time`, if not nil, gets the microframe's system time.
- `- (uint64_t) referenceMicroframeWithTime:(IOUSBHostTime *) time error:(NSError **) error;`: Objective-C only; not in the class page's topics. A recent microframe number, 0 on failure.

## IOUSBHostDevice

`class IOUSBHostDevice: IOUSBHostObject`

Claims and configures a device, reads its descriptors, and sends device requests on endpoint 0; the interest handler monitors its state. Made with `IOUSBHostObject`'s initializers. Keep it while the device's state must not be changed by other drivers.

- `+ (CFMutableDictionaryRef) createMatchingDictionaryWithVendorID:(NSNumber *) vendorID productID:(NSNumber *) productID bcdDevice:(NSNumber *) bcdDevice deviceClass:(NSNumber *) deviceClass deviceSubclass:(NSNumber *) deviceSubclass deviceProtocol:(NSNumber *) deviceProtocol speed:(NSNumber *) speed productIDArray:(NSArray *) productIDArray;`: Objective-C only. A dictionary for IOKit matching (e.g. `IOServiceGetMatchingService`), which the caller releases. Don't pass both `productID` and `productIDArray`. Valid key combinations, most specific first: vendor + product + bcdDevice; vendor + product; vendor + subclass + protocol, or vendor + subclass, only if the device class is 0xFF; class + subclass + protocol, or class + subclass, only if it isn't.
- `var configurationDescriptor: UnsafePointer<IOUSBConfigurationDescriptor>? { get }`: the selected configuration; nil if none. The device owns it; it is valid until `destroy()`.
- `- (BOOL) configureWithValue:(NSUInteger) value matchInterfaces:(BOOL) matchInterfaces error:(NSError **) error;`: Objective-C only. Selects a configuration; `matchInterfaces` (default `YES`) registers its interfaces for matching.
- `- (BOOL) configureWithValue:(NSUInteger) value error:(NSError **) error;`: Objective-C only; registers the interfaces for matching.
- `func reset() throws`: resets and tries to re-enumerate the device, terminating its kernel service and child services; on success a new service is registered and this object's connection is no longer valid. Use the re-enumerated device through a new object.

## IOUSBHostObjectInitOptions

`struct IOUSBHostObjectInitOptions` (`OptionSet`, `Sendable`)

- `init(rawValue: UInt)`
- `IOUSBHostObjectInitOptionsNone`: the default.
- `static var deviceCapture: IOUSBHostObjectInitOptions { get }`: captures the device, terminating all clients and drivers of the device and its interfaces, the caller's included. Needs `com.apple.vm.device-access` plus `IOServiceAuthorize` authorization, or root. On `destroy()` the device resets and its interfaces re-register for matching.
- `static var deviceSeize: IOUSBHostObjectInitOptions { get }`: undocumented.

## IOUSBHostObjectDestroyOptions

`struct IOUSBHostObjectDestroyOptions` (`OptionSet`, `Sendable`)

Options for `destroy(options:)`.

- `init(rawValue: UInt)`
- `static var deviceSurrender: IOUSBHostObjectDestroyOptions { get }`: the device isn't reset and drivers aren't re-registered for matching, so an object made with `deviceCapture` can honour `kUSBHostMessageDeviceIsRequestingClose`. macOS only.

## IOUSBHostObjectDataOptions

`struct IOUSBHostObjectDataOptions` (`OptionSet`, `Sendable`)

Options for `data(withCapacity:options:)`. Apple's page text is cut off.

- `init(rawValue: UInt)`
- `static var kernelUserShared: IOUSBHostObjectDataOptions { get }`: per the surviving text, shared by the task and the kernel; recommended for isochronous frame lists.

## IOUSBHostAbortOption

`enum IOUSBHostAbortOption` (`RawRepresentable`, `Sendable`)

- `init?(rawValue: UInt)`
- `case synchronous`: the default; every request is aborted before the call returns.
- `case asynchronous`

## IOUSBHostInterestHandler

`typealias IOUSBHostInterestHandler = (IOUSBHostObject, UInt32, UnsafeMutableRawPointer?) -> Void`

The `kIOGeneralInterest` handler for service-state changes such as termination: the object, an IOKit message type (`IOKit/IOMessage.h`), and its argument (a pointer to the data if larger than `sizeof(void*)`, otherwise the data). Runs on an internal serial queue separate from the I/O queue.

## IOUSBHostCompletionHandler

`typealias IOUSBHostCompletionHandler = (IOReturn, Int) -> Void`

Asynchronous control, bulk, and interrupt completion: the status and the bytes transferred.

## IOUSBHostDeviceRequestType(_:_:_:)

`func IOUSBHostDeviceRequestType(_ direction: tIOUSBDeviceRequestDirectionValue, _ type: tIOUSBDeviceRequestTypeValue, _ recipient: tIOUSBDeviceRequestRecipientValue) -> UInt8`

Builds a device request's `bmRequestType`.

## IOUSBHostDefaultControlCompletionTimeout

`let IOUSBHostDefaultControlCompletionTimeout: TimeInterval`

The default completion timeout for I/O requests.

## IOUSBHostTime

`typealias IOUSBHostTime = UInt64`

An absolute time.

## IOUSBHostErrorDomain

`let IOUSBHostErrorDomain: String`

## IOUSBDeviceRequest

`typedef struct IOUSBDeviceRequest IOUSBDeviceRequest;` (C, macOS 10.15)

A standard device request.

- `uint8_t bmRequestType;`
- `uint8_t bRequest;`
- `uint16_t wValue;`
- `uint16_t wIndex;`
- `uint16_t wLength;`

## IOUSBDescriptorHeader

`typedef struct IOUSBDescriptorHeader IOUSBDescriptorHeader;` (C)

The header every descriptor begins with. `IOUSBDescriptor` is `typedef IOUSBDescriptorHeader IOUSBDescriptor;` (macOS 10.15).

- `uint8_t bLength;`: the descriptor's size.
- `uint8_t bDescriptorType;`

## IOUSBDeviceDescriptor

`typedef struct IOUSBDeviceDescriptor IOUSBDeviceDescriptor;` (C)

- `uint8_t bLength;`
- `uint8_t bDescriptorType;`
- `uint16_t bcdUSB;`: the USB specification version.
- `uint8_t bDeviceClass;`
- `uint8_t bDeviceSubClass;`
- `uint8_t bDeviceProtocol;`
- `uint8_t bMaxPacketSize0;`: an exponent giving endpoint 0's maximum packet size.
- `uint16_t idVendor;`
- `uint16_t idProduct;`
- `uint16_t bcdDevice;`: the release number, BCD.
- `uint8_t iManufacturer;`: string descriptor index.
- `uint8_t iProduct;`: string descriptor index.
- `uint8_t iSerialNumber;`: string descriptor index.
- `uint8_t bNumConfigurations;`

## IOUSBConfigurationDescriptor

`typedef struct IOUSBConfigurationDescriptor IOUSBConfigurationDescriptor;` (C)

- `uint8_t bLength;`
- `uint8_t bDescriptorType;`
- `uint16_t wTotalLength;`: including its interface, endpoint, and vendor-specific descriptors.
- `uint8_t bNumInterfaces;`
- `uint8_t bConfigurationValue;`: the value that selects it.
- `uint8_t iConfiguration;`: string descriptor index.
- `uint8_t bmAttributes;`
- `uint8_t MaxPower;`: in 2 mA units.

## IOUSBBOSDescriptor

`typedef struct IOUSBBOSDescriptor IOUSBBOSDescriptor;` (C)

A binary object store descriptor.

- `uint8_t bLength;`
- `uint8_t bDescriptorType;`
- `uint16_t wTotalLength;`: including its subdescriptors.
- `uint8_t bNumDeviceCaps;`

## tIOUSBDescriptorType

`typedef enum tIOUSBDescriptorType tIOUSBDescriptorType;` (C, macOS 10.15)

- `kIOUSBDescriptorTypeDevice`, `kIOUSBDescriptorTypeConfiguration`, `kIOUSBDescriptorTypeString`, `kIOUSBDescriptorTypeInterface`, `kIOUSBDescriptorTypeEndpoint`, `kIOUSBDescriptorTypeDeviceQualifier`, `kIOUSBDescriptorTypeOtherSpeedConfiguration`, `kIOUSBDescriptorTypeInterfacePower`, `kIOUSBDescriptorTypeOTG`, `kIOUSBDescriptorTypeDebug`, `kIOUSBDescriptorTypeInterfaceAssociation`, `kIOUSBDescriptorTypeBOS`, `kIOUSBDescriptorTypeDeviceCapability`, `kIOUSBDescriptorTypeHub`, `kIOUSBDescriptorTypeSuperSpeedHub`, `kIOUSBDescriptorTypeSuperSpeedUSBEndpointCompanion`, `kIOUSBDescriptorTypeSuperSpeedPlusIsochronousEndpointCompanion`
- `kIOUSBDecriptorTypeHID`, `kIOUSBDecriptorTypeReport` (Apple's spelling), `kIOUSBDescriptorTypePhysical`

## tIOUSBDeviceRequestDirectionValue

`typedef enum tIOUSBDeviceRequestDirectionValue tIOUSBDeviceRequestDirectionValue;` (C, macOS 10.15)

- `kIOUSBDeviceRequestDirectionValueIn`
- `kIOUSBDeviceRequestDirectionValueOut`

## tIOUSBDeviceRequestTypeValue

`typedef enum tIOUSBDeviceRequestTypeValue tIOUSBDeviceRequestTypeValue;` (C, macOS 10.15)

- `kIOUSBDeviceRequestTypeValueStandard`
- `kIOUSBDeviceRequestTypeValueClass`
- `kIOUSBDeviceRequestTypeValueVendor`

## tIOUSBDeviceRequestRecipientValue

`typedef enum tIOUSBDeviceRequestRecipientValue tIOUSBDeviceRequestRecipientValue;` (C, macOS 10.15)

- `kIOUSBDeviceRequestRecipientValueDevice`
- `kIOUSBDeviceRequestRecipientValueInterface`
- `kIOUSBDeviceRequestRecipientValueEndpoint`
- `kIOUSBDeviceRequestRecipientValueOther`

## IOUSBHostMatchingPropertyKey, IOUSBHostDevicePropertyKey, IOUSBHostPropertyKey

Service-plane property keys, the keys of `createMatchingDictionaryWithVendorID:…`'s dictionary.

- `struct IOUSBHostMatchingPropertyKey`, `init(rawValue: String)`: `static let` `vendorID`, `productID`, `deviceReleaseNumber`, `configurationValue`, `speed`, `productIDArray`, `productIDMask`, `interfaceNumber`, `interfaceClass`, `interfaceSubClass`, `interfaceProtocol`, `deviceClass`, `deviceSubClass`, `deviceProtocol`.
- `struct IOUSBHostDevicePropertyKey`, `init(rawValue: String)`: `static let` `currentConfiguration`, `containerID`, `serialNumberString`, `vendorString`.
- `typealias IOUSBHostPropertyKey = NSString`: `let IOUSBHostPropertyKeyLocationID: String`, the device's location ID.

## Other IOUSBHost types

Not documented here. None is taken or returned by `IOUSBHostDevice`'s or `IOUSBHostObject`'s members; listed for scope.

- `class IOUSBHostInterface: IOUSBHostObject`: "The class for accessing USB-related services"; `IOUSBHostObject`'s other subclass.
- `class IOUSBHostPipe`: sends control, bulk, interrupt, and isochronous I/O requests for function drivers, and manages stream capabilities. `frameNumberWithTime:` serves its isochronous requests.
- `class IOUSBHostStream`: sends stream data for function drivers.
- `class IOUSBHostIOSource`: base class of pipes and streams.
- `IOUSBHostCIControllerStateMachine`, `IOUSBHostCIDeviceStateMachine`, `IOUSBHostCIEndpointStateMachine`, `IOUSBHostCIPortStateMachine`, `IOUSBHostControllerInterface`, and the `IOUSBHostCI…` structs and typealiases: undescribed on Apple's framework page.
- `IOUSBHostIsochronousTransaction`, `IOUSBHostIsochronousTransactionOptions`, `IOUSBHostIsochronousTransferOptions`, `IOUSBHostIsochronousTransactionCompletionHandler`: isochronous transfer types.
- `IOUSBGetPlatformCapabilityDescriptor(_:)`, `IOUSBGetSuperSpeedPlusDeviceCapabilityDescriptor(_:)`, `IOUSBGetEndpointBurstSize(_:_:_:_:)`, and the rest of "IOUSBHost Functions": descriptor helpers.
