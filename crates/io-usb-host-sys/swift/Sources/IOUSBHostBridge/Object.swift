//===----------------------------------------------------------------------===//
// `IOUSBHostObject`, a base class: its object crosses by address. Its members
// IOUSBHost refines for Swift are its `__` ones.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import IOUSBHost

/// An `IOUSBHostObject` Rust lends for the call.
private func bridging(hostObject pointer: UnsafeRawPointer) -> IOUSBHostObject {
  bridging(borrowed: pointer)
}

func iousbhost_object_destroy(this: UnsafeRawPointer) {
  bridging(hostObject: this).destroy()
}

/// `options` is an `IOUSBHostObjectDestroyOptions`' raw value.
func iousbhost_object_destroy_options(this: UnsafeRawPointer, options: UInt) {
  bridging(hostObject: this).destroy(options: IOUSBHostObjectDestroyOptions(rawValue: options))
}

func iousbhost_object_io_service(this: UnsafeRawPointer) -> UInt32 {
  bridging(hostObject: this).ioService
}

func iousbhost_object_queue(this: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: bridging(hostObject: this).queue)
}

/// The descriptor's `bLength` bytes, as an `NSData` by address: `0` for
/// `nil`.
func iousbhost_object_device_descriptor(this: UnsafeRawPointer) -> UInt {
  guard let descriptor = bridging(hostObject: this).deviceDescriptor else {
    return 0
  }

  return UInt(bitPattern: bridging(descriptor: descriptor, length: Int(descriptor.pointee.bLength)))
}

/// The descriptor's `wTotalLength` bytes, its device capability descriptors
/// with it, as an `NSData` by address: `0` for `nil`.
func iousbhost_object_capability_descriptors(this: UnsafeRawPointer) -> UInt {
  guard let descriptor = bridging(hostObject: this).capabilityDescriptors else {
    return 0
  }

  return UInt(
    bitPattern: bridging(descriptor: descriptor, length: Int(UInt16(littleEndian: descriptor.pointee.wTotalLength))))
}

// What follows throws and returns an object, which crosses by address:
// swift-bridge 0.1.59 can't return a pointer in a `Result`.

/// The descriptor's `wTotalLength` bytes, as an `NSData`.
func iousbhost_object_configuration_descriptor_with(this: UnsafeRawPointer, index: Int) throws(IOUSBHostThrown)
  -> UInt
{
  let descriptor = try bridging {
    try bridging(hostObject: this).configurationDescriptor(with: index)
  }

  return UInt(bitPattern: bridging(configurationDescriptor: descriptor))
}

/// The descriptor's `wTotalLength` bytes, as an `NSData`.
func iousbhost_object_configuration_descriptor_with_configuration_value(
  this: UnsafeRawPointer,
  configuration_value: Int
) throws(IOUSBHostThrown) -> UInt {
  let descriptor = try bridging {
    try bridging(hostObject: this).configurationDescriptor(withConfigurationValue: configuration_value)
  }

  return UInt(bitPattern: bridging(configurationDescriptor: descriptor))
}

/// A configuration descriptor, its interface and endpoint descriptors with
/// it, as an `NSData`.
func bridging(configurationDescriptor descriptor: UnsafePointer<IOUSBConfigurationDescriptor>)
  -> UnsafeMutableRawPointer
{
  bridging(descriptor: descriptor, length: Int(UInt16(littleEndian: descriptor.pointee.wTotalLength)))
}

/// The descriptor's `length` bytes, as an `NSData`.
func iousbhost_object_descriptor_with_type(
  this: UnsafeRawPointer,
  descriptor_type: BridgedIOUSBDescriptorType,
  index: Int,
  language_id: Int,
  request_type: BridgedIOUSBDeviceRequestTypeValue,
  request_recipient: BridgedIOUSBDeviceRequestRecipientValue
) throws(IOUSBHostThrown) -> UInt {
  var length = 0
  let descriptor = try bridging {
    try bridging(hostObject: this).__descriptor(
      with: tIOUSBDescriptorType(descriptor_type),
      length: &length,
      index: index,
      languageID: language_id,
      requestType: tIOUSBDeviceRequestTypeValue(request_type),
      requestRecipient: tIOUSBDeviceRequestRecipientValue(request_recipient))
  }

  return UInt(bitPattern: bridging(descriptor: descriptor, length: length))
}

func iousbhost_object_string_with_index(this: UnsafeRawPointer, index: Int, language_id: Int)
  throws(IOUSBHostThrown) -> RustString
{
  try bridging {
    try bridging(hostObject: this).__string(with: index, languageID: language_id) as String
  }.intoRustString()
}

