use std::fmt;

use crate::MacHardwareModel;
use crate::platform::ffi;

/// [`VZMacOSConfigurationRequirements`](https://developer.apple.com/documentation/virtualization/vzmacosconfigurationrequirements)
pub struct MacOSConfigurationRequirements {
  pub(crate) inner: ffi::VZMacOSConfigurationRequirements,
}

impl MacOSConfigurationRequirements {
  /// [`hardwareModel`](https://developer.apple.com/documentation/virtualization/vzmacosconfigurationrequirements/hardwaremodel)
  pub fn hardware_model(&self) -> MacHardwareModel {
    MacHardwareModel {
      inner: self.inner.vz_hardware_model(),
    }
  }

  /// [`minimumSupportedCPUCount`](https://developer.apple.com/documentation/virtualization/vzmacosconfigurationrequirements/minimumsupportedcpucount)
  pub fn minimum_supported_cpu_count(&self) -> isize {
    self.inner.vz_minimum_supported_cpu_count()
  }

  /// [`minimumSupportedMemorySize`](https://developer.apple.com/documentation/virtualization/vzmacosconfigurationrequirements/minimumsupportedmemorysize)
  pub fn minimum_supported_memory_size(&self) -> u64 {
    self.inner.vz_minimum_supported_memory_size()
  }
}

impl Clone for MacOSConfigurationRequirements {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacOSConfigurationRequirements {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
