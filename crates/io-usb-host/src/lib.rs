//! Rust bindings for Apple's [IOUSBHost] framework: `IOUSBHostDevice`, which
//! AccessoryAccess's `AAUSBAccessory.open(serviceQueue:completionHandler:)`
//! opens, and the `IOUSBHostObject` it is.
//!
//! IOUSBHost as Apple ships it, with nothing added. Names drop `IOUSBHost` and
//! `IOUSB` (`IOUSBHostDevice` is [`Device`], `IOUSBDeviceRequest`
//! [`DeviceRequest`]); members are snake case. Apple documents most members
//! only in Objective-C, so their names are the selector's, without `error:`
//! (`sendDeviceRequest:data:bytesTransferred:completionTimeout:error:` is
//! [`Object::send_device_request`]). Apple's documentation is this crate's.
//!
//! Where Rust differs from Objective-C:
//!
//! - Objects are shared references: [`Clone`] is the same object, and nothing
//!   is `Send` or `Sync`. [`Debug`] is the object's `description`.
//! - `IOUSBHostObject`, the base class, is the sealed trait [`Object`], which
//!   [`Device`] implements.
//! - A descriptor IOUSBHost returns as a pointer into memory the object owns
//!   is a copy of its bytes: `bLength` of them, or `wTotalLength` for a
//!   configuration or BOS descriptor, so the descriptors it holds come along.
//! - An I/O buffer is [`NSMutableData`]; a matching dictionary is a
//!   `BTreeMap` of [`PropertyListValue`]s; a dispatch queue is
//!   [`dispatch2`]'s, re-exported. `NSNumber`s are `i64`s.
//! - An `NSError **` is a `Result`. An out parameter is returned: alone, or
//!   in a tuple after the result.
//! - A handler is a closure. An `IOReturn` is its `i32`.
//! - An option set is a struct of its raw value; an enum has an
//!   `Unknown(rawValue)` case, for one newer than this crate.
//! - Initializers, class members, constants, and members that fail return
//!   `Result<_, Error>`.
//!
//! Off macOS, the same API compiles, and every initializer, class member, and
//! constant fails with [`Error::Unsupported`].
//!
//! [IOUSBHost]: https://developer.apple.com/documentation/iousbhost

mod definitions;
mod device;
mod error;
mod foundation;
mod object;
mod platform;
mod sealed;

pub use dispatch2;
pub use foundation_sys::{NSMutableData, PropertyListValue};

pub use crate::definitions::{
  DescriptorType, DevicePropertyKey, DeviceRequest, DeviceRequestDirectionValue, DeviceRequestRecipientValue,
  DeviceRequestTypeValue, MatchingPropertyKey, default_control_completion_timeout, device_request_type, error_domain,
  property_key_location_id,
};
pub use crate::device::{Device, DeviceMatchingDictionaryArguments, InterestHandler};
pub use crate::error::{Error, NSError};
pub use crate::object::{
  AbortOption, CompletionHandler, Object, ObjectDataOptions, ObjectDestroyOptions, ObjectInitOptions,
};

/// What another crate's bridge passes on, e.g. AccessoryAccess's
/// `open(serviceQueue:completionHandler:)`. Not for callers.
#[doc(hidden)]
pub mod bridge {
  /// A `Device` of the `io-usb-host-sys` object another bridge returned.
  #[cfg(target_os = "macos")]
  pub fn device_from(inner: io_usb_host_sys::IOUSBHostDevice) -> crate::Device {
    crate::Device { inner }
  }

  /// Off macOS no `Device` exists: this is never called.
  #[cfg(not(target_os = "macos"))]
  pub fn device_from(device: crate::Device) -> crate::Device {
    device
  }
}
