mod virtio_entropy_device_configuration;
mod virtio_traditional_memory_balloon_device;
mod virtio_traditional_memory_balloon_device_configuration;
mod xhci_controller;
mod xhci_controller_configuration;

use uuid::Uuid;

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
///
/// Its `delegate` hears only of `VZUSBPassthroughDevice`s, and waits for
/// them.
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
}

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
