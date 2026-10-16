//! [Foundation], as the Apple framework sys crates share it.
//!
//! swift-bridge gives each bridge its own copy of every type it declares, so
//! two bridges can't pass the same `NSURL` between them. Foundation's objects
//! are objc2's instead, which any crate can hold:
//!
//! - [`objc`] (macOS only) has them, and the glue for passing them through a
//!   swift-bridge bridge as pointers.
//! - [`NSError`] is a thrown error as plain Rust, the same everywhere, for the
//!   safe crates to report.
//! - [`NSMutableData`] is a buffer a framework reads and writes as itself, and
//!   [`PropertyListValue`] a property list's value, for the safe crates to
//!   re-export.
//!
//! [Foundation]: https://developer.apple.com/documentation/foundation

mod error;
mod mutable_data;
#[cfg(target_os = "macos")]
pub mod objc;
mod property_list;

pub use self::error::NSError;
pub use self::mutable_data::NSMutableData;
pub use self::property_list::PropertyListValue;
