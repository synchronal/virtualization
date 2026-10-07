//! Closes the traits that stand for Virtualization's abstract classes: as in
//! Virtualization, only its own subclasses are one. Its half crosses the
//! bridge, where an abstract class is an `NSObject`.

use crate::platform::ffi;

pub trait Sealed {
  /// The object, as its abstract class.
  fn bridge_object(&self) -> &ffi::NSObject;

  /// `object as? Self`.
  fn bridge_cast(object: &ffi::NSObject) -> Option<Self>
  where
    Self: Sized;
}
