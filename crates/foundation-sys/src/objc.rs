//! Foundation's objects, through objc2, and the glue for passing them
//! through swift-bridge.
//!
//! swift-bridge passes a pointer, never an `Option` of one (0.1.59 panics), so
//! an optional object crosses as a `usize` address, `0` for `nil`. Each
//! package's `Foundation.swift` has the Swift half:
//!
//! - `bridging(borrowed:)` / `bridging(borrowedAddress:)`: an object Rust
//!   lends for one call, made here with [`borrowed`] / [`borrowed_or_nil`].
//! - `bridging(retained:)` / `bridging(retainedAddress:)`: an object Swift
//!   hands over, taken here with [`retained`] / [`retained_or_nil`].
//! - `bridging(borrowedArray:)`: an array Rust lends, made here with
//!   [`ns_array`] and lent with [`borrowed`]. Swift returns one as an
//!   `NSArray`, read here with [`ns_array_objects`].
//! - `bridging(borrowedDictionary:)`: a `[String: T]` Rust lends, made here
//!   with [`ns_dictionary`]. Swift returns one as an `NSDictionary`, read here
//!   with [`ns_dictionary_entries`].

use std::collections::BTreeMap;
use std::ffi::{CStr, c_void};
use std::os::fd::{IntoRawFd, OwnedFd, RawFd};
use std::ptr::NonNull;

use objc2::AllocAnyThread;
pub use objc2::rc::Retained;
pub use objc2::runtime::{AnyObject, NSObject};
use objc2_foundation::NSRange;
pub use objc2_foundation::{
  NSArray, NSData, NSDictionary, NSError, NSFileHandle, NSMutableData, NSNull, NSNumber, NSString, NSURL, NSUUID,
};

use crate::PropertyListValue;

/// A Foundation object, lent to Swift for one call.
pub fn borrowed<T: objc2::Message>(object: &T) -> *const c_void {
  std::ptr::from_ref(object).cast()
}

/// An optional Foundation object, lent to Swift for one call, by address.
pub fn borrowed_or_nil<T: objc2::Message>(object: Option<&T>) -> usize {
  object.map_or(0, |object| borrowed(object).expose_provenance())
}

/// A Foundation object Swift handed over through `bridging(retained:)`.
///
/// # Safety
///
/// `pointer` is what `bridging(retained:)` returned for an object of class
/// `T` (or a subclass), taken once.
pub unsafe fn retained<T: objc2::Message>(pointer: *mut c_void) -> Retained<T> {
  // SAFETY: the caller's: a reference Swift retained for Rust, to a `T`.
  unsafe { Retained::from_raw(pointer.cast()) }.expect("Swift passes a non-null pointer")
}

/// An optional Foundation object Swift handed over through
/// `bridging(retainedAddress:)`.
///
/// # Safety
///
/// `address` is what `bridging(retainedAddress:)` returned for an optional
/// object of class `T` (or a subclass), taken once.
pub unsafe fn retained_or_nil<T: objc2::Message>(address: usize) -> Option<Retained<T>> {
  // SAFETY: the caller's; `0` is `nil`, anything else a retained `T`.
  (address != 0).then(|| unsafe { retained(std::ptr::with_exposed_provenance_mut(address)) })
}

/// An `NSArray` of `objects`, as Swift bridges an array.
pub fn ns_array<T: objc2::Message>(objects: &[&T]) -> Retained<NSArray<T>> {
  NSArray::from_slice(objects)
}

/// An `NSArray`'s objects, in order.
pub fn ns_array_objects<T: objc2::Message>(array: &NSArray<T>) -> Vec<Retained<T>> {
  array.to_vec()
}

/// An `NSDictionary` of each name to its object, as Swift bridges a
/// `[String: T]`.
pub fn ns_dictionary<T: objc2::Message>(entries: &[(&str, &T)]) -> Retained<NSDictionary<NSString, T>> {
  let keys: Vec<_> = entries
    .iter()
    .map(|(key, _)| NSString::from_str(key))
    .collect();
  let keys: Vec<_> = keys.iter().map(|key| &**key).collect();
  let objects: Vec<_> = entries.iter().map(|(_, object)| *object).collect();

  NSDictionary::from_slices(&keys, &objects)
}

