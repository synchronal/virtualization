//! FFI to Apple's [AccessoryAccess] framework, through swift-bridge. Use the
//! `accessory-access` crate instead; this one holds its `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them, the way
//! `virtualization-sys`'s are:
//!
//! - Classes keep Apple's names; a nested type joins them
//!   (`AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption` is
//!   [`AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption`]). Members are
//!   methods `aa_<member>`; initializers and class members are free functions
//!   `aa_<type>_init[_<labels>]` and `aa_<type>_<member>`.
//! - Foundation's objects are objc2's, through `foundation-sys`, and cross as
//!   pointers. A `[String: any Sendable]` is a property list dictionary.
//! - IOUSBHost's `IOUSBHostDevice` is `io-usb-host-sys`'s, crossing by
//!   address.
//! - A listener is the trait [`AAUSBAccessoryListener`], which Swift holds
//!   while it is registered: AccessoryAccess knows a listener by its object,
//!   so the bridge keeps one per Rust listener, by a key Rust gives.
//! - Enums keep Apple's cases, plus `Unknown(rawValue)` for a case newer than
//!   this crate.
//!
//! The glue's names differ from the other bridges': all link into one binary,
//! and swift-bridge names symbols for each bridge's types.
//!
//! macOS only: the build fails elsewhere.
//!
//! [AccessoryAccess]: https://developer.apple.com/documentation/accessoryaccess

use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use dispatch2::DispatchQueue;
use foundation_sys::PropertyListValue;
use foundation_sys::objc::{
  NSArray, NSData, NSObject, borrowed, borrowed_or_nil, ns_array, ns_array_objects, ns_data_bytes,
  ns_property_list_dictionary, retained, retained_or_nil,
};
pub use foundation_sys::objc::{NSError, Retained};
use io_usb_host_sys::IOUSBHostDevice;

pub use self::ffi::{
  AAErrorCode, AAUSBAccessory, AAUSBAccessoryManager, AAUSBAccessoryMatchingCriteria,
  AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption, aa_error_code_init_raw_value, aa_error_code_raw_value,
  aa_error_domain, aa_usb_accessory_manager_shared,
};

// No doc comments inside the module: swift-bridge can't parse them. Shared
// enums are renamed on the Swift side, where Apple's names are taken.
//
// A function with a Foundation pointer takes the object it is a member of by
// value: swift-bridge 0.1.59 releases an object a free function borrows.
#[swift_bridge::bridge]
mod ffi {
  // Foundation.swift

  enum AccessoryAccessNil {
    Nil,
  }

  // Error.swift

  #[swift_bridge(swift_name = "BridgedAAErrorCode")]
  enum AAErrorCode {
    InternalError,
    AccessoryListenerAlreadyRegistered,
    AccessoryNotAccessible,
    InvalidAccessoryState,
    Unknown(isize),
  }

  extern "Swift" {
    fn aa_error_domain() -> String;
    fn aa_error_code_init_raw_value(raw_value: isize) -> AAErrorCode;
    fn aa_error_code_raw_value(code: AAErrorCode) -> isize;
  }

  // USBAccessory.swift

  extern "Swift" {
    type AAUSBAccessory;

    fn bridge_retained(self: &AAUSBAccessory) -> AAUSBAccessory;
    fn bridge_borrowed(self: &AAUSBAccessory) -> *const c_void;
    fn ns_description(self: &AAUSBAccessory) -> String;
    fn aa_usb_accessory_bridge_from(object: *const c_void) -> AAUSBAccessory;
    fn aa_registry_id(self: &AAUSBAccessory) -> u64;
    fn aa_usb_accessory_device_descriptor_data(this: AAUSBAccessory) -> *mut c_void;
    fn aa_usb_accessory_configuration_descriptor_data(this: AAUSBAccessory) -> usize;
    fn aa_usb_accessory_open(
      this: AAUSBAccessory,
      service_queue: usize,
      completion_handler: Box<dyn FnOnce(usize, usize)>,
    );
    fn aa_usb_accessory_close(this: AAUSBAccessory, completion_handler: Box<dyn FnOnce(usize)>);
  }

  // USBAccessoryListener.swift

  extern "Rust" {
    type AAUSBAccessoryListenerBridge;

    fn usb_accessory_did_connect(self: &AAUSBAccessoryListenerBridge, usb_accessory: AAUSBAccessory);
    fn usb_accessory_did_disconnect(self: &AAUSBAccessoryListenerBridge, usb_accessory: AAUSBAccessory);
  }

  // USBAccessoryManager.swift

