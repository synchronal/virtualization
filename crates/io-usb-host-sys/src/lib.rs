//! FFI to Apple's [IOUSBHost] framework, through swift-bridge. Use the
//! `io-usb-host` crate instead; this one holds its `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them, the way
//! `virtualization-sys`'s are:
//!
//! - Classes keep Apple's names. Members are methods `iousbhost_<member>`;
//!   initializers and class members are free functions
//!   `iousbhost_<type>_init[_<labels>]` and `iousbhost_<type>_<member>`.
//!   IOUSBHost refines most of its members for Swift; the bridge calls the
//!   refined (`__`) ones, as a Swift caller does.
//! - `IOUSBHostObject`, the base class, is [`NSObject`], crossing by
//!   address: swift-bridge has no subclassing. Its members are free functions
//!   `iousbhost_object_<member>`, and `iousbhost_device_bridge_cast` is
//!   Swift's `as?` back to [`IOUSBHostDevice`].
//! - Foundation's objects are objc2's, through `foundation-sys`, and cross as
//!   pointers. A descriptor, which IOUSBHost lends as a pointer to memory the
//!   object owns, is a copy of its bytes.
//! - A throwing function returns `Result<T, Retained<NSError>>`.
//! - An option set crosses as its raw value. Enums keep Apple's cases, plus
//!   `Unknown(rawValue)` for a case newer than this crate.
//!
//! The glue's names differ from the other bridges': all link into one binary,
//! and swift-bridge names symbols for each bridge's types.
//!
//! macOS only: the build fails elsewhere.
//!
//! [IOUSBHost]: https://developer.apple.com/documentation/iousbhost

use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::{self, NonNull};

use dispatch2::{DispatchQueue, DispatchRetained};
use foundation_sys::PropertyListValue;
use foundation_sys::objc::{
  AnyObject, NSArray, NSData, NSDictionary, NSNumber, NSString, borrowed, borrowed_or_nil, ns_data_bytes,
  property_list_dictionary, retained, retained_or_nil,
};
pub use foundation_sys::objc::{NSError, NSMutableData, NSObject, Retained};

pub use self::ffi::{
  BridgedIOUSBDeviceRequest, IOUSBDescriptorType, IOUSBDeviceRequestDirectionValue, IOUSBDeviceRequestRecipientValue,
  IOUSBDeviceRequestTypeValue, IOUSBHostAbortOption, IOUSBHostDevice, iousbhost_default_control_completion_timeout,
  iousbhost_device_property_key_container_id, iousbhost_device_property_key_current_configuration,
  iousbhost_device_property_key_serial_number_string, iousbhost_device_property_key_vendor_string,
  iousbhost_device_request_type, iousbhost_error_domain, iousbhost_matching_property_key_configuration_value,
  iousbhost_matching_property_key_device_class, iousbhost_matching_property_key_device_protocol,
  iousbhost_matching_property_key_device_release_number, iousbhost_matching_property_key_device_sub_class,
  iousbhost_matching_property_key_interface_class, iousbhost_matching_property_key_interface_number,
  iousbhost_matching_property_key_interface_protocol, iousbhost_matching_property_key_interface_sub_class,
  iousbhost_matching_property_key_product_id, iousbhost_matching_property_key_product_id_array,
  iousbhost_matching_property_key_product_id_mask, iousbhost_matching_property_key_speed,
  iousbhost_matching_property_key_vendor_id, iousbhost_object_data_options_kernel_user_shared,
  iousbhost_object_destroy_options_device_surrender, iousbhost_object_init_options_device_capture,
  iousbhost_object_init_options_device_seize, iousbhost_property_key_location_id,
};

// No doc comments inside the module: swift-bridge can't parse them. Shared
// types are renamed on the Swift side, where Apple's names are taken.
//
// A function with a Foundation pointer takes the object it is a member of by
// value: swift-bridge 0.1.59 releases an object a free function borrows.
#[swift_bridge::bridge]
mod ffi {
  // Foundation.swift

  enum IOUSBHostNil {
    Nil,
  }

