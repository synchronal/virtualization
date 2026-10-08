use std::fmt;

use crate::NetworkDeviceAttachment;
use crate::platform::ffi;

/// [`VZNetworkDevice`](https://developer.apple.com/documentation/virtualization/vznetworkdevice)
pub struct NetworkDevice {
  inner: ffi::VZNetworkDevice,
}

impl NetworkDevice {
  pub(crate) fn bridge_from(inner: ffi::VZNetworkDevice) -> Self {
    Self { inner }
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vznetworkdevice/attachment),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn attachment<T: NetworkDeviceAttachment>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_attachment()?)
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vznetworkdevice/attachment)
  pub fn set_attachment(&self, attachment: Option<&dyn NetworkDeviceAttachment>) {
    self
      .inner
      .vz_set_attachment(attachment.map(|attachment| attachment.bridge_object()));
  }
}

impl Clone for NetworkDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NetworkDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
