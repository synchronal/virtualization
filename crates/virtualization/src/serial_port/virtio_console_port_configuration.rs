use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{ConsolePortConfiguration, Error};

/// [`VZVirtioConsolePortConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration)
pub struct VirtioConsolePortConfiguration {
  pub(crate) inner: ffi::VZVirtioConsolePortConfiguration,
}

impl VirtioConsolePortConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtio_console_port_configuration_init(),
    })
  }

  /// [`name`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration/name)
  pub fn name(&self) -> Option<String> {
    self.inner.vz_name()
  }

  /// [`name`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration/name)
  pub fn set_name(&self, name: Option<&str>) {
    self.inner.vz_set_name(name);
  }

  /// [`isConsole`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration/isconsole)
  pub fn is_console(&self) -> bool {
    self.inner.vz_is_console()
  }

  /// [`isConsole`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfiguration/isconsole)
  pub fn set_is_console(&self, is_console: bool) {
    self.inner.vz_set_is_console(is_console);
  }
}

impl Clone for VirtioConsolePortConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsolePortConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioConsolePortConfiguration {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_console_port_configuration_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl ConsolePortConfiguration for VirtioConsolePortConfiguration {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtioConsolePortConfiguration::new(), Err(Error::Unsupported)));
  }
}