  extern "Swift" {
    type AAUSBAccessoryManager;

    fn aa_usb_accessory_manager_shared() -> AAUSBAccessoryManager;
    fn bridge_retained(self: &AAUSBAccessoryManager) -> AAUSBAccessoryManager;
    fn ns_description(self: &AAUSBAccessoryManager) -> String;
    fn aa_usb_accessory_manager_register_listener(
      this: AAUSBAccessoryManager,
      listener: AAUSBAccessoryListenerBridge,
      key: usize,
      matching_criteria: *const c_void,
      completion_handler: Box<dyn FnOnce(usize, usize)>,
    );
    fn aa_usb_accessory_manager_unregister_listener(
      this: AAUSBAccessoryManager,
      listener: AAUSBAccessoryListenerBridge,
      key: usize,
      completion_handler: Box<dyn FnOnce()>,
    );
  }

  // USBAccessoryMatchingCriteria.swift

  #[swift_bridge(swift_name = "BridgedInterfaceMatchingOption")]
  enum AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption {
    MatchAll,
    MatchAny,
    Unknown(isize),
  }

  extern "Swift" {
    type AAUSBAccessoryMatchingCriteria;

    fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary(
      dictionary: *const c_void,
    ) -> Result<AAUSBAccessoryMatchingCriteria, AccessoryAccessNil>;
    fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
      device_matching_dictionary: usize,
      interface_matching_dictionaries: *const c_void,
      interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
    ) -> Result<AAUSBAccessoryMatchingCriteria, AccessoryAccessNil>;
    fn aa_usb_accessory_matching_criteria_init_device_matching_criteria(
      device_matching_criteria: *const c_void,
    ) -> AAUSBAccessoryMatchingCriteria;
    fn aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
      device_matching_criteria: usize,
      interface_matching_criteria: *const c_void,
      interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
    ) -> AAUSBAccessoryMatchingCriteria;
    fn bridge_retained(self: &AAUSBAccessoryMatchingCriteria) -> AAUSBAccessoryMatchingCriteria;
    fn bridge_borrowed(self: &AAUSBAccessoryMatchingCriteria) -> *const c_void;
    fn ns_description(self: &AAUSBAccessoryMatchingCriteria) -> String;
  }
}

// Foundation.swift

/// An object of a class this bridge declares, as an `NSObject`, for as long
/// as `owner`, which holds it.
///
/// # Safety
///
/// `pointer` is what `bridge_borrowed()` returned for an object `owner`
/// holds.
unsafe fn bridge_object<O>(_owner: &O, pointer: *const c_void) -> &NSObject {
  // SAFETY: the caller's: `owner` keeps the object alive while it is
  // borrowed, and every AccessoryAccess class is an `NSObject`.
  unsafe { &*pointer.cast::<NSObject>() }
}

/// An optional dispatch queue, lent to Swift by address.
fn borrowed_queue_or_nil(queue: Option<&DispatchQueue>) -> usize {
  queue.map_or(0, |queue| ptr::from_ref(queue).expose_provenance())
}

/// What a completion handler failed with, which Swift handed over by
/// address: `0` for nothing.
fn completed_error(error: usize) -> Option<Retained<NSError>> {
  // SAFETY: `bridging(retainedError:)`, a retained `NSError` or `0`.
  unsafe { retained_or_nil(error) }
}

// USBAccessory.swift

// SAFETY: Apple declares `AAUSBAccessory` `Sendable`.
unsafe impl Send for AAUSBAccessory {}

// SAFETY: as for `Send`.
unsafe impl Sync for AAUSBAccessory {}

impl AAUSBAccessory {
  pub fn aa_device_descriptor_data(&self) -> Vec<u8> {
    // SAFETY: `deviceDescriptorData`, a retained `NSData`.
    ns_data_bytes(&*unsafe { retained::<NSData>(ffi::aa_usb_accessory_device_descriptor_data(self.bridge_retained())) })
  }

  pub fn aa_configuration_descriptor_data(&self) -> Option<Vec<u8>> {
    // SAFETY: `configurationDescriptorData`, a retained `NSData` or `0`.
    unsafe {
      retained_or_nil::<NSData>(ffi::aa_usb_accessory_configuration_descriptor_data(
        self.bridge_retained(),
      ))
    }
    .map(|data| ns_data_bytes(&data))
  }

