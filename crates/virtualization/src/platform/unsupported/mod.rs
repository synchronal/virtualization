//! virtualization-sys's names and signatures, off macOS, grouped as its Swift
//! package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.)

mod boot_loader;
mod foundation;
mod network;
mod serial_port;
mod storage;
mod virtual_machine;

pub(crate) use self::boot_loader::*;
pub(crate) use self::foundation::*;
pub(crate) use self::network::*;
pub(crate) use self::serial_port::*;
pub(crate) use self::storage::*;
pub(crate) use self::virtual_machine::*;

pub(crate) enum Never {}

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
