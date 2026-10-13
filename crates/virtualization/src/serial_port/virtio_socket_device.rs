use std::fmt;

use crate::platform::ffi;
use crate::sealed::Sealed;
use crate::{Error, SocketDevice, VirtioSocketConnection, VirtioSocketListener, foundation};

/// [`VZVirtioSocketDevice`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdevice)
pub struct VirtioSocketDevice {
  pub(crate) inner: ffi::VZVirtioSocketDevice,
}

impl VirtioSocketDevice {
  /// [`setSocketListener(_:forPort:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdevice/setsocketlistener(_:forport:))
  pub fn set_socket_listener(&self, listener: &VirtioSocketListener, port: u32) {
    self
      .inner
      .vz_set_socket_listener(listener.inner.bridge_retained(), port);
  }

  /// [`removeSocketListener(forPort:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdevice/removesocketlistener(forport:))
  pub fn remove_socket_listener(&self, port: u32) {
    self.inner.vz_remove_socket_listener(port);
  }

  /// [`connect(toPort:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosocketdevice/connect(toport:completionhandler:))
  pub fn connect(
    &self,
    port: u32,
    completion_handler: impl FnOnce(Result<VirtioSocketConnection, Error>) + Send + 'static,
  ) {
    self.inner.vz_connect(port, move |result| {
      completion_handler(match result {
        Ok(inner) => Ok(VirtioSocketConnection { inner }),
        Err(error) => Err(foundation::error(&error)),
      });
    });
  }
}

impl Clone for VirtioSocketDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSocketDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioSocketDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_socket_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SocketDevice for VirtioSocketDevice {}
