mod host_audio_input_stream_source;
mod host_audio_output_stream_sink;
mod mac_graphics_device;
mod mac_graphics_device_configuration;
mod mac_graphics_display;
mod mac_graphics_display_configuration;
mod mac_keyboard_configuration;
mod mac_trackpad_configuration;
mod usb_keyboard_configuration;
mod usb_screen_coordinate_pointing_device_configuration;
mod virtio_graphics_device;
mod virtio_graphics_device_configuration;
mod virtio_graphics_scanout;
mod virtio_graphics_scanout_configuration;
mod virtio_sound_device_configuration;
mod virtio_sound_device_input_stream_configuration;
mod virtio_sound_device_output_stream_configuration;

use std::sync::Arc;
#[cfg(target_os = "macos")]
use std::sync::Weak;

pub use self::host_audio_input_stream_source::HostAudioInputStreamSource;
pub use self::host_audio_output_stream_sink::HostAudioOutputStreamSink;
pub use self::mac_graphics_device::MacGraphicsDevice;
pub use self::mac_graphics_device_configuration::MacGraphicsDeviceConfiguration;
pub use self::mac_graphics_display::MacGraphicsDisplay;
pub use self::mac_graphics_display_configuration::MacGraphicsDisplayConfiguration;
pub use self::mac_keyboard_configuration::MacKeyboardConfiguration;
pub use self::mac_trackpad_configuration::MacTrackpadConfiguration;
pub use self::usb_keyboard_configuration::USBKeyboardConfiguration;
pub use self::usb_screen_coordinate_pointing_device_configuration::USBScreenCoordinatePointingDeviceConfiguration;
pub use self::virtio_graphics_device::VirtioGraphicsDevice;
pub use self::virtio_graphics_device_configuration::VirtioGraphicsDeviceConfiguration;
pub use self::virtio_graphics_scanout::VirtioGraphicsScanout;
pub use self::virtio_graphics_scanout_configuration::VirtioGraphicsScanoutConfiguration;
pub use self::virtio_sound_device_configuration::VirtioSoundDeviceConfiguration;
pub use self::virtio_sound_device_input_stream_configuration::VirtioSoundDeviceInputStreamConfiguration;
pub use self::virtio_sound_device_output_stream_configuration::VirtioSoundDeviceOutputStreamConfiguration;
use crate::platform::ffi;
use crate::sealed::{Sealed, bridge_cast_each};
use crate::{CGSize, Error, foundation};

/// [`VZGraphicsDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzgraphicsdeviceconfiguration)
pub trait GraphicsDeviceConfiguration: Sealed {}

/// [`VZGraphicsDisplayConfiguration`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplayconfiguration)
pub trait GraphicsDisplayConfiguration: Sealed {}

/// [`VZKeyboardConfiguration`](https://developer.apple.com/documentation/virtualization/vzkeyboardconfiguration)
pub trait KeyboardConfiguration: Sealed {}

/// [`VZPointingDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzpointingdeviceconfiguration)
pub trait PointingDeviceConfiguration: Sealed {}

/// [`VZAudioDeviceConfiguration`](https://developer.apple.com/documentation/virtualization/vzaudiodeviceconfiguration)
pub trait AudioDeviceConfiguration: Sealed {}

/// [`VZVirtioSoundDeviceStreamConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtiosounddevicestreamconfiguration)
pub trait VirtioSoundDeviceStreamConfiguration: Sealed {}

/// [`VZAudioInputStreamSource`](https://developer.apple.com/documentation/virtualization/vzaudioinputstreamsource)
pub trait AudioInputStreamSource: Sealed {}

/// [`VZAudioOutputStreamSink`](https://developer.apple.com/documentation/virtualization/vzaudiooutputstreamsink)
pub trait AudioOutputStreamSink: Sealed {}

/// [`VZGraphicsDevice`](https://developer.apple.com/documentation/virtualization/vzgraphicsdevice)
pub trait GraphicsDevice: Sealed {
  /// [`displays`](https://developer.apple.com/documentation/virtualization/vzgraphicsdevice/displays),
  /// each as Swift's `as? T`: `None` for one that isn't a `T`.
  fn displays<T: GraphicsDisplay>(&self) -> Vec<Option<T>>
  where
    Self: Sized,
  {
    bridge_cast_each(ffi::vz_graphics_device_displays(self.bridge_object()))
  }
}

/// [`VZGraphicsDisplay`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay)
pub trait GraphicsDisplay: Sealed {
  /// [`sizeInPixels`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay/sizeinpixels)
  fn size_in_pixels(&self) -> CGSize {
    ffi::vz_graphics_display_size_in_pixels(self.bridge_object()).into()
  }

