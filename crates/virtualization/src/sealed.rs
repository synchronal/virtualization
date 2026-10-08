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

/// Each element as its abstract class.
pub(crate) fn bridge_objects<'a, T: Sealed + ?Sized>(elements: &[&'a T]) -> Vec<&'a ffi::NSObject> {
  elements
    .iter()
    .map(|element| element.bridge_object())
    .collect()
}

/// Each element `as? T`.
pub(crate) fn bridge_cast_each<T: Sealed>(elements: Vec<ffi::Retained<ffi::NSObject>>) -> Vec<Option<T>> {
  elements
    .iter()
    .map(|element| T::bridge_cast(element))
    .collect()
}
