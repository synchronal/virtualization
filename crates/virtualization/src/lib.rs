//! Rust bindings for Apple's [Virtualization] framework.
//!
//! This crate is Virtualization as Apple ships it: the same types, members,
//! and behaviour, with nothing added. Names drop the `VZ` prefix and members
//! are snake case; otherwise, Apple's documentation is this crate's.
//!
//! macOS only: elsewhere this crate is empty.
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

#![cfg(target_os = "macos")]

mod virtual_machine;

pub use crate::virtual_machine::VirtualMachine;
