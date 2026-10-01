//! FFI to Apple's [Virtualization] framework, through swift-bridge. Use the
//! `virtualization` crate instead; this one holds every `unsafe` so that one
//! needn't.
//!
//! Each function forwards to exactly one Virtualization API, named
//! `vz_<type>_<member>` in snake case, and the Swift side does nothing else.
//!
//! macOS only: the build fails elsewhere.
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

// No doc comments inside the module: swift-bridge can't parse them.
#[swift_bridge::bridge]
pub mod ffi {
  extern "Swift" {
    // `VZVirtualMachine.isSupported`
    fn vz_virtual_machine_is_supported() -> bool;
  }
}
