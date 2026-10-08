use std::fmt;

use crate::MemoryBalloonDevice;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZVirtioTraditionalMemoryBalloonDevice`](https://developer.apple.com/documentation/virtualization/vzvirtiotraditionalmemoryballoondevice)
pub struct VirtioTraditionalMemoryBalloonDevice {
  inner: ffi::VZVirtioTraditionalMemoryBalloonDevice,
}

impl VirtioTraditionalMemoryBalloonDevice {
  /// [`targetVirtualMachineMemorySize`](https://developer.apple.com/documentation/virtualization/vzvirtiotraditionalmemoryballoondevice/targetvirtualmachinememorysize)
  pub fn target_virtual_machine_memory_size(&self) -> u64 {
    self.inner.vz_target_virtual_machine_memory_size()
  }

  /// [`targetVirtualMachineMemorySize`](https://developer.apple.com/documentation/virtualization/vzvirtiotraditionalmemoryballoondevice/targetvirtualmachinememorysize)
  pub fn set_target_virtual_machine_memory_size(&self, target_virtual_machine_memory_size: u64) {
    self
      .inner
      .vz_set_target_virtual_machine_memory_size(target_virtual_machine_memory_size);
  }
}

impl Clone for VirtioTraditionalMemoryBalloonDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioTraditionalMemoryBalloonDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioTraditionalMemoryBalloonDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_traditional_memory_balloon_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl MemoryBalloonDevice for VirtioTraditionalMemoryBalloonDevice {}
