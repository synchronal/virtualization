use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use dispatch2::{DispatchQueue, DispatchRetained};

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{CustomVirtioDevice, CustomVirtioDeviceConfiguration, CustomVirtioDeviceProvider, Error};

/// [`VZCustomVirtioDeviceDelegateProvider`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegateprovider)
pub struct CustomVirtioDeviceDelegateProvider {
  inner: ffi::VZCustomVirtioDeviceDelegateProvider,
}

impl CustomVirtioDeviceDelegateProvider {
  /// [`init(deviceQueue:delegate:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegateprovider/init(devicequeue:delegate:)):
  /// `delegate` is held weakly, as in Swift.
  pub fn new(
    device_queue: &DispatchQueue,
    delegate: &Arc<dyn CustomVirtioDeviceConfigurationDelegate>,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_custom_virtio_device_delegate_provider_init(
        device_queue,
        Arc::new(Delegating(Arc::downgrade(delegate))),
      ),
    })
  }

  /// [`deviceQueue`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegateprovider/devicequeue)
  pub fn device_queue(&self) -> DispatchRetained<DispatchQueue> {
    self.inner.vz_device_queue()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodevicedelegateprovider/delegate):
  /// `None` when it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn CustomVirtioDeviceConfigurationDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }
}

impl Clone for CustomVirtioDeviceDelegateProvider {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for CustomVirtioDeviceDelegateProvider {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for CustomVirtioDeviceDelegateProvider {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_custom_virtio_device_delegate_provider_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl CustomVirtioDeviceProvider for CustomVirtioDeviceDelegateProvider {}

/// [`VZCustomVirtioDeviceConfigurationDelegate`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfigurationdelegate)
///
/// Its method is optional, as in Swift: unimplemented, it does nothing.
/// Virtualization calls it on the machine's queue, as it makes the machine.
pub trait CustomVirtioDeviceConfigurationDelegate: Send + Sync {
  /// [`customVirtioConfiguration(_:didCreateDevice:)`](https://developer.apple.com/documentation/virtualization/vzcustomvirtiodeviceconfigurationdelegate/customvirtioconfiguration(_:didcreatedevice:))
  fn custom_virtio_configuration_did_create_device(
    &self,
    _device_configuration: &CustomVirtioDeviceConfiguration,
    _device: &CustomVirtioDevice,
  ) {
  }
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn CustomVirtioDeviceConfigurationDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZCustomVirtioDeviceConfigurationDelegate for Delegating {
  fn custom_virtio_configuration_did_create_device(
    &self,
    device_configuration: ffi::VZCustomVirtioDeviceConfiguration,
    device: ffi::VZCustomVirtioDevice,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.custom_virtio_configuration_did_create_device(
        &CustomVirtioDeviceConfiguration {
          inner: device_configuration,
        },
        &CustomVirtioDevice { inner: device },
      );
    }
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  struct Nothing;

  impl CustomVirtioDeviceConfigurationDelegate for Nothing {}

  #[test]
  fn is_unsupported_off_macos() {
    let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);

    assert!(matches!(
      CustomVirtioDeviceDelegateProvider::new(&DispatchQueue::new("test", None), &delegate),
      Err(Error::Unsupported)
    ));
  }
}
