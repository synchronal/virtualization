use crate::sealed::Sealed;

/// [`VZSerialPortConfiguration`](https://developer.apple.com/documentation/virtualization/vzserialportconfiguration)
pub trait SerialPortConfiguration: Sealed {}

/// [`VZConsoleDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzconsoledeviceconfiguration)
pub trait ConsoleDeviceConfiguration: Sealed {}

/// [`VZSocketDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzsocketdeviceconfiguration)
pub trait SocketDeviceConfiguration: Sealed {}
