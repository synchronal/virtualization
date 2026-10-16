use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use crate::platform::{self, ffi};
use crate::{Error, VirtioSocketConnection, VirtioSocketDevice};

/// [`VZVirtioSocketListener`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketlistener)
pub struct VirtioSocketListener {
  pub(crate) inner: ffi::VZVirtioSocketListener,
}

impl VirtioSocketListener {
  /// [`init()`](https://developer.apple.com/documentation/objectivec/nsobject-swift.class/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_socket_listener_init(),
    })
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketlistener/delegate):
  /// `None` when there is none, or it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn VirtioSocketListenerDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketlistener/delegate),
  /// held weakly, as in Swift.
  pub fn set_delegate(&self, delegate: Option<&Arc<dyn VirtioSocketListenerDelegate>>) {
    self
      .inner
      .vz_set_delegate(delegate.map(|delegate| {
        Arc::new(Delegating(Arc::downgrade(delegate))) as Arc<dyn ffi::VZVirtioSocketListenerDelegate>
      }));
  }
}

impl Clone for VirtioSocketListener {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSocketListener {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZVirtioSocketListenerDelegate`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketlistenerdelegate)
///
/// Its method is optional, as in Swift: unimplemented, it accepts no
/// connection. Virtualization calls it on the machine's queue.
pub trait VirtioSocketListenerDelegate: Send + Sync {
  /// [`listener(_:shouldAcceptNewConnection:from:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketlistenerdelegate/listener(_:shouldacceptnewconnection:from:))
  fn listener_should_accept_new_connection_from(
    &self,
    _listener: &VirtioSocketListener,
    _connection: &VirtioSocketConnection,
    _socket_device: &VirtioSocketDevice,
  ) -> bool {
    false
  }
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn VirtioSocketListenerDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZVirtioSocketListenerDelegate for Delegating {
  fn listener_should_accept_new_connection_from(
    &self,
    listener: ffi::VZVirtioSocketListener,
    connection: ffi::VZVirtioSocketConnection,
    socket_device: ffi::VZVirtioSocketDevice,
  ) -> bool {
    self.0.upgrade().is_some_and(|delegate| {
      delegate.listener_should_accept_new_connection_from(
        &VirtioSocketListener { inner: listener },
        &VirtioSocketConnection { inner: connection },
        &VirtioSocketDevice { inner: socket_device },
      )
    })
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtioSocketListener::new(), Err(Error::Unsupported)));
  }
}
