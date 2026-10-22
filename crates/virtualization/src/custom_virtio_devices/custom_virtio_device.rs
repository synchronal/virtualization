use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use dispatch2::{DispatchQueue, DispatchRetained};

use crate::platform::ffi;
use crate::{
  Error, GuestMemoryMapping, NegotiatedVirtioFeatureSet, VirtioDeviceSpecificConfiguration, VirtioQueue,
  VirtioSharedMemoryRegion, foundation,
};

/// [`VZCustomVirtioDevice`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice)
pub struct CustomVirtioDevice {
  pub(crate) inner: ffi::VZCustomVirtioDevice,
}

impl CustomVirtioDevice {
  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/delegate):
  /// `None` when there is none, or it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn CustomVirtioDeviceDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/delegate),
  /// held weakly, as in Swift.
  pub fn set_delegate(&self, delegate: Option<&Arc<dyn CustomVirtioDeviceDelegate>>) {
    self.inner.vz_set_delegate(
      delegate
        .map(|delegate| Arc::new(Delegating(Arc::downgrade(delegate))) as Arc<dyn ffi::VZCustomVirtioDeviceDelegate>),
    );
  }

  /// [`deviceQueue`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/devicequeue)
  pub fn device_queue(&self) -> DispatchRetained<DispatchQueue> {
    self.inner.vz_device_queue()
  }

  /// [`negotiatedFeatures`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/negotiatedfeatures)
  pub fn negotiated_features(&self) -> Option<NegotiatedVirtioFeatureSet> {
    self
      .inner
      .vz_negotiated_features()
      .ok()
      .map(|inner| NegotiatedVirtioFeatureSet { inner })
  }

  /// [`sharedMemoryRegions`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/sharedmemoryregions)
  pub fn shared_memory_regions(&self) -> Vec<VirtioSharedMemoryRegion> {
    self
      .inner
      .vz_shared_memory_regions()
      .into_iter()
      .map(|inner| VirtioSharedMemoryRegion { inner })
      .collect()
  }

  /// [`guestMemoryMapping(atPhysicalAddress:length:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/guestmemorymapping(atphysicaladdress:length:))
  pub fn guest_memory_mapping(&self, physical_address: u64, length: isize) -> Option<GuestMemoryMapping> {
    self
      .inner
      .vz_guest_memory_mapping(physical_address, length)
      .ok()
      .map(|inner| GuestMemoryMapping { inner })
  }

  /// [`queue(at:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/queue(at:))
  pub fn queue(&self, index: u16) -> Option<VirtioQueue> {
    self
      .inner
      .vz_queue(index)
      .ok()
      .map(|inner| VirtioQueue { inner })
  }

  /// [`requestReset()`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/requestreset())
  pub fn request_reset(&self) {
    self.inner.vz_request_reset();
  }

  /// [`update(_:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevice/update(_:completionhandler:))
  pub fn update(
    &self,
    new_configuration: &VirtioDeviceSpecificConfiguration,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) {
    self
      .inner
      .vz_update(&new_configuration.inner, move |error| {
        completion_handler(foundation::completed(error));
      });
  }
}

impl Clone for CustomVirtioDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for CustomVirtioDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZCustomVirtioDeviceDelegate`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing, but
/// for the save and restore methods, which fail the save and the restore.
/// Virtualization calls them on the device's queue.
pub trait CustomVirtioDeviceDelegate: Send + Sync {
  /// [`customVirtioDevice(_:didReceiveNotificationFor:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevice(_:didreceivenotificationfor:))
  fn custom_virtio_device_did_receive_notification_for(&self, _device: &CustomVirtioDevice, _queue: &VirtioQueue) {}

  /// [`customVirtioDeviceDidAcceptDriverOk(_:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicedidacceptdriverok(_:))
  fn custom_virtio_device_did_accept_driver_ok(&self, _device: &CustomVirtioDevice) {}

  /// [`customVirtioDeviceWillStop(_:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicewillstop(_:))
  fn custom_virtio_device_will_stop(&self, _device: &CustomVirtioDevice) {}

  /// [`customVirtioDeviceWillPause(_:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicewillpause(_:))
  fn custom_virtio_device_will_pause(&self, _device: &CustomVirtioDevice) {}

  /// [`customVirtioDeviceWillResume(_:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicewillresume(_:))
  fn custom_virtio_device_will_resume(&self, _device: &CustomVirtioDevice) {}

  /// [`customVirtioDeviceWillReset(_:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicewillreset(_:)).
  fn custom_virtio_device_will_reset(&self, _device: &CustomVirtioDevice) {}

  /// [`customVirtioDeviceSaveState(forRestore:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodevicesavestate(forrestore:)):
  /// `Data` is its bytes. Unimplemented, it is `None`, which fails the save.
  fn custom_virtio_device_save_state_for_restore(&self, _device: &CustomVirtioDevice) -> Option<Vec<u8>> {
    None
  }

  /// [`customVirtioDeviceShouldRestore(_:saveState:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegate/customvirtiodeviceshouldrestore(_:savestate:)):
  /// `Data` is its bytes. Unimplemented, it is `false`, which fails the
  /// restore.
  fn custom_virtio_device_should_restore_save_state(&self, _device: &CustomVirtioDevice, _save_state: &[u8]) -> bool {
    false
  }
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn CustomVirtioDeviceDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZCustomVirtioDeviceDelegate for Delegating {
  fn custom_virtio_device_did_receive_notification_for(
    &self,
    device: ffi::VZCustomVirtioDevice,
    queue: ffi::VZVirtioQueue,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_did_receive_notification_for(
        &CustomVirtioDevice { inner: device },
        &VirtioQueue { inner: queue },
      );
    }
  }

  fn custom_virtio_device_did_accept_driver_ok(&self, device: ffi::VZCustomVirtioDevice) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_did_accept_driver_ok(&CustomVirtioDevice { inner: device });
    }
  }

  fn custom_virtio_device_will_stop(&self, device: ffi::VZCustomVirtioDevice) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_will_stop(&CustomVirtioDevice { inner: device });
    }
  }

  fn custom_virtio_device_will_pause(&self, device: ffi::VZCustomVirtioDevice) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_will_pause(&CustomVirtioDevice { inner: device });
    }
  }

  fn custom_virtio_device_will_resume(&self, device: ffi::VZCustomVirtioDevice) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_will_resume(&CustomVirtioDevice { inner: device });
    }
  }

  fn custom_virtio_device_will_reset(&self, device: ffi::VZCustomVirtioDevice) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_device_will_reset(&CustomVirtioDevice { inner: device });
    }
  }

  fn custom_virtio_device_save_state_for_restore(&self, device: ffi::VZCustomVirtioDevice) -> Option<Vec<u8>> {
    self
      .0
      .upgrade()?
      .custom_virtio_device_save_state_for_restore(&CustomVirtioDevice { inner: device })
  }

  fn custom_virtio_device_should_restore_save_state(
    &self,
    device: ffi::VZCustomVirtioDevice,
    save_state: Vec<u8>,
  ) -> bool {
    self.0.upgrade().is_some_and(|delegate| {
      delegate.custom_virtio_device_should_restore_save_state(&CustomVirtioDevice { inner: device }, &save_state)
    })
  }
}
