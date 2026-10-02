//! disk-image-kit-sys's names and signatures, off macOS, grouped as its Swift
//! package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.)

mod creation_configuration;
mod disk_image;
mod foundation;
mod open_configuration;

pub(crate) use self::creation_configuration::*;
pub(crate) use self::disk_image::*;
pub(crate) use self::foundation::*;
pub(crate) use self::open_configuration::*;

pub(crate) enum Never {}

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