  /// [`reconfigure(sizeInPixels:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay/reconfigure(sizeinpixels:))
  fn reconfigure_with_size_in_pixels(&self, size_in_pixels: CGSize) -> Result<(), Error> {
    ffi::vz_graphics_display_reconfigure_size_in_pixels(self.bridge_object(), size_in_pixels.into())
      .map_err(foundation::thrown)
  }

  /// [`reconfigure(configuration:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay/reconfigure(configuration:))
  fn reconfigure_with_configuration(&self, configuration: &dyn GraphicsDisplayConfiguration) -> Result<(), Error> {
    ffi::vz_graphics_display_reconfigure_configuration(self.bridge_object(), configuration.bridge_object())
      .map_err(foundation::thrown)
  }

  /// [`addObserver(_:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay/addobserver(_:)),
  /// held weakly, as in Swift.
  fn add_observer(&self, observer: &Arc<dyn GraphicsDisplayObserver>) {
    ffi::vz_graphics_display_add_observer(self.bridge_object(), observing(observer), observer_key(observer));
  }

  /// [`removeObserver(_:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplay/removeobserver(_:)):
  /// `observer` is the `Arc` it was added with.
  fn remove_observer(&self, observer: &Arc<dyn GraphicsDisplayObserver>) {
    // The bridge finds the observer by its key, and uses this one only for an
    // observer never added, which the display must still be given.
    ffi::vz_graphics_display_remove_observer(self.bridge_object(), observing(observer), observer_key(observer));
  }
}

/// [`VZGraphicsDisplayObserver`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplayobserver)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing.
/// Virtualization calls them on the machine's queue.
pub trait GraphicsDisplayObserver: Send + Sync {
  /// [`displayDidBeginReconfiguration(_:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplayobserver/displaydidbeginreconfiguration(_:)):
  /// `display` is the display as its abstract class, as Swift passes it.
  fn display_did_begin_reconfiguration(&self, _display: &dyn GraphicsDisplay) {}

  /// [`displayDidEndReconfiguration(_:)`](https://developer.apple.com/documentation/virtualization/vzgraphicsdisplayobserver/displaydidendreconfiguration(_:)):
  /// `display` is the display as its abstract class, as Swift passes it.
  fn display_did_end_reconfiguration(&self, _display: &dyn GraphicsDisplay) {}
}

/// What the bridge knows `observer` by: its object's address.
fn observer_key(observer: &Arc<dyn GraphicsDisplayObserver>) -> usize {
  Arc::as_ptr(observer).cast::<()>().addr()
}

/// `observer` as Virtualization holds it: weakly.
#[cfg(target_os = "macos")]
fn observing(observer: &Arc<dyn GraphicsDisplayObserver>) -> Arc<dyn ffi::VZGraphicsDisplayObserver> {
  Arc::new(Observing(Arc::downgrade(observer)))
}

/// Off macOS, `platform::unsupported` takes the observer as it is.
#[cfg(not(target_os = "macos"))]
fn observing(observer: &Arc<dyn GraphicsDisplayObserver>) -> Arc<dyn GraphicsDisplayObserver> {
  Arc::clone(observer)
}

/// An observer as Virtualization holds it: weakly.
#[cfg(target_os = "macos")]
struct Observing(Weak<dyn GraphicsDisplayObserver>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZGraphicsDisplayObserver for Observing {
  fn display_did_begin_reconfiguration(&self, display: ffi::Retained<ffi::NSObject>) {
    if let Some(observer) = self.0.upgrade() {
      observer.display_did_begin_reconfiguration(&AnyGraphicsDisplay { object: display });
    }
  }

  fn display_did_end_reconfiguration(&self, display: ffi::Retained<ffi::NSObject>) {
    if let Some(observer) = self.0.upgrade() {
      observer.display_did_end_reconfiguration(&AnyGraphicsDisplay { object: display });
    }
  }

  fn bridge_is_live(&self) -> bool {
    self.0.strong_count() > 0
  }
}

/// A `VZGraphicsDisplay` of whichever subclass, as an observer is told of it.
#[cfg(target_os = "macos")]
struct AnyGraphicsDisplay {
  object: ffi::Retained<ffi::NSObject>,
}

#[cfg(target_os = "macos")]
impl Sealed for AnyGraphicsDisplay {
  fn bridge_object(&self) -> &ffi::NSObject {
    &self.object
  }

  /// Never asked: no caller can name this type to cast to it.
  fn bridge_cast(_object: &ffi::NSObject) -> Option<Self> {
    None
  }
}

#[cfg(target_os = "macos")]
impl GraphicsDisplay for AnyGraphicsDisplay {}