  enum IOUSBHostThrown {
    Error(usize),
  }

  // Definitions.swift

  #[swift_bridge(swift_name = "BridgedIOUSBHostAbortOption")]
  enum IOUSBHostAbortOption {
    Asynchronous,
    Synchronous,
    Unknown(usize),
  }

  #[swift_bridge(swift_name = "BridgedIOUSBDescriptorType")]
  enum IOUSBDescriptorType {
    Device,
    Configuration,
    String,
    Interface,
    Endpoint,
    DeviceQualifier,
    OtherSpeedConfiguration,
    InterfacePower,
    Otg,
    Debug,
    InterfaceAssociation,
    Bos,
    DeviceCapability,
    Hid,
    Report,
    Physical,
    Hub,
    SuperSpeedHub,
    SuperSpeedUSBEndpointCompanion,
    SuperSpeedPlusIsochronousEndpointCompanion,
    Unknown(u32),
  }

  #[swift_bridge(swift_name = "BridgedIOUSBDeviceRequestDirectionValue")]
  enum IOUSBDeviceRequestDirectionValue {
    Out,
    In,
    Unknown(u32),
  }

  #[swift_bridge(swift_name = "BridgedIOUSBDeviceRequestTypeValue")]
  enum IOUSBDeviceRequestTypeValue {
    Standard,
    Class,
    Vendor,
    Unknown(u32),
  }

  #[swift_bridge(swift_name = "BridgedIOUSBDeviceRequestRecipientValue")]
  enum IOUSBDeviceRequestRecipientValue {
    Device,
    Interface,
    Endpoint,
    Other,
    Unknown(u32),
  }

  #[swift_bridge(swift_repr = "struct")]
  struct BridgedIOUSBDeviceRequest {
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
  }

  extern "Swift" {
    fn iousbhost_object_init_options_device_capture() -> usize;
    fn iousbhost_object_init_options_device_seize() -> usize;
    fn iousbhost_object_destroy_options_device_surrender() -> usize;
    fn iousbhost_object_data_options_kernel_user_shared() -> usize;
    fn iousbhost_device_request_type(
      direction: IOUSBDeviceRequestDirectionValue,
      request_type: IOUSBDeviceRequestTypeValue,
      recipient: IOUSBDeviceRequestRecipientValue,
    ) -> u8;
    fn iousbhost_default_control_completion_timeout() -> f64;
    fn iousbhost_error_domain() -> String;
    fn iousbhost_property_key_location_id() -> String;
    fn iousbhost_matching_property_key_vendor_id() -> String;
    fn iousbhost_matching_property_key_product_id() -> String;
    fn iousbhost_matching_property_key_product_id_mask() -> String;
    fn iousbhost_matching_property_key_product_id_array() -> String;
    fn iousbhost_matching_property_key_interface_number() -> String;
    fn iousbhost_matching_property_key_configuration_value() -> String;
    fn iousbhost_matching_property_key_device_release_number() -> String;
    fn iousbhost_matching_property_key_interface_class() -> String;
    fn iousbhost_matching_property_key_interface_sub_class() -> String;
    fn iousbhost_matching_property_key_interface_protocol() -> String;
    fn iousbhost_matching_property_key_device_class() -> String;
    fn iousbhost_matching_property_key_device_sub_class() -> String;
    fn iousbhost_matching_property_key_device_protocol() -> String;
    fn iousbhost_matching_property_key_speed() -> String;
    fn iousbhost_device_property_key_vendor_string() -> String;
    fn iousbhost_device_property_key_serial_number_string() -> String;
    fn iousbhost_device_property_key_container_id() -> String;
    fn iousbhost_device_property_key_current_configuration() -> String;
  }

  // Object.swift

  #[swift_bridge(swift_repr = "struct")]
  struct BridgedIOUSBHostFrame {
    number: u64,
    time: u64,
  }

