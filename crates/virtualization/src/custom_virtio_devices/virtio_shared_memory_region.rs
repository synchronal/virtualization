use std::fmt;

use crate::platform::ffi;
use crate::{Error, MemoryPages, foundation};

/// [`VZVirtioSharedMemoryRegion`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregion)
pub struct VirtioSharedMemoryRegion {
  pub(crate) inner: ffi::VZVirtioSharedMemoryRegion,
}

impl VirtioSharedMemoryRegion {
  /// [`regionID`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregion/regionid)
  pub fn region_id(&self) -> u8 {
    self.inner.vz_region_id()
  }

  /// [`size`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregion/size)
  pub fn size(&self) -> u64 {
    self.inner.vz_size()
  }

  /// [`mapMemory(_:atOffset:size:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregion/mapmemory(_:atoffset:size:completionhandler:)),
  /// of all of `memory`: its length is the size. The region holds `memory`
  /// until an [`unmap_memory`](Self::unmap_memory) covering all of it, called
  /// after this, completes, or mapping it fails, so the guest never sees it
  /// freed.
  pub fn map_memory(
    &self,
    memory: &MemoryPages,
    offset: u64,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) {
    self
      .inner
      .vz_map_memory_pages(&memory.inner, offset, move |error| {
        completion_handler(foundation::completed(error));
      });
  }

  /// [`unmapMemory(atOffset:size:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtiosharedmemoryregion/unmapmemory(atoffset:size:completionhandler:)):
  /// once it completes, the region lets go of the [`MemoryPages`] mapped
  /// wholly within `offset..offset + size` by
  /// [`map_memory`](Self::map_memory) calls made before it.
  pub fn unmap_memory(
    &self,
    offset: u64,
    size: u64,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) {
    self.inner.vz_unmap_memory(offset, size, move |error| {
      completion_handler(foundation::completed(error));
    });
  }
}

impl Clone for VirtioSharedMemoryRegion {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioSharedMemoryRegion {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