/// An `NSDictionary`'s names and objects, in no particular order.
pub fn ns_dictionary_entries<T: objc2::Message>(dictionary: &NSDictionary<NSString, T>) -> Vec<(String, Retained<T>)> {
  let (keys, objects) = dictionary.to_vecs();

  keys
    .iter()
    .map(|key| key.to_string())
    .zip(objects)
    .collect()
}

/// An `NSData` of a copy of `bytes`.
pub fn ns_data(bytes: &[u8]) -> Retained<NSData> {
  NSData::with_bytes(bytes)
}

/// A copy of an `NSData`'s bytes.
pub fn ns_data_bytes(data: &NSData) -> Vec<u8> {
  data.to_vec()
}

/// [`replaceBytes(in:withBytes:)`](https://developer.apple.com/documentation/foundation/nsmutabledata/replacebytes(in:withbytes:)),
/// over the `bytes.len()` bytes from `location`.
pub(crate) fn ns_mutable_data_replace_bytes(data: &NSMutableData, location: usize, bytes: &[u8]) {
  // SAFETY: Foundation reads the range's length from `bytes`, which is that
  // long.
  unsafe { data.replaceBytesInRange_withBytes(NSRange::new(location, bytes.len()), NonNull::from(bytes).cast()) }
}

/// A property list value as Foundation's object: `NSNumber`, `NSString`,
/// `NSData`, `NSArray`, or `NSDictionary`.
pub(crate) fn ns_property_list(value: &PropertyListValue) -> Retained<AnyObject> {
  match value {
    PropertyListValue::Boolean(value) => NSNumber::new_bool(*value).into(),
    PropertyListValue::Integer(value) => NSNumber::new_i64(*value).into(),
    PropertyListValue::Real(value) => NSNumber::new_f64(*value).into(),
    PropertyListValue::String(value) => NSString::from_str(value).into(),
    PropertyListValue::Data(value) => NSData::with_bytes(value).into(),
    PropertyListValue::Array(values) => {
      let objects: Vec<_> = values.iter().map(ns_property_list).collect();

      NSArray::from_retained_slice(&objects).into()
    }
    PropertyListValue::Dictionary(entries) => ns_property_list_dictionary(entries).into(),
  }
}

/// A `[String: Any]` property list as an `NSDictionary`.
pub fn ns_property_list_dictionary(
  entries: &BTreeMap<String, PropertyListValue>,
) -> Retained<NSDictionary<NSString, AnyObject>> {
  let keys: Vec<_> = entries.keys().map(|key| NSString::from_str(key)).collect();
  let keys: Vec<_> = keys.iter().map(|key| &**key).collect();
  let objects: Vec<_> = entries.values().map(ns_property_list).collect();
  let objects: Vec<_> = objects.iter().map(|object| &**object).collect();

  NSDictionary::from_slices(&keys, &objects)
}

/// A property list object Foundation holds, as its value.
///
/// # Panics
///
/// When `object` isn't a property list object: IOKit's dictionaries hold
/// nothing else.
pub(crate) fn property_list_value(object: &AnyObject) -> PropertyListValue {
  if let Some(number) = object.downcast_ref::<NSNumber>() {
    return ns_number_value(number);
  }

  if let Some(string) = object.downcast_ref::<NSString>() {
    return PropertyListValue::String(string.to_string());
  }

  if let Some(data) = object.downcast_ref::<NSData>() {
    return PropertyListValue::Data(data.to_vec());
  }

  if let Some(array) = object.downcast_ref::<NSArray>() {
    return PropertyListValue::Array(
      array
        .iter()
        .map(|object| property_list_value(&object))
        .collect(),
    );
  }

  if let Some(dictionary) = object.downcast_ref::<NSDictionary>() {
    let (keys, objects) = dictionary.to_vecs();

    return PropertyListValue::Dictionary(
      keys
        .iter()
        .zip(&objects)
        .map(|(key, object)| (property_list_key(key), property_list_value(object)))
        .collect(),
    );
  }

  panic!("not a property list object: {}", ns_description(object));
}

