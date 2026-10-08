use std::fmt;

use crate::SerialPortAttachment;
use crate::platform::ffi;

/// [`VZVirtioConsolePort`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleport)
pub struct VirtioConsolePort {
  inner: ffi::VZVirtioConsolePort,
}

impl VirtioConsolePort {
  pub(crate) fn bridge_from(inner: ffi::VZVirtioConsolePort) -> Self {
    Self { inner }
  }

  /// [`name`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleport/name)
  pub fn name(&self) -> Option<String> {
    self.inner.vz_name()
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleport/attachment),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn attachment<T: SerialPortAttachment>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_attachment()?)
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleport/attachment)
  pub fn set_attachment(&self, attachment: Option<&dyn SerialPortAttachment>) {
    self
      .inner
      .vz_set_attachment(attachment.map(|attachment| attachment.bridge_object()));
  }
}

impl Clone for VirtioConsolePort {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsolePort {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
