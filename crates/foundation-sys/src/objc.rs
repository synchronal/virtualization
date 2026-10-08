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

use std::ffi::c_void;
use std::os::fd::{IntoRawFd, OwnedFd, RawFd};

use objc2::AllocAnyThread;
pub use objc2::rc::Retained;
pub use objc2::runtime::{AnyObject, NSObject};
pub use objc2_foundation::{NSArray, NSData, NSDictionary, NSError, NSFileHandle, NSString, NSURL, NSUUID};

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