  extern "Swift" {
    fn iousbhost_object_destroy(this: *const c_void);
    fn iousbhost_object_destroy_options(this: *const c_void, options: usize);
    fn iousbhost_object_io_service(this: *const c_void) -> u32;
    fn iousbhost_object_queue(this: *const c_void) -> *mut c_void;
    fn iousbhost_object_device_descriptor(this: *const c_void) -> usize;
    fn iousbhost_object_capability_descriptors(this: *const c_void) -> usize;
    fn iousbhost_object_configuration_descriptor_with(
      this: *const c_void,
      index: isize,
    ) -> Result<usize, IOUSBHostThrown>;
    fn iousbhost_object_configuration_descriptor_with_configuration_value(
      this: *const c_void,
      configuration_value: isize,
    ) -> Result<usize, IOUSBHostThrown>;
    fn iousbhost_object_descriptor_with_type(
      this: *const c_void,
      descriptor_type: IOUSBDescriptorType,
      index: isize,
      language_id: isize,
      request_type: IOUSBDeviceRequestTypeValue,
      request_recipient: IOUSBDeviceRequestRecipientValue,
    ) -> Result<usize, IOUSBHostThrown>;
    fn iousbhost_object_string_with_index(
      this: *const c_void,
      index: isize,
      language_id: isize,
    ) -> Result<String, IOUSBHostThrown>;
    fn iousbhost_object_io_data_with_capacity(this: *const c_void, capacity: isize) -> Result<usize, IOUSBHostThrown>;
    fn iousbhost_object_data_with_capacity_options(
      this: *const c_void,
      capacity: isize,
      options: usize,
    ) -> Result<usize, IOUSBHostThrown>;
    fn iousbhost_object_send_device_request(
      this: *const c_void,
      request: BridgedIOUSBDeviceRequest,
      data: usize,
      completion_timeout: f64,
    ) -> Result<isize, IOUSBHostThrown>;
    fn iousbhost_object_enqueue_device_request(
      this: *const c_void,
      request: BridgedIOUSBDeviceRequest,
      data: usize,
      completion_timeout: f64,
      completion_handler: Box<dyn FnOnce(i32, isize)>,
    ) -> usize;
    fn iousbhost_object_enqueue_device_request_nil_completion_handler(
      this: *const c_void,
      request: BridgedIOUSBDeviceRequest,
      data: usize,
      completion_timeout: f64,
    ) -> Result<(), IOUSBHostThrown>;
    fn iousbhost_object_abort_device_requests_with_option(
      this: *const c_void,
      option: IOUSBHostAbortOption,
    ) -> Result<(), IOUSBHostThrown>;
    fn iousbhost_object_device_address(this: *const c_void) -> isize;
    fn iousbhost_object_frame_number_with_time(this: *const c_void) -> BridgedIOUSBHostFrame;
    fn iousbhost_object_current_microframe_with_time(
      this: *const c_void,
    ) -> Result<BridgedIOUSBHostFrame, IOUSBHostThrown>;
    fn iousbhost_object_reference_microframe_with_time(
      this: *const c_void,
    ) -> Result<BridgedIOUSBHostFrame, IOUSBHostThrown>;
  }

  // Device.swift

  #[swift_bridge(swift_repr = "struct")]
  struct BridgedIOUSBHostMatchingDictionaryArguments {
    vendor_id: usize,
    product_id: usize,
    bcd_device: usize,
    device_class: usize,
    device_subclass: usize,
    device_protocol: usize,
    speed: usize,
    product_id_array: usize,
  }

