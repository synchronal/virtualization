mod file_handle_serial_port_attachment;
mod file_serial_port_attachment;
mod spice_agent_port_attachment;
mod virtio_console_device;
mod virtio_console_device_configuration;
mod virtio_console_device_serial_port_configuration;
mod virtio_console_port;
mod virtio_console_port_array;
mod virtio_console_port_configuration;
mod virtio_console_port_configuration_array;

pub use self::file_handle_serial_port_attachment::FileHandleSerialPortAttachment;
pub use self::file_serial_port_attachment::FileSerialPortAttachment;
pub use self::spice_agent_port_attachment::SpiceAgentPortAttachment;
pub use self::virtio_console_device::{VirtioConsoleDevice, VirtioConsoleDeviceDelegate};
pub use self::virtio_console_device_configuration::VirtioConsoleDeviceConfiguration;
pub use self::virtio_console_device_serial_port_configuration::VirtioConsoleDeviceSerialPortConfiguration;
pub use self::virtio_console_port::VirtioConsolePort;
pub use self::virtio_console_port_array::VirtioConsolePortArray;
pub use self::virtio_console_port_configuration::VirtioConsolePortConfiguration;
pub use self::virtio_console_port_configuration_array::VirtioConsolePortConfigurationArray;
use crate::platform::ffi;
use crate::sealed::Sealed;

/// [`VZConsolePortConfiguration`](https://developer.apple.com/documentation/virtualization/vzconsoleportconfiguration)
pub trait ConsolePortConfiguration: Sealed {
  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzconsoleportconfiguration/attachment),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  fn attachment<T: SerialPortAttachment>(&self) -> Option<T>
  where
    Self: Sized,
  {
    T::bridge_cast(&*ffi::vz_console_port_configuration_attachment(self.bridge_object())?)
  }

  /// [`attachment`](https://developer.apple.com/documentation/virtualization/vzconsoleportconfiguration/attachment)
  fn set_attachment(&self, attachment: Option<&dyn SerialPortAttachment>) {
    ffi::vz_console_port_configuration_set_attachment(
      self.bridge_object(),
      attachment.map(|attachment| attachment.bridge_object()),
    );
  }
}

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
