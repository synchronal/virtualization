mod usb_passthrough_device;
mod usb_passthrough_device_configuration;
mod virtio_entropy_device_configuration;
mod virtio_traditional_memory_balloon_device;
mod virtio_traditional_memory_balloon_device_configuration;
mod xhci_controller;
mod xhci_controller_configuration;

use std::any::Any;
use std::sync::{Arc, Weak};

use uuid::Uuid;

pub use self::usb_passthrough_device::USBPassthroughDevice;
pub use self::usb_passthrough_device_configuration::USBPassthroughDeviceConfiguration;
pub use self::virtio_entropy_device_configuration::VirtioEntropyDeviceConfiguration;
pub use self::virtio_traditional_memory_balloon_device::VirtioTraditionalMemoryBalloonDevice;
pub use self::virtio_traditional_memory_balloon_device_configuration::VirtioTraditionalMemoryBalloonDeviceConfiguration;
pub use self::xhci_controller::XHCIController;
pub use self::xhci_controller_configuration::XHCIControllerConfiguration;
use crate::platform::ffi;
use crate::sealed::{Sealed, bridge_cast_each, bridge_objects};
use crate::{Error, foundation};

/// [`VZEntropyDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzentropydeviceconfiguration)
pub trait EntropyDeviceConfiguration: Sealed {}

/// [`VZMemoryBalloonDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzmemoryballoondeviceconfiguration)
pub trait MemoryBalloonDeviceConfiguration: Sealed {}

/// [`VZUSBControllerConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbcontrollerconfiguration)
pub trait USBControllerConfiguration: Sealed {
  /// [`usbDevices`](https://developer.apple.com/documentation/virtualization/vzusbcontrollerconfiguration/usbdevices),
  /// each as Swift's `as? T`: `None` when the device isn't a `T`.
  fn usb_devices<T: USBDeviceConfiguration>(&self) -> Vec<Option<T>>
  where
    Self: Sized,
  {
    bridge_cast_each(ffi::vz_usb_controller_configuration_usb_devices(self.bridge_object()))
  }

  /// [`usbDevices`](https://developer.apple.com/documentation/virtualization/vzusbcontrollerconfiguration/usbdevices)
  fn set_usb_devices(&self, usb_devices: &[&dyn USBDeviceConfiguration]) {
    ffi::vz_usb_controller_configuration_set_usb_devices(self.bridge_object(), &bridge_objects(usb_devices));
  }
}

/// [`VZUSBDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzusbdeviceconfiguration)
pub trait USBDeviceConfiguration: Sealed {
  /// [`uuid`](https://developer.apple.com/documentation/virtualization/vzusbdeviceconfiguration/uuid)
  fn uuid(&self) -> Uuid {
    foundation::uuid(&ffi::vz_usb_device_configuration_uuid(self.bridge_object()))
  }

  /// [`uuid`](https://developer.apple.com/documentation/virtualization/vzusbdeviceconfiguration/uuid)
  fn set_uuid(&self, uuid: &Uuid) {
    ffi::vz_usb_device_configuration_set_uuid(self.bridge_object(), &foundation::ns_uuid(uuid));
  }
}

/// [`VZMemoryBalloonDevice`](https://developer.apple.com/documentation/virtualization/vzmemoryballoondevice)
pub trait MemoryBalloonDevice: Sealed {}

/// [`VZUSBController`](https://developer.apple.com/documentation/virtualization/vzusbcontroller)
pub trait USBController: Sealed {
  /// [`attach(device:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/attach(device:completionhandler:))
  fn attach(&self, device: &dyn USBDevice, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static)
  where
    Self: Sized,
  {
    ffi::vz_usb_controller_attach(self.bridge_object(), device.bridge_object(), move |error| {
      completion_handler(foundation::completed(error));
    });
  }

  /// [`detach(device:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/detach(device:completionhandler:))
  fn detach(&self, device: &dyn USBDevice, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static)
  where
    Self: Sized,
  {
    ffi::vz_usb_controller_detach(self.bridge_object(), device.bridge_object(), move |error| {
      completion_handler(foundation::completed(error));
    });
  }

  /// [`usbDevices`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/usbdevices),
  /// each as Swift's `as? T`: `None` when the device isn't a `T`.
  fn usb_devices<T: USBDevice>(&self) -> Vec<Option<T>>
  where
    Self: Sized,
  {
    bridge_cast_each(ffi::vz_usb_controller_usb_devices(self.bridge_object()))
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/delegate):
  /// `None` when there is none, or it has been dropped.
  fn delegate(&self) -> Option<Arc<dyn USBControllerDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = ffi::vz_usb_controller_delegate(self.bridge_object())?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/delegate),
  /// held weakly, as in Swift.
  fn set_delegate(&self, delegate: Option<&Arc<dyn USBControllerDelegate>>) {
    ffi::vz_usb_controller_set_delegate(
      self.bridge_object(),
      delegate.map(|delegate| Arc::new(Delegating(Arc::downgrade(delegate))) as Arc<dyn ffi::VZUSBControllerDelegate>),
    );
  }
}

/// [`VZUSBController.Delegate`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/delegate-swift.protocol)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing.
pub trait USBControllerDelegate: Send + Sync {
  /// [`usbController(_:usbPassthroughDeviceDidDisconnect:)`](https://developer.apple.com/documentation/virtualization/vzusbcontroller/delegate-swift.protocol/usbcontroller(_:usbpassthroughdevicediddisconnect:)):
  /// `usb_controller` is the controller as its abstract class, as Swift
  /// passes it.
  fn usb_controller_usb_passthrough_device_did_disconnect(
    &self,
    _usb_controller: &dyn USBController,
    _device: &USBPassthroughDevice,
  ) {
  }
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn USBControllerDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZUSBControllerDelegate for Delegating {
  fn usb_controller_usb_passthrough_device_did_disconnect(
    &self,
    usb_controller: ffi::Retained<ffi::NSObject>,
    device: ffi::VZUSBPassthroughDevice,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.usb_controller_usb_passthrough_device_did_disconnect(
        &AnyUSBController { object: usb_controller },
        &USBPassthroughDevice { inner: device },
      );
    }
  }
}

/// A `VZUSBController` of whichever subclass, as a delegate is told of it.
#[cfg(target_os = "macos")]
struct AnyUSBController {
  object: ffi::Retained<ffi::NSObject>,
}

#[cfg(target_os = "macos")]
impl Sealed for AnyUSBController {
  fn bridge_object(&self) -> &ffi::NSObject {
    &self.object
  }

  /// Never asked: no caller can name this type to cast to it.
  fn bridge_cast(_object: &ffi::NSObject) -> Option<Self> {
    None
  }
}

#[cfg(target_os = "macos")]
impl USBController for AnyUSBController {}

/// [`VZUSBDevice`](https://developer.apple.com/documentation/virtualization/vzusbdevice)
pub trait USBDevice: Sealed {
  /// [`usbController`](https://developer.apple.com/documentation/virtualization/vzusbdevice/usbcontroller),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  fn usb_controller<T: USBController>(&self) -> Option<T>
  where
    Self: Sized,
  {
    T::bridge_cast(&*ffi::vz_usb_device_usb_controller(self.bridge_object())?)
  }

  /// [`uuid`](https://developer.apple.com/documentation/virtualization/vzusbdevice/uuid)
  fn uuid(&self) -> Uuid {
    foundation::uuid(&ffi::vz_usb_device_uuid(self.bridge_object()))
  }
}
