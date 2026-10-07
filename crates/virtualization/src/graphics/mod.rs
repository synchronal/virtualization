use crate::sealed::Sealed;

/// [`VZGraphicsDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzgraphicsdeviceconfiguration)
pub trait GraphicsDeviceConfiguration: Sealed {}

/// [`VZKeyboardConfiguration`](https://developer.apple.com/documentation/virtualization/vzkeyboardconfiguration)
pub trait KeyboardConfiguration: Sealed {}

/// [`VZPointingDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzpointingdeviceconfiguration)
pub trait PointingDeviceConfiguration: Sealed {}

/// [`VZAudioDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzaudiodeviceconfiguration)
pub trait AudioDeviceConfiguration: Sealed {}
