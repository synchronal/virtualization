//! vmnet-sys's names and signatures, off macOS, grouped as its Swift package
//! is.
//!
//! Each reference type holds a [`Never`], so no instance exists and its
//! methods are unreachable. Free functions are entry points, which
//! [`super::supported`] fails first. (Empty enums instead would make rustc
//! warn that code after every entry point call is unreachable.)

mod mode;
mod network;
mod network_configuration;
mod r#return;

pub(crate) use self::mode::*;
pub(crate) use self::network::*;
pub(crate) use self::network_configuration::*;
pub(crate) use self::r#return::*;

pub(crate) enum Never {}

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
