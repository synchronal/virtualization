//! virtualization-sys's names and signatures, off macOS, grouped as its Swift
//! package is.
//!
//! Each class holds a [`Never`], so no instance exists and its methods are
//! unreachable. Free functions are entry points, which [`super::supported`]
//! fails first. (Empty enums instead would make rustc warn that code after
//! every entry point call is unreachable.) One that isn't `Send` and `Sync`
//! on macOS holds a [`Local`] too, so its auto traits are the same here.

use std::marker::PhantomData;

mod app_kit;
mod boot_loader;
mod core_graphics;
mod custom_virtio_devices;
mod directory_sharing;
mod error;
mod foundation;
mod graphics;
mod guest_memory_mapping;
mod guest_provisioning_options;
mod macos;
mod network;
mod other_devices;
mod security;
mod serial_port;
mod storage;
mod virtual_machine;
mod virtual_machine_view;

pub(crate) use self::app_kit::*;
pub(crate) use self::boot_loader::*;
pub(crate) use self::core_graphics::*;
pub(crate) use self::custom_virtio_devices::*;
pub(crate) use self::directory_sharing::*;
pub(crate) use self::error::*;
pub(crate) use self::foundation::*;
pub(crate) use self::graphics::*;
pub(crate) use self::guest_memory_mapping::*;
pub(crate) use self::guest_provisioning_options::*;
pub(crate) use self::macos::*;
pub(crate) use self::network::*;
pub(crate) use self::other_devices::*;
pub(crate) use self::security::*;
pub(crate) use self::serial_port::*;
pub(crate) use self::storage::*;
pub(crate) use self::virtual_machine::*;
pub(crate) use self::virtual_machine_view::*;

#[derive(Clone, Copy)]
pub(crate) enum Never {}

/// Held beside a [`Never`] by each class virtualization-sys doesn't make
/// `Send` and `Sync`, so it is neither here either.
pub(crate) type Local = PhantomData<*const ()>;

const GUARDED: &str = "an entry point called a stand-in without checking `platform::supported`";
