//! Rust bindings for Apple's [vmnet] framework: its networks, which
//! Virtualization's `VZVmnetNetworkDeviceAttachment` connects a virtual
//! machine to.
//!
//! vmnet as Apple ships it, with nothing added. Apple's documentation is this
//! crate's.
//!
//! Where Rust differs from C:
//!
//! - A reference type is a type named for it, without `vmnet_` or `_ref`
//!   (`vmnet_network_ref` is [`Network`]), and its functions are its methods,
//!   named without its prefix (`vmnet_network_configuration_set_mtu` is
//!   [`NetworkConfiguration::set_mtu`]), and without `get_`. The function
//!   that creates one is `new`.
//! - References are shared, as in C: [`Clone`] is the same object, setters
//!   take `&self`, and nothing is `Send` or `Sync`. [`Debug`] is the object's
//!   Core Foundation description. Each releases its reference when dropped.
//! - A [`Return`] other than `Success` is an [`Error`]. A function that
//!   returns `NULL` with a `status` returns `Result<_, Error>`.
//! - `in_addr` and `in6_addr` are [`std::net`]'s addresses; `ether_addr_t`
//!   is its six octets. A pair of out parameters is a tuple returned.
//! - An enum has an `Unknown(rawValue)` case, for one newer than this crate.
//!
//! Off macOS, the same API compiles, and every function that creates a
//! reference fails with [`Error::Unsupported`].
//!
//! [vmnet]: https://developer.apple.com/documentation/vmnet

mod error;
mod mode;
mod network;
mod network_configuration;
mod platform;

pub use crate::error::{Error, Return};
pub use crate::mode::Mode;
pub use crate::network::Network;
pub use crate::network_configuration::NetworkConfiguration;

/// What another crate's bridge passes on, e.g. `virtualization`'s
/// `VZVmnetNetworkDeviceAttachment(network:)`. Not for callers.
#[doc(hidden)]
pub mod bridge {
  /// The `vmnet-sys` object behind `network`.
  #[cfg(target_os = "macos")]
  pub fn network(network: &crate::Network) -> &vmnet_sys::VmnetNetwork {
    &network.inner
  }

  /// A `Network` of the `vmnet-sys` object another bridge returned.
  #[cfg(target_os = "macos")]
  pub fn network_from(inner: vmnet_sys::VmnetNetwork) -> crate::Network {
    crate::Network { inner }
  }

  /// Off macOS no `Network` exists: this is never called.
  #[cfg(not(target_os = "macos"))]
  pub fn network(network: &crate::Network) -> &crate::Network {
    network
  }

  /// Off macOS no `Network` exists: this is never called.
  #[cfg(not(target_os = "macos"))]
  pub fn network_from(network: crate::Network) -> crate::Network {
    network
  }
}
