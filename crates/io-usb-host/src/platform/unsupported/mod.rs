//! io-usb-host-sys's names and signatures, off macOS, grouped as its Swift
//! package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.)

mod definitions;
mod device;
mod foundation;
mod object;

pub(crate) use self::definitions::*;
pub(crate) use self::device::*;
pub(crate) use self::foundation::*;
pub(crate) use self::object::*;

pub(crate) enum Never {}

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