  /// `service_queue` `None` is `nil`. The device crosses to
  /// `io-usb-host-sys`'s bridge.
  pub fn aa_open(
    &self,
    service_queue: Option<&DispatchQueue>,
    completion_handler: impl FnOnce(Result<IOUSBHostDevice, Retained<NSError>>) + Send + 'static,
  ) {
    ffi::aa_usb_accessory_open(
      self.bridge_retained(),
      borrowed_queue_or_nil(service_queue),
      Box::new(move |device, error| {
        // SAFETY: `bridging(retainedAddress:)`, a retained `IOUSBHostDevice`,
        // or `0` when it failed.
        let device: Option<Retained<NSObject>> = unsafe { retained_or_nil(device) };

        completion_handler(match device {
          // SAFETY: an `IOUSBHostDevice`, which `device` keeps alive for the
          // call.
          Some(device) => Ok(unsafe { io_usb_host_sys::iousbhost_device_bridge_from(borrowed(&*device)) }),
          None => Err(completed_error(error).expect("Swift passes a device or an error")),
        });
      }),
    );
  }

  pub fn aa_close(&self, completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    ffi::aa_usb_accessory_close(
      self.bridge_retained(),
      Box::new(move |error| completion_handler(completed_error(error))),
    );
  }
}

// USBAccessoryListener.swift

/// `AAUSBAccessoryListener`, which Rust implements. AccessoryAccess calls it
/// on an arbitrary thread.
pub trait AAUSBAccessoryListener: Send + Sync {
  fn usb_accessory_did_connect(&self, usb_accessory: AAUSBAccessory);

  fn usb_accessory_did_disconnect(&self, usb_accessory: AAUSBAccessory);
}

/// An [`AAUSBAccessoryListener`] as Swift holds it.
pub struct AAUSBAccessoryListenerBridge {
  listener: Arc<dyn AAUSBAccessoryListener>,
}

impl AAUSBAccessoryListenerBridge {
  fn usb_accessory_did_connect(&self, usb_accessory: AAUSBAccessory) {
    self.listener.usb_accessory_did_connect(usb_accessory);
  }

  fn usb_accessory_did_disconnect(&self, usb_accessory: AAUSBAccessory) {
    self.listener.usb_accessory_did_disconnect(usb_accessory);
  }
}

// USBAccessoryManager.swift

// SAFETY: Apple declares `AAUSBAccessoryManager` `Sendable`.
unsafe impl Send for AAUSBAccessoryManager {}

// SAFETY: as for `Send`.
unsafe impl Sync for AAUSBAccessoryManager {}

impl AAUSBAccessoryManager {
  /// `key` names `listener` to the bridge, which passes AccessoryAccess the
  /// same object for each listener with the same key.
  pub fn aa_register_listener(
    &self,
    listener: Arc<dyn AAUSBAccessoryListener>,
    key: usize,
    matching_criteria: &[&AAUSBAccessoryMatchingCriteria],
    completion_handler: impl FnOnce(Result<Vec<AAUSBAccessory>, Retained<NSError>>) + Send + 'static,
  ) {
    let matching_criteria: Vec<_> = matching_criteria
      .iter()
      .map(|criteria| criteria.bridge_object())
      .collect();

    ffi::aa_usb_accessory_manager_register_listener(
      self.bridge_retained(),
      AAUSBAccessoryListenerBridge { listener },
      key,
      borrowed(&*ns_array(&matching_criteria)),
      Box::new(move |accessories, error| {
        if let Some(error) = completed_error(error) {
          return completion_handler(Err(error));
        }

        // SAFETY: `bridging(retainedAddress:)`, a retained `NSArray` of
        // `AAUSBAccessory`s.
        let accessories =
          unsafe { retained_or_nil::<NSArray<NSObject>>(accessories) }.expect("Swift passes the accessories");

        completion_handler(Ok(
          ns_array_objects(&accessories)
            .iter()
            .map(|accessory| ffi::aa_usb_accessory_bridge_from(borrowed(&**accessory)))
            .collect(),
        ));
      }),
    );
  }

  /// `key` names `listener` as [`Self::aa_register_listener`] did.
  pub fn aa_unregister_listener(
    &self,
    listener: Arc<dyn AAUSBAccessoryListener>,
    key: usize,
    completion_handler: impl FnOnce() + Send + 'static,
  ) {
    ffi::aa_usb_accessory_manager_unregister_listener(
      self.bridge_retained(),
      AAUSBAccessoryListenerBridge { listener },
      key,
      Box::new(completion_handler),
    );
  }
}

// USBAccessoryMatchingCriteria.swift

/// `AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria`: each `None` is
/// `nil`.
pub struct AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  pub vendor_id: Option<isize>,
  pub product_id: Option<isize>,
  pub device_class: Option<isize>,
  pub device_sub_class: Option<isize>,
  pub device_protocol: Option<isize>,
  pub speed: Option<isize>,
}

