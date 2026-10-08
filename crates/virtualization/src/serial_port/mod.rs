mod file_handle_serial_port_attachment;
mod file_serial_port_attachment;
mod virtio_console_device_serial_port_configuration;

pub use self::file_handle_serial_port_attachment::FileHandleSerialPortAttachment;
pub use self::file_serial_port_attachment::FileSerialPortAttachment;
pub use self::virtio_console_device_serial_port_configuration::VirtioConsoleDeviceSerialPortConfiguration;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZSerialPortConfiguration`](https://developer.apple.com/documentation/virtualization/vzserialportconfiguration)
pub trait SerialPortConfiguration: Sealed {
  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzserialportconfiguration/attachment),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  fn attachment<T: SerialPortAttachment>(&self) -> Option<T>
  where
    Self: Sized,
  {
    T::bridge_cast(&*ffi::vz_serial_port_configuration_attachment(self.bridge_object())?)
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzserialportconfiguration/attachment)
  fn set_attachment(&self, attachment: Option<&dyn SerialPortAttachment>) {
    ffi::vz_serial_port_configuration_set_attachment(
      self.bridge_object(),
      attachment.map(|attachment| attachment.bridge_object()),
    );
  }
}

/// [`VZSerialPortAttachment`](https://developer.apple.com/documentation/virtualization/vzserialportattachment)
pub trait SerialPortAttachment: Sealed {}

/// [`VZConsoleDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzconsoledeviceconfiguration)
pub trait ConsoleDeviceConfiguration: Sealed {}

/// [`VZSocketDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzsocketdeviceconfiguration)
pub trait SocketDeviceConfiguration: Sealed {}

/// [`VZConsoleDevice`](https://developer.apple.com/documentation/virtualization/vzconsoledevice)
pub trait ConsoleDevice: Sealed {}

/// [`VZSocketDevice`](https://developer.apple.com/documentation/virtualization/vzsocketdevice)
pub trait SocketDevice: Sealed {}
