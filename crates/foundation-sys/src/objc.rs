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

use std::ffi::c_void;

pub use objc2::rc::Retained;
pub use objc2::runtime::AnyObject;
use objc2::runtime::NSObject;
use objc2_foundation::NSString;
pub use objc2_foundation::{NSError, NSURL, NSUUID};

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
