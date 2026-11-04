//! Rust bindings for Apple's [AccessoryAccess] framework: the USB accessories
//! Virtualization's `VZUSBPassthroughDeviceConfiguration` captures for a
//! virtual machine.
//!
//! AccessoryAccess as Apple ships it, with nothing added. Names drop `AA`
//! (`AAUSBAccessory` is [`USBAccessory`]) and a nested type joins its names
//! (`AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption` is
//! [`USBAccessoryMatchingCriteriaInterfaceMatchingOption`]); members are snake
//! case. Apple's documentation is this crate's.
//!
//! Every call needs the `com.apple.developer.accessory-access.usb`
//! entitlement, and `AAUSBAccessoryManager` an app with a UI. Without them,
//! AccessoryAccess fails as it fails.
//!
//! Where Rust differs from Swift:
//!
//! - Objects are shared references, as in Swift: [`Clone`] is the same
//!   object. Each class is `Sendable`, so `Send` and `Sync`. [`Debug`] is the
//!   object's `description`.
//! - A listener is a trait object. The bridge holds it from registering until
//!   it is unregistered: AccessoryAccess knows a listener by its object, so
//!   the bridge passes it the same one for the same `Arc`.
//! - A `[String: any Sendable]` is a `BTreeMap` of [`PropertyListValue`]s.
//!   `IOUSBHostDevice` is [`io_usb_host::Device`], and a dispatch queue
//!   [`dispatch2`]'s, both re-exported.
//! - A completion handler is a closure.
//! - An enum has an `Unknown(rawValue)` case, for one newer than this crate.
//! - Swift's `async` forms aren't bridged, so neither is `events(matching:)`,
//!   which has no other, nor the `AAUSBAccessory.Event` it yields. The XPC
//!   representation and `NSSecureCoding` take XPC objects and coders, which
//!   aren't either.
//! - Initializers and class members return `Result<_, Error>`.
//!
//! Off macOS, the same API compiles, and every initializer and class member
//! fails with [`Error::Unsupported`].
//!
//! [AccessoryAccess]: https://developer.apple.com/documentation/accessoryaccess

mod error;
mod foundation;
mod platform;
mod usb_accessory;
mod usb_accessory_listener;
mod usb_accessory_manager;
mod usb_accessory_matching_criteria;

pub use apple_bridge_foundation::PropertyListValue;
pub use {dispatch2, io_usb_host};

pub use crate::error::{Error, ErrorCode, NSError, error_domain};
pub use crate::usb_accessory::USBAccessory;
pub use crate::usb_accessory_listener::USBAccessoryListener;
pub use crate::usb_accessory_manager::USBAccessoryManager;
pub use crate::usb_accessory_matching_criteria::{
  USBAccessoryMatchingCriteria, USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
  USBAccessoryMatchingCriteriaInterfaceMatchingCriteria, USBAccessoryMatchingCriteriaInterfaceMatchingOption,
};

// Every class is `Sendable`, so `Send + Sync`, on macOS or off it.
#[cfg(test)]
apple_bridge_foundation::assert_auto_traits! {
  send_sync: [USBAccessory, USBAccessoryManager, USBAccessoryMatchingCriteria],
  local: [],
}

/// What another crate's bridge passes on, e.g. Virtualization's
/// `VZUSBPassthroughDeviceConfiguration(device:)`. Not for callers.
#[doc(hidden)]
pub mod bridge {
  /// The `accessory-access-sys` object behind `usb_accessory`.
  #[cfg(target_os = "macos")]
  pub fn usb_accessory(usb_accessory: &crate::USBAccessory) -> &accessory_access_sys::AAUSBAccessory {
    &usb_accessory.inner
  }

  /// Off macOS no `USBAccessory` exists: this is never called.
  #[cfg(not(target_os = "macos"))]
  pub fn usb_accessory(usb_accessory: &crate::USBAccessory) -> &crate::USBAccessory {
    usb_accessory
  }
}
