//! Rust bindings for Apple's [DiskImageKit] framework.
//!
//! DiskImageKit as Apple ships it, with nothing added. Members are snake
//! case; Apple's documentation is this crate's.
//!
//! Where Rust differs from Swift:
//!
//! - Objects are shared references, as in Swift: [`Clone`] is the same
//!   object. Only what Apple declares `Sendable` is `Send` and `Sync`.
//!   [`Debug`] is the value's `String(describing:)`.
//! - A nested type joins its names: `DiskImage.Format` is [`DiskImageFormat`].
//! - A protocol is a trait. One only DiskImageKit's own types conform to is
//!   sealed.
//! - `URL` is [`url::Url`] and `UUID` is [`uuid::Uuid`].
//! - An enum has an `Unknown` case, for one newer than this crate.
//! - Initializers, static members, throwing members, and members that take or
//!   return a URL return `Result<_, Error>`. What DiskImageKit throws is its
//!   own error type, or an [`NSError`].
//!
//! Off macOS, the same API compiles, and every initializer and static member
//! fails with [`Error::Unsupported`].
//!
//! [DiskImageKit]: https://developer.apple.com/documentation/diskimagekit

mod creation_configuration;
mod disk_image;
mod error;
mod foundation;
mod open_configuration;
mod platform;
mod sealed;

pub use crate::creation_configuration::{
  ASIFCreationConfiguration, ASIFLayerCreationConfiguration, CreationConfiguration, RAWCreationConfiguration,
  StackableLayer,
};
pub use crate::disk_image::{DiskImage, DiskImageBlockSize, DiskImageFormat, DiskImageLayerType, StackedImage};
pub use crate::error::{
  CorruptedImageError, Error, IncompatibleStackingError, InvalidBlockCountError, NSError, UnsupportedFormatError,
};
pub use crate::open_configuration::{OpenConfiguration, OpenConfigurationMode, OpenConfigurationProtocol};

// Each type is `Send + Sync` off macOS exactly where disk-image-kit-sys makes
// it so on macOS.
#[cfg(test)]
apple_bridge_foundation::assert_auto_traits! {
  send_sync: [DiskImageLayerType],
  local: [
    ASIFCreationConfiguration,
    ASIFLayerCreationConfiguration,
    DiskImage,
    RAWCreationConfiguration,
    StackedImage,
  ],
}

/// What another crate's bridge passes on, e.g. `virtualization`'s
/// `VZDiskImageStorageDeviceAttachment(diskImage:…)`. Not for callers.
#[doc(hidden)]
pub mod bridge {
  /// The `disk-image-kit-sys` object behind `disk_image`.
  #[cfg(target_os = "macos")]
  pub fn disk_image(disk_image: &crate::DiskImage) -> &disk_image_kit_sys::DiskImage {
    disk_image.bridged()
  }

  /// Off macOS no `DiskImage` exists: this is never called.
  #[cfg(not(target_os = "macos"))]
  pub fn disk_image(disk_image: &crate::DiskImage) -> &crate::DiskImage {
    disk_image
  }
}
