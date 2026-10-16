use std::sync::Arc;

use crate::USBAccessory;

/// [`AAUSBAccessoryListener`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorylistener)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing.
/// AccessoryAccess calls them on an arbitrary thread.
pub trait USBAccessoryListener: Send + Sync {
  /// [`usbAccessoryDidConnect(_:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorylistener/usbaccessorydidconnect(_:))
  fn usb_accessory_did_connect(&self, _usb_accessory: &USBAccessory) {}

  /// [`usbAccessoryDidDisconnect(_:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorylistener/usbaccessorydiddisconnect(_:))
  fn usb_accessory_did_disconnect(&self, _usb_accessory: &USBAccessory) {}
}

/// `listener` as the bridge holds it.
#[cfg(target_os = "macos")]
pub(crate) fn listening(
  listener: &Arc<dyn USBAccessoryListener>,
) -> Arc<dyn accessory_access_sys::AAUSBAccessoryListener> {
  Arc::new(Listening(Arc::clone(listener)))
}

/// Off macOS, `platform::unsupported` takes the listener as it is.
#[cfg(not(target_os = "macos"))]
pub(crate) fn listening(listener: &Arc<dyn USBAccessoryListener>) -> Arc<dyn USBAccessoryListener> {
  Arc::clone(listener)
}

/// A listener as the bridge holds it.
#[cfg(target_os = "macos")]
struct Listening(Arc<dyn USBAccessoryListener>);

/// Called from Swift.
#[cfg(target_os = "macos")]
impl accessory_access_sys::AAUSBAccessoryListener for Listening {
  fn usb_accessory_did_connect(&self, usb_accessory: accessory_access_sys::AAUSBAccessory) {
    self
      .0
      .usb_accessory_did_connect(&USBAccessory { inner: usb_accessory });
  }

  fn usb_accessory_did_disconnect(&self, usb_accessory: accessory_access_sys::AAUSBAccessory) {
    self
      .0
      .usb_accessory_did_disconnect(&USBAccessory { inner: usb_accessory });
  }
}
