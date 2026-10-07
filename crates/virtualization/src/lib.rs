//! Rust bindings for Apple's [Virtualization] framework.
//!
//! Virtualization as Apple ships it, with nothing added. Names drop `VZ` and
//! members are snake case; Apple's documentation is this crate's.
//!
//! Where Rust differs from Swift:
//!
//! - Objects are shared references, as in Swift: [`Clone`] is the same
//!   object, setters take `&self`, and nothing is `Send` or `Sync`. [`Debug`]
//!   is the object's `description`.
//! - An abstract class is a sealed trait, implemented by its subclasses. A
//!   member that returns one takes the subclass to cast it to, as Swift's
//!   `as?`, and returns `None` when it is another.
//! - `URL` is [`url::Url`], re-exported as [`url`].
//! - DiskImageKit's types are [`disk_image_kit`]'s, re-exported.
//! - An enum has an `Unknown(rawValue)` case, for one newer than this crate.
//! - Initializers, class members, throwing members, and members that take or
//!   return a URL return `Result<_, Error>`.
//!
//! Off macOS, the same API compiles, and every initializer and class member
//! fails with [`Error::Unsupported`].
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

mod boot_loader;
mod error;
mod foundation;
mod platform;
mod sealed;
mod storage;
mod virtual_machine;

pub use disk_image_kit;
pub use url;

pub use crate::boot_loader::{BootLoader, LinuxBootLoader};
pub use crate::error::{Error, NSError};
pub use crate::storage::{
  DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode, StorageDeviceAttachment,
  StorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
};
pub use crate::virtual_machine::VirtualMachine;