  extern "Swift" {
    type IOUSBHostDevice;

    fn iousbhost_device_init_io_service_options_queue_interest_handler(
      io_service: u32,
      options: usize,
      queue: usize,
      interest_handler: IOUSBHostInterestHandlerBridge,
    ) -> Result<IOUSBHostDevice, IOUSBHostThrown>;
    fn iousbhost_device_init_io_service_options_queue_nil_interest_handler(
      io_service: u32,
      options: usize,
      queue: usize,
    ) -> Result<IOUSBHostDevice, IOUSBHostThrown>;
    fn iousbhost_device_create_matching_dictionary(
      arguments: BridgedIOUSBHostMatchingDictionaryArguments,
    ) -> *mut c_void;
    fn iousbhost_device_bridge_from(device: *const c_void) -> IOUSBHostDevice;
    fn iousbhost_device_bridge_cast(object: *const c_void) -> Result<IOUSBHostDevice, IOUSBHostNil>;
    fn bridge_retained(self: &IOUSBHostDevice) -> IOUSBHostDevice;
    fn bridge_borrowed(self: &IOUSBHostDevice) -> *const c_void;
    fn ns_description(self: &IOUSBHostDevice) -> String;
    fn iousbhost_device_configuration_descriptor(this: IOUSBHostDevice) -> usize;
    fn bridge_configure_with_value(
      self: &IOUSBHostDevice,
      value: isize,
      match_interfaces: bool,
    ) -> Result<(), IOUSBHostThrown>;
    fn bridge_reset(self: &IOUSBHostDevice) -> Result<(), IOUSBHostThrown>;
  }

  extern "Rust" {
    type IOUSBHostInterestHandlerBridge;

    fn call(
      self: &IOUSBHostInterestHandlerBridge,
      host_object: IOUSBHostDevice,
      message_type: u32,
      message_argument: usize,
    );
  }
}

// Foundation.swift

/// What a throwing call threw.
fn thrown(thrown: ffi::IOUSBHostThrown) -> Retained<NSError> {
  let ffi::IOUSBHostThrown::Error(error) = thrown;

  // SAFETY: Swift's `bridging(_ body:)` passes each `NSError` it throws
  // retained, by address, once.
  unsafe { retained_or_nil(error) }.expect("Swift throws an NSError, never nil")
}

/// A descriptor's bytes, from an `NSData` Swift handed over by address
/// (swift-bridge 0.1.59 can't return a pointer in a `Result`).
///
/// # Safety
///
/// `address` is a `bridging(descriptor:length:)` result's, taken once.
unsafe fn descriptor(address: usize) -> Vec<u8> {
  // SAFETY: the caller's.
  unsafe { descriptor_or_nil(address) }.expect("Swift passes a descriptor, never nil")
}

/// An optional descriptor's bytes, from an `NSData` Swift handed over by
/// address.
///
/// # Safety
///
/// `address` is a `bridging(descriptor:length:)` result's, or `0`, taken
/// once.
unsafe fn descriptor_or_nil(address: usize) -> Option<Vec<u8>> {
  // SAFETY: the caller's.
  unsafe { retained_or_nil::<NSData>(address) }.map(|data| ns_data_bytes(&data))
}

/// An `NSMutableData` Swift handed over by address.
///
/// # Safety
///
/// `address` is a retained `NSMutableData`'s, taken once.
unsafe fn mutable_data(address: usize) -> Retained<NSMutableData> {
  // SAFETY: the caller's.
  unsafe { retained_or_nil(address) }.expect("Swift passes an NSMutableData, never nil")
}

/// An optional dispatch queue, lent to Swift by address.
fn borrowed_queue_or_nil(queue: Option<&DispatchQueue>) -> usize {
  queue.map_or(0, |queue| ptr::from_ref(queue).expose_provenance())
}

// Object.swift

pub fn iousbhost_object_destroy(this: &NSObject) {
  ffi::iousbhost_object_destroy(borrowed(this));
}

/// `options` is an `IOUSBHostObjectDestroyOptions`' raw value.
pub fn iousbhost_object_destroy_options(this: &NSObject, options: usize) {
  ffi::iousbhost_object_destroy_options(borrowed(this), options);
}

pub fn iousbhost_object_io_service(this: &NSObject) -> u32 {
  ffi::iousbhost_object_io_service(borrowed(this))
}

pub fn iousbhost_object_queue(this: &NSObject) -> DispatchRetained<DispatchQueue> {
  let queue = ffi::iousbhost_object_queue(borrowed(this));

  // SAFETY: `queue`, a retained `dispatch_queue_t`, never `nil`.
  unsafe { DispatchRetained::from_raw(NonNull::new_unchecked(queue.cast())) }
}

