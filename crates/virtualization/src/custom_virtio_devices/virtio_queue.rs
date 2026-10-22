use std::fmt;

use crate::VirtioQueueElement;
use crate::platform::ffi;

/// [`VZVirtioQueue`](https://developer.apple.com/documentation/virtualization/vzvirtioqueue)
pub struct VirtioQueue {
  pub(crate) inner: ffi::VZVirtioQueue,
}

impl VirtioQueue {
  /// [`nextElement()`](https://developer.apple.com/documentation/virtualization/vzvirtioqueue/nextelement())
  pub fn next_element(&self) -> Option<VirtioQueueElement> {
    self
      .inner
      .vz_next_element()
      .ok()
      .map(|inner| VirtioQueueElement { inner })
  }

  /// [`queueIndex`](https://developer.apple.com/documentation/virtualization/vzvirtioqueue/queueindex)
  pub fn queue_index(&self) -> u16 {
    self.inner.vz_queue_index()
  }

  /// [`queueSize`](https://developer.apple.com/documentation/virtualization/vzvirtioqueue/queuesize)
  pub fn queue_size(&self) -> u16 {
    self.inner.vz_queue_size()
  }
}

impl Clone for VirtioQueue {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioQueue {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
