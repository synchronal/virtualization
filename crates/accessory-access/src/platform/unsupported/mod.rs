//! accessory-access-sys's names and signatures, off macOS, grouped as its
//! Swift package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.)

mod error;
mod foundation;
mod usb_accessory;
mod usb_accessory_manager;
mod usb_accessory_matching_criteria;

pub(crate) use self::error::*;
pub(crate) use self::foundation::*;
pub(crate) use self::usb_accessory::*;
pub(crate) use self::usb_accessory_manager::*;
pub(crate) use self::usb_accessory_matching_criteria::*;

pub(crate) enum Never {}

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