impl AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  /// Its non-`nil` fields by their Swift names, as Swift reads it.
  fn bridge_dictionary(&self) -> BTreeMap<String, PropertyListValue> {
    criteria_dictionary([
      ("vendorID", self.vendor_id),
      ("productID", self.product_id),
      ("deviceClass", self.device_class),
      ("deviceSubClass", self.device_sub_class),
      ("deviceProtocol", self.device_protocol),
      ("speed", self.speed),
    ])
  }
}

/// `AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria`: each `None` is
/// `nil`.
pub struct AAUSBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
  pub vendor_id: Option<isize>,
  pub product_id: Option<isize>,
  pub bcd_device: Option<isize>,
  pub interface_number: Option<isize>,
  pub configuration_value: Option<isize>,
  pub interface_class: Option<isize>,
  pub interface_sub_class: Option<isize>,
  pub interface_protocol: Option<isize>,
  pub speed: Option<isize>,
}

impl AAUSBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
  /// Its non-`nil` fields by their Swift names, as Swift reads it.
  fn bridge_dictionary(&self) -> BTreeMap<String, PropertyListValue> {
    criteria_dictionary([
      ("vendorID", self.vendor_id),
      ("productID", self.product_id),
      ("bcdDevice", self.bcd_device),
      ("interfaceNumber", self.interface_number),
      ("configurationValue", self.configuration_value),
      ("interfaceClass", self.interface_class),
      ("interfaceSubClass", self.interface_sub_class),
      ("interfaceProtocol", self.interface_protocol),
      ("speed", self.speed),
    ])
  }
}

/// A matching criteria struct's non-`nil` fields.
fn criteria_dictionary<const N: usize>(fields: [(&str, Option<isize>); N]) -> BTreeMap<String, PropertyListValue> {
  fields
    .into_iter()
    .filter_map(|(name, value)| Some((name.to_string(), PropertyListValue::Integer(value? as i64))))
    .collect()
}

pub fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary(
  dictionary: &BTreeMap<String, PropertyListValue>,
) -> Option<AAUSBAccessoryMatchingCriteria> {
  ffi::aa_usb_accessory_matching_criteria_init_device_matching_dictionary(borrowed(&*ns_property_list_dictionary(
    dictionary,
  )))
  .ok()
}

pub fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
  device_matching_dictionary: Option<&BTreeMap<String, PropertyListValue>>,
  interface_matching_dictionaries: &[BTreeMap<String, PropertyListValue>],
  interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
) -> Option<AAUSBAccessoryMatchingCriteria> {
  let device_matching_dictionary = device_matching_dictionary.map(ns_property_list_dictionary);
  let interface_matching_dictionaries: Vec<_> = interface_matching_dictionaries
    .iter()
    .map(ns_property_list_dictionary)
    .collect();
  let interface_matching_dictionaries: Vec<_> = interface_matching_dictionaries
    .iter()
    .map(|dictionary| &**dictionary)
    .collect();

  ffi::aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
    borrowed_or_nil(device_matching_dictionary.as_deref()),
    borrowed(&*ns_array(&interface_matching_dictionaries)),
    interface_matching_option,
  )
  .ok()
}

pub fn aa_usb_accessory_matching_criteria_init_device_matching_criteria(
  device_matching_criteria: &AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria,
) -> AAUSBAccessoryMatchingCriteria {
  ffi::aa_usb_accessory_matching_criteria_init_device_matching_criteria(borrowed(&*ns_property_list_dictionary(
    &device_matching_criteria.bridge_dictionary(),
  )))
}

pub fn aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
  device_matching_criteria: Option<&AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria>,
  interface_matching_criteria: &[AAUSBAccessoryMatchingCriteriaInterfaceMatchingCriteria],
  interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
) -> AAUSBAccessoryMatchingCriteria {
  let device_matching_criteria =
    device_matching_criteria.map(|criteria| ns_property_list_dictionary(&criteria.bridge_dictionary()));
  let interface_matching_criteria: Vec<_> = interface_matching_criteria
    .iter()
    .map(|criteria| ns_property_list_dictionary(&criteria.bridge_dictionary()))
    .collect();
  let interface_matching_criteria: Vec<_> = interface_matching_criteria
    .iter()
    .map(|criteria| &**criteria)
    .collect();

  ffi::aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
    borrowed_or_nil(device_matching_criteria.as_deref()),
    borrowed(&*ns_array(&interface_matching_criteria)),
    interface_matching_option,
  )
}

// SAFETY: Apple declares `AAUSBAccessoryMatchingCriteria` `Sendable`.
unsafe impl Send for AAUSBAccessoryMatchingCriteria {}

// SAFETY: as for `Send`.
unsafe impl Sync for AAUSBAccessoryMatchingCriteria {}

impl AAUSBAccessoryMatchingCriteria {
  fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}