/// The descriptor's bytes.
pub fn iousbhost_object_device_descriptor(this: &NSObject) -> Option<Vec<u8>> {
  // SAFETY: a `bridging(descriptor:length:)` result, or `0`.
  unsafe { descriptor_or_nil(ffi::iousbhost_object_device_descriptor(borrowed(this))) }
}

/// The BOS descriptor's bytes, its device capability descriptors with it.
pub fn iousbhost_object_capability_descriptors(this: &NSObject) -> Option<Vec<u8>> {
  // SAFETY: a `bridging(descriptor:length:)` result, or `0`.
  unsafe { descriptor_or_nil(ffi::iousbhost_object_capability_descriptors(borrowed(this))) }
}

/// The descriptor's bytes, its interface and endpoint descriptors with it.
pub fn iousbhost_object_configuration_descriptor_with(
  this: &NSObject,
  index: isize,
) -> Result<Vec<u8>, Retained<NSError>> {
  let address = ffi::iousbhost_object_configuration_descriptor_with(borrowed(this), index).map_err(thrown)?;

  // SAFETY: a `bridging(descriptor:length:)` result.
  Ok(unsafe { descriptor(address) })
}

/// The descriptor's bytes, its interface and endpoint descriptors with it.
pub fn iousbhost_object_configuration_descriptor_with_configuration_value(
  this: &NSObject,
  configuration_value: isize,
) -> Result<Vec<u8>, Retained<NSError>> {
  let address =
    ffi::iousbhost_object_configuration_descriptor_with_configuration_value(borrowed(this), configuration_value)
      .map_err(thrown)?;

  // SAFETY: a `bridging(descriptor:length:)` result.
  Ok(unsafe { descriptor(address) })
}

/// The descriptor's `length` bytes. `descriptor_type` is `type`, a Rust
/// keyword.
pub fn iousbhost_object_descriptor_with_type(
  this: &NSObject,
  descriptor_type: IOUSBDescriptorType,
  index: isize,
  language_id: isize,
  request_type: IOUSBDeviceRequestTypeValue,
  request_recipient: IOUSBDeviceRequestRecipientValue,
) -> Result<Vec<u8>, Retained<NSError>> {
  let address = ffi::iousbhost_object_descriptor_with_type(
    borrowed(this),
    descriptor_type,
    index,
    language_id,
    request_type,
    request_recipient,
  )
  .map_err(thrown)?;

  // SAFETY: a `bridging(descriptor:length:)` result.
  Ok(unsafe { descriptor(address) })
}

pub fn iousbhost_object_string_with_index(
  this: &NSObject,
  index: isize,
  language_id: isize,
) -> Result<String, Retained<NSError>> {
  ffi::iousbhost_object_string_with_index(borrowed(this), index, language_id).map_err(thrown)
}

pub fn iousbhost_object_io_data_with_capacity(
  this: &NSObject,
  capacity: isize,
) -> Result<Retained<NSMutableData>, Retained<NSError>> {
  let address = ffi::iousbhost_object_io_data_with_capacity(borrowed(this), capacity).map_err(thrown)?;

  // SAFETY: a retained `NSMutableData`.
  Ok(unsafe { mutable_data(address) })
}

/// `options` is an `IOUSBHostObjectDataOptions`' raw value.
pub fn iousbhost_object_data_with_capacity_options(
  this: &NSObject,
  capacity: isize,
  options: usize,
) -> Result<Retained<NSMutableData>, Retained<NSError>> {
  let address = ffi::iousbhost_object_data_with_capacity_options(borrowed(this), capacity, options).map_err(thrown)?;

  // SAFETY: a retained `NSMutableData`.
  Ok(unsafe { mutable_data(address) })
}

/// Returns `bytesTransferred`.
pub fn iousbhost_object_send_device_request(
  this: &NSObject,
  request: BridgedIOUSBDeviceRequest,
  data: Option<&NSMutableData>,
  completion_timeout: f64,
) -> Result<isize, Retained<NSError>> {
  ffi::iousbhost_object_send_device_request(borrowed(this), request, borrowed_or_nil(data), completion_timeout)
    .map_err(thrown)
}

