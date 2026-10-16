//===----------------------------------------------------------------------===//
// `IOUSBHostDevice`, and the `IOUSBHostObject` initializer it is made with.
// Its interest handler is Rust's, which `BridgedIOUSBHostInterestHandler`
// holds.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import IOUSBHost

/// `options` is an `IOUSBHostObjectInitOptions`' raw value; `queue` a
/// `dispatch_queue_t` Rust lends by address, `0` for `nil`.
func iousbhost_device_init_io_service_options_queue_interest_handler(
  io_service: UInt32,
  options: UInt,
  queue: UInt,
  interest_handler: IOUSBHostInterestHandlerBridge
) throws(IOUSBHostThrown) -> IOUSBHostDevice {
  try bridging {
    try IOUSBHostDevice(
      __ioService: io_service,
      options: IOUSBHostObjectInitOptions(rawValue: options),
      queue: bridging(borrowedAddress: queue),
      interestHandler: { hostObject, messageType, messageArgument in
        interest_handler.call(
          hostObject as! IOUSBHostDevice, messageType, UInt(bitPattern: messageArgument))
      })
  }
}

/// As `iousbhost_device_init_io_service_options_queue_interest_handler`, with
/// a `nil` interest handler.
func iousbhost_device_init_io_service_options_queue_nil_interest_handler(
  io_service: UInt32,
  options: UInt,
  queue: UInt
) throws(IOUSBHostThrown) -> IOUSBHostDevice {
  try bridging {
    try IOUSBHostDevice(
      __ioService: io_service,
      options: IOUSBHostObjectInitOptions(rawValue: options),
      queue: bridging(borrowedAddress: queue),
      interestHandler: nil)
  }
}

/// Each argument is an `NSNumber` Rust lends by address, `0` for `nil`;
/// `product_id_array` an `NSArray` of them. They cross in a struct: there are
/// more than clippy lets a Rust function take. The result is an
/// `NSDictionary`, which Rust takes over.
func iousbhost_device_create_matching_dictionary(arguments: BridgedIOUSBHostMatchingDictionaryArguments)
  -> UnsafeMutableRawPointer
{
  let dictionary = IOUSBHostDevice.__createMatchingDictionary(
    withVendorID: bridging(borrowedAddress: arguments.vendor_id),
    productID: bridging(borrowedAddress: arguments.product_id),
    bcdDevice: bridging(borrowedAddress: arguments.bcd_device),
    deviceClass: bridging(borrowedAddress: arguments.device_class),
    deviceSubclass: bridging(borrowedAddress: arguments.device_subclass),
    deviceProtocol: bridging(borrowedAddress: arguments.device_protocol),
    speed: bridging(borrowedAddress: arguments.speed),
    productIDArray: (bridging(borrowedAddress: arguments.product_id_array) as NSArray?).map { $0 as [AnyObject] })

  return bridging(retained: dictionary.takeRetainedValue() as NSDictionary)
}

/// Another bridge's `IOUSBHostDevice`, lent by address for the call.
func iousbhost_device_bridge_from(device: UnsafeRawPointer) -> IOUSBHostDevice {
  bridging(borrowed: device)
}

/// `object as? IOUSBHostDevice`, for a base class's object.
func iousbhost_device_bridge_cast(object: UnsafeRawPointer) throws(IOUSBHostNil) -> IOUSBHostDevice {
  try bridging(bridging(borrowed: object) as NSObject as? IOUSBHostDevice)
}

/// The descriptor's `wTotalLength` bytes, as an `NSData` by address: `0` for
/// `nil`.
func iousbhost_device_configuration_descriptor(this: IOUSBHostDevice) -> UInt {
  UInt(bitPattern: this.configurationDescriptor.map { bridging(configurationDescriptor: $0) })
}

func iousbhost_device_configure_with_value(this: IOUSBHostDevice, value: Int, match_interfaces: Bool)
  throws(IOUSBHostThrown)
{
  try bridging {
    try this.__configure(withValue: value, matchInterfaces: match_interfaces)
  }
}

func iousbhost_device_reset(this: IOUSBHostDevice) throws(IOUSBHostThrown) {
  try bridging {
    try this.reset()
  }
}