/// An `NSMutableData`, which Rust takes over.
func iousbhost_object_io_data_with_capacity(this: UnsafeRawPointer, capacity: Int) throws(IOUSBHostThrown)
  -> UInt
{
  bridging(
    retainedAddress: try bridging {
      try bridging(hostObject: this).ioData(withCapacity: capacity)
    })
}

/// `options` is an `IOUSBHostObjectDataOptions`' raw value; the result an
/// `NSMutableData`, which Rust takes over.
func iousbhost_object_data_with_capacity_options(this: UnsafeRawPointer, capacity: Int, options: UInt)
  throws(IOUSBHostThrown) -> UInt
{
  bridging(
    retainedAddress: try bridging {
      try bridging(hostObject: this).data(
        withCapacity: capacity, options: IOUSBHostObjectDataOptions(rawValue: options))
    })
}

/// `data` is an `NSMutableData` Rust lends by address, `0` for `nil`; the
/// result is `bytesTransferred`.
func iousbhost_object_send_device_request(
  this: UnsafeRawPointer,
  request: BridgedIOUSBDeviceRequest,
  data: UInt,
  completion_timeout: Double
) throws(IOUSBHostThrown) -> Int {
  var bytesTransferred = 0

  try bridging {
    try bridging(hostObject: this).__send(
      IOUSBDeviceRequest(request),
      data: bridging(borrowedAddress: data),
      bytesTransferred: &bytesTransferred,
      completionTimeout: completion_timeout)
  }

  return bytesTransferred
}

/// `data` is an `NSMutableData` Rust lends by address, `0` for `nil`. What
/// it throws, Rust takes over by address, `0` for nothing: swift-bridge
/// 0.1.59 can't pass a closure to a throwing function.
func iousbhost_object_enqueue_device_request(
  this: UnsafeRawPointer,
  request: BridgedIOUSBDeviceRequest,
  data: UInt,
  completion_timeout: Double,
  completion_handler: @escaping (Int32, Int) -> Void
) -> UInt {
  thrownAddress {
    try bridging(hostObject: this).__enqueue(
      IOUSBDeviceRequest(request),
      data: bridging(borrowedAddress: data),
      completionTimeout: completion_timeout,
      completionHandler: completion_handler)
  }
}

/// As `iousbhost_object_enqueue_device_request`, with a `nil` completion
/// handler.
func iousbhost_object_enqueue_device_request_nil_completion_handler(
  this: UnsafeRawPointer,
  request: BridgedIOUSBDeviceRequest,
  data: UInt,
  completion_timeout: Double
) throws(IOUSBHostThrown) {
  try bridging {
    try bridging(hostObject: this).__enqueue(
      IOUSBDeviceRequest(request),
      data: bridging(borrowedAddress: data),
      completionTimeout: completion_timeout,
      completionHandler: nil)
  }
}

func iousbhost_object_abort_device_requests_with_option(this: UnsafeRawPointer, option: BridgedIOUSBHostAbortOption)
  throws(IOUSBHostThrown)
{
  try bridging {
    try bridging(hostObject: this).__abortDeviceRequests(with: IOUSBHostAbortOption(option))
  }
}

func iousbhost_object_device_address(this: UnsafeRawPointer) -> Int {
  bridging(hostObject: this).deviceAddress
}

/// The frame number and `time`.
func iousbhost_object_frame_number_with_time(this: UnsafeRawPointer) -> BridgedIOUSBHostFrame {
  var time: IOUSBHostTime = 0
  let frame = bridging(hostObject: this).__frameNumber(withTime: &time)

  return BridgedIOUSBHostFrame(number: frame, time: time)
}

/// The microframe number and `time`.
func iousbhost_object_current_microframe_with_time(this: UnsafeRawPointer) throws(IOUSBHostThrown)
  -> BridgedIOUSBHostFrame
{
  var time: IOUSBHostTime = 0
  var error: NSError?
  let microframe = bridging(hostObject: this).__currentMicroframe(withTime: &time, error: &error)

  try bridging(error: error)

  return BridgedIOUSBHostFrame(number: microframe, time: time)
}

/// The microframe number and `time`.
func iousbhost_object_reference_microframe_with_time(this: UnsafeRawPointer) throws(IOUSBHostThrown)
  -> BridgedIOUSBHostFrame
{
  var time: IOUSBHostTime = 0
  var error: NSError?
  let microframe = bridging(hostObject: this).__referenceMicroframe(withTime: &time, error: &error)

  try bridging(error: error)

  return BridgedIOUSBHostFrame(number: microframe, time: time)
}