/// `IOUSBHostCompletionHandler`: an `IOReturn` and the bytes transferred.
pub type IOUSBHostCompletionHandler = Box<dyn FnOnce(i32, isize) + Send + 'static>;

pub fn iousbhost_object_enqueue_device_request(
  this: &NSObject,
  request: BridgedIOUSBDeviceRequest,
  data: Option<&NSMutableData>,
  completion_timeout: f64,
  completion_handler: Option<IOUSBHostCompletionHandler>,
) -> Result<(), Retained<NSError>> {
  let Some(completion_handler) = completion_handler else {
    return ffi::iousbhost_object_enqueue_device_request_nil_completion_handler(
      borrowed(this),
      request,
      borrowed_or_nil(data),
      completion_timeout,
    )
    .map_err(thrown);
  };

  let error = ffi::iousbhost_object_enqueue_device_request(
    borrowed(this),
    request,
    borrowed_or_nil(data),
    completion_timeout,
    completion_handler,
  );

  // SAFETY: `bridging(thrownAddress:)`, a retained `NSError` or `0`.
  match unsafe { retained_or_nil(error) } {
    Some(error) => Err(error),
    None => Ok(()),
  }
}

pub fn iousbhost_object_abort_device_requests_with_option(
  this: &NSObject,
  option: IOUSBHostAbortOption,
) -> Result<(), Retained<NSError>> {
  ffi::iousbhost_object_abort_device_requests_with_option(borrowed(this), option).map_err(thrown)
}

pub fn iousbhost_object_device_address(this: &NSObject) -> isize {
  ffi::iousbhost_object_device_address(borrowed(this))
}

/// The frame number and `time`.
pub fn iousbhost_object_frame_number_with_time(this: &NSObject) -> (u64, u64) {
  let frame = ffi::iousbhost_object_frame_number_with_time(borrowed(this));

  (frame.number, frame.time)
}

/// The microframe number and `time`.
pub fn iousbhost_object_current_microframe_with_time(this: &NSObject) -> Result<(u64, u64), Retained<NSError>> {
  let frame = ffi::iousbhost_object_current_microframe_with_time(borrowed(this)).map_err(thrown)?;

  Ok((frame.number, frame.time))
}

/// The microframe number and `time`.
pub fn iousbhost_object_reference_microframe_with_time(this: &NSObject) -> Result<(u64, u64), Retained<NSError>> {
  let frame = ffi::iousbhost_object_reference_microframe_with_time(borrowed(this)).map_err(thrown)?;

  Ok((frame.number, frame.time))
}

// Device.swift

/// `options` is an `IOUSBHostObjectInitOptions`' raw value.
pub fn iousbhost_device_init_io_service_options_queue_interest_handler(
  io_service: u32,
  options: usize,
  queue: Option<&DispatchQueue>,
  interest_handler: Option<IOUSBHostInterestHandler>,
) -> Result<IOUSBHostDevice, Retained<NSError>> {
  let queue = borrowed_queue_or_nil(queue);

  match interest_handler {
    Some(handler) => ffi::iousbhost_device_init_io_service_options_queue_interest_handler(
      io_service,
      options,
      queue,
      IOUSBHostInterestHandlerBridge { handler },
    ),
    None => ffi::iousbhost_device_init_io_service_options_queue_nil_interest_handler(io_service, options, queue),
  }
  .map_err(thrown)
}

/// `createMatchingDictionaryWithVendorID:…`'s arguments, each `None` for
/// `nil`: more than clippy lets a function take.
pub struct IOUSBHostDeviceMatchingDictionaryArguments<'a> {
  pub vendor_id: Option<i64>,
  pub product_id: Option<i64>,
  pub bcd_device: Option<i64>,
  pub device_class: Option<i64>,
  pub device_subclass: Option<i64>,
  pub device_protocol: Option<i64>,
  pub speed: Option<i64>,
  pub product_id_array: Option<&'a [i64]>,
}

