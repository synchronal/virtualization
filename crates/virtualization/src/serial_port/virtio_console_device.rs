use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use crate::platform::ffi;
use crate::sealed::Sealed;
use crate::{ConsoleDevice, VirtioConsolePort, VirtioConsolePortArray};

/// [`VZVirtioConsoleDevice`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevice)
pub struct VirtioConsoleDevice {
  inner: ffi::VZVirtioConsoleDevice,
}

impl VirtioConsoleDevice {
  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevice/delegate):
  /// `None` when there is none, or it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn VirtioConsoleDeviceDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevice/delegate),
  /// held weakly, as in Swift.
  pub fn set_delegate(&self, delegate: Option<&Arc<dyn VirtioConsoleDeviceDelegate>>) {
    self.inner.vz_set_delegate(
      delegate
        .map(|delegate| Arc::new(Delegating(Arc::downgrade(delegate))) as Arc<dyn ffi::VZVirtioConsoleDeviceDelegate>),
    );
  }

  /// [`ports`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevice/ports)
  pub fn ports(&self) -> VirtioConsolePortArray {
    VirtioConsolePortArray::bridge_from(self.inner.vz_ports())
  }
}

impl Clone for VirtioConsoleDevice {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioConsoleDevice {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for VirtioConsoleDevice {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_virtio_console_device_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl ConsoleDevice for VirtioConsoleDevice {}

/// [`VZVirtioConsoleDeviceDelegate`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevicedelegate)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing.
/// Virtualization calls them on the machine's queue.
pub trait VirtioConsoleDeviceDelegate: Send + Sync {
  /// [`consoleDevice(_:didOpen:)`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevicedelegate/consoledevice(_:didopen:))
  fn console_device_did_open(&self, _console_device: &VirtioConsoleDevice, _console_port: &VirtioConsolePort) {}

  /// [`consoleDevice(_:didClose:)`](https://developer.apple.com/documentation/virtualization/vzvirtioconsoledevicedelegate/consoledevice(_:didclose:))
  fn console_device_did_close(&self, _console_device: &VirtioConsoleDevice, _console_port: &VirtioConsolePort) {}
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn VirtioConsoleDeviceDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZVirtioConsoleDeviceDelegate for Delegating {
  fn console_device_did_open(
    &self,
    console_device: ffi::VZVirtioConsoleDevice,
    console_port: ffi::VZVirtioConsolePort,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.console_device_did_open(
        &VirtioConsoleDevice { inner: console_device },
        &VirtioConsolePort::bridge_from(console_port),
      );
    }
  }

  fn console_device_did_close(
    &self,
    console_device: ffi::VZVirtioConsoleDevice,
    console_port: ffi::VZVirtioConsolePort,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.console_device_did_close(
        &VirtioConsoleDevice { inner: console_device },
        &VirtioConsolePort::bridge_from(console_port),
      );
    }
  }
}
