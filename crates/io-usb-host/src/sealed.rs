//! Closes the trait that stands for `IOUSBHostObject`: as in IOUSBHost, only
//! its own subclasses are one. Its half crosses the bridge, where the base
//! class is an `NSObject`.

use crate::platform::ffi;

pub trait Sealed {
  /// The object, as its base class.
  fn bridge_object(&self) -> &ffi::NSObject;
}