/// An `NSDictionary` of `String`s to property list objects, as its entries.
pub fn property_list_dictionary(dictionary: &NSDictionary<NSString, AnyObject>) -> BTreeMap<String, PropertyListValue> {
  let (keys, objects) = dictionary.to_vecs();

  keys
    .iter()
    .map(|key| key.to_string())
    .zip(objects.iter().map(|object| property_list_value(object)))
    .collect()
}

/// A property list dictionary's key, which is a string.
fn property_list_key(key: &AnyObject) -> String {
  key
    .downcast_ref::<NSString>()
    .unwrap_or_else(|| panic!("not a property list key: {}", ns_description(key)))
    .to_string()
}

/// A Boolean `NSNumber` is one of Foundation's two Boolean singletons; any
/// other is an integer or, by its Objective-C type, floating-point.
fn ns_number_value(number: &NSNumber) -> PropertyListValue {
  for value in [false, true] {
    if std::ptr::eq(number, &*NSNumber::new_bool(value)) {
      return PropertyListValue::Boolean(value);
    }
  }

  // SAFETY: `objCType` is a C string Foundation keeps for the number's life.
  let objc_type = unsafe { CStr::from_ptr(number.objCType().as_ptr()) };

  match objc_type.to_bytes() {
    b"f" | b"d" => PropertyListValue::Real(number.as_f64()),
    _ => PropertyListValue::Integer(number.as_i64()),
  }
}

/// [`NSURL(string:)`](https://developer.apple.com/documentation/foundation/nsurl/init(string:)).
pub fn ns_url_init_string(string: &str) -> Option<Retained<NSURL>> {
  NSURL::URLWithString(&NSString::from_str(string))
}

/// [`absoluteString`](https://developer.apple.com/documentation/foundation/url/absolutestring),
/// which Swift's `URL` declares non-optional. `NSURL`'s is optional; `nil`
/// is empty.
pub fn ns_url_absolute_string(url: &NSURL) -> String {
  url
    .absoluteString()
    .map(|string| string.to_string())
    .unwrap_or_default()
}

/// [`FileHandle(fileDescriptor:closeOnDealloc:)`](https://developer.apple.com/documentation/foundation/filehandle/init(filedescriptor:closeondealloc:)),
/// closing on dealloc: the handle owns `fd`.
pub fn ns_file_handle_init_file_descriptor_close_on_dealloc(fd: OwnedFd) -> Retained<NSFileHandle> {
  NSFileHandle::initWithFileDescriptor_closeOnDealloc(NSFileHandle::alloc(), fd.into_raw_fd(), true)
}

/// [`fileDescriptor`](https://developer.apple.com/documentation/foundation/filehandle/filedescriptor).
pub fn ns_file_handle_file_descriptor(file_handle: &NSFileHandle) -> RawFd {
  file_handle.fileDescriptor()
}

/// [`NSUUID(uuidString:)`](https://developer.apple.com/documentation/foundation/nsuuid/init(uuidstring:)).
pub fn ns_uuid_init_uuid_string(string: &str) -> Option<Retained<NSUUID>> {
  NSUUID::initWithUUIDString(NSUUID::alloc(), &NSString::from_str(string))
}

/// [`uuidString`](https://developer.apple.com/documentation/foundation/nsuuid/uuidstring).
/// (objc2's `as_bytes` fails its own check of `getUUIDBytes:`'s type encoding
/// on macOS 27.)
pub fn ns_uuid_uuid_string(uuid: &NSUUID) -> String {
  uuid.UUIDString().to_string()
}

/// [`description`](https://developer.apple.com/documentation/objectivec/nsobjectprotocol/description),
/// for an object of any class: objc2 reads it only from an `NSObject`. Any
/// other root class's object is its class and address.
pub fn ns_description(object: &AnyObject) -> String {
  match object.downcast_ref::<NSObject>() {
    Some(object) => format!("{object:?}"),
    None => format!("{object:?}"),
  }
}