/// The dictionary's entries.
pub fn iousbhost_device_create_matching_dictionary(
  arguments: &IOUSBHostDeviceMatchingDictionaryArguments<'_>,
) -> BTreeMap<String, PropertyListValue> {
  let number = |value: Option<i64>| value.map(NSNumber::new_i64);
  let vendor_id = number(arguments.vendor_id);
  let product_id = number(arguments.product_id);
  let bcd_device = number(arguments.bcd_device);
  let device_class = number(arguments.device_class);
  let device_subclass = number(arguments.device_subclass);
  let device_protocol = number(arguments.device_protocol);
  let speed = number(arguments.speed);
  let product_id_array = arguments.product_id_array.map(|product_ids| {
    let product_ids: Vec<_> = product_ids
      .iter()
      .map(|product_id| NSNumber::new_i64(*product_id))
      .collect();

    NSArray::from_retained_slice(&product_ids)
  });

  let dictionary = ffi::iousbhost_device_create_matching_dictionary(ffi::BridgedIOUSBHostMatchingDictionaryArguments {
    vendor_id: borrowed_or_nil(vendor_id.as_deref()),
    product_id: borrowed_or_nil(product_id.as_deref()),
    bcd_device: borrowed_or_nil(bcd_device.as_deref()),
    device_class: borrowed_or_nil(device_class.as_deref()),
    device_subclass: borrowed_or_nil(device_subclass.as_deref()),
    device_protocol: borrowed_or_nil(device_protocol.as_deref()),
    speed: borrowed_or_nil(speed.as_deref()),
    product_id_array: borrowed_or_nil(product_id_array.as_deref()),
  });

  // SAFETY: a retained `NSDictionary` of `String`s to property list objects.
  property_list_dictionary(&*unsafe { retained::<NSDictionary<NSString, AnyObject>>(dictionary) })
}

/// Another reference to the `IOUSBHostDevice` at `device`.
///
/// # Safety
///
/// `device` is an `IOUSBHostDevice`, alive for the call: e.g. one another
/// bridge's completion handler was given.
pub unsafe fn iousbhost_device_bridge_from(device: *const c_void) -> IOUSBHostDevice {
  ffi::iousbhost_device_bridge_from(device)
}

pub fn iousbhost_device_bridge_cast(object: &NSObject) -> Option<IOUSBHostDevice> {
  ffi::iousbhost_device_bridge_cast(borrowed(object)).ok()
}

impl IOUSBHostDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: `bridge_borrowed` is this device's own object, which `self`
    // holds, and every IOUSBHost class is an `NSObject`.
    unsafe { &*self.bridge_borrowed().cast::<NSObject>() }
  }

  /// The descriptor's bytes, its interface and endpoint descriptors with it.
  pub fn iousbhost_configuration_descriptor(&self) -> Option<Vec<u8>> {
    // SAFETY: a `bridging(descriptor:length:)` result, or `0`.
    unsafe { descriptor_or_nil(ffi::iousbhost_device_configuration_descriptor(self.bridge_retained())) }
  }

  pub fn iousbhost_configure_with_value(&self, value: isize, match_interfaces: bool) -> Result<(), Retained<NSError>> {
    self
      .bridge_configure_with_value(value, match_interfaces)
      .map_err(thrown)
  }

  pub fn iousbhost_reset(&self) -> Result<(), Retained<NSError>> {
    self.bridge_reset().map_err(thrown)
  }
}

/// `IOUSBHostInterestHandler`: the device, the message type, and its
/// argument's address (or the argument itself).
pub type IOUSBHostInterestHandler = Box<dyn Fn(IOUSBHostDevice, u32, usize) + Send + Sync + 'static>;

/// An [`IOUSBHostInterestHandler`] as Swift holds it.
pub struct IOUSBHostInterestHandlerBridge {
  handler: IOUSBHostInterestHandler,
}

impl IOUSBHostInterestHandlerBridge {
  fn call(&self, host_object: IOUSBHostDevice, message_type: u32, message_argument: usize) {
    (self.handler)(host_object, message_type, message_argument);
  }
}
