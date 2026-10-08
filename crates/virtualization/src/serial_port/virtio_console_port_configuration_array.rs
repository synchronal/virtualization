use std::fmt;

use crate::VirtioConsolePortConfiguration;
use crate::platform::ffi;

/// [`VZVirtioConsolePortConfigurationArray`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray)
pub struct VirtioConsolePortConfigurationArray {
  inner: ffi::VZVirtioConsolePortConfigurationArray,
}

impl VirtioConsolePortConfigurationArray {
  pub(crate) fn bridge_from(inner: ffi::VZVirtioConsolePortConfigurationArray) -> Self {
    Self { inner }
  }

  /// [`maximumPortCount`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray/maximumportcount)
  pub fn maximum_port_count(&self) -> u32 {
    self.inner.vz_maximum_port_count()
  }

  /// [`maximumPortCount`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray/maximumportcount)
  pub fn set_maximum_port_count(&self, maximum_port_count: u32) {
    self.inner.vz_set_maximum_port_count(maximum_port_count);
  }

  /// [`subscript(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray/subscript(_:))
  pub fn get(&self, port_index: isize) -> Option<VirtioConsolePortConfiguration> {
    self
      .inner
      .vz_subscript(port_index)
      .ok()
      .map(|inner| VirtioConsolePortConfiguration { inner })
  }

  /// [`subscript(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray/subscript(_:))
  pub fn set(&self, port_index: isize, value: Option<&VirtioConsolePortConfiguration>) {
    match value {
      Some(value) => self
        .inner
        .vz_set_subscript(port_index, value.inner.bridge_retained()),
      None => self.inner.bridge_set_subscript_nil(port_index),
    }
  }
}

impl Clone for VirtioConsolePortConfigurationArray {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsolePortConfigurationArray {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
