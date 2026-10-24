//! disk-image-kit-sys's names and signatures, off macOS, grouped as its Swift
//! package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.) One that isn't `Send` and `Sync`
//! on macOS holds a [`Local`] too, so its auto traits are the same here.

use std::marker::PhantomData;

mod creation_configuration;
mod disk_image;
mod foundation;
mod open_configuration;

pub(crate) use self::creation_configuration::*;
pub(crate) use self::disk_image::*;
pub(crate) use self::foundation::*;
pub(crate) use self::open_configuration::*;

pub(crate) enum Never {}

/// Held beside a [`Never`] by each class disk-image-kit-sys doesn't make
/// `Send` and `Sync`, so it is neither here either.
pub(crate) type Local = PhantomData<*const ()>;

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
