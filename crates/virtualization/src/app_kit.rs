//! AppKit's `NSScreen`, which `MacGraphicsDisplayConfiguration` is made for,
//! and the main thread its members need.

use std::fmt;

use crate::Error;
use crate::platform::{self, ffi};

/// Proof of being on the main thread, where Swift's `@MainActor` members
/// run: AppKit's take it, as objc2's do.
#[derive(Clone, Copy)]
pub struct MainThreadMarker {
  pub(crate) inner: ffi::MainThreadMarker,
}

impl MainThreadMarker {
  /// `None` off the main thread.
  pub fn new() -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(ffi::MainThreadMarker::new().map(|inner| Self { inner }))
  }
}

impl fmt::Debug for MainThreadMarker {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("MainThreadMarker")
  }
}

/// [`NSScreen`](https://developer.apple.com/documentation/appkit/nsscreen)
///
/// Only what makes one: AppKit's own members aren't Virtualization's.
pub struct NSScreen {
  pub(crate) inner: ffi::NSScreen,
}

impl NSScreen {
  /// [`screens`](https://developer.apple.com/documentation/appkit/nsscreen/screens)
  pub fn screens(mtm: MainThreadMarker) -> Vec<Self> {
    ffi::ns_screen_screens(mtm.inner)
      .into_iter()
      .map(|inner| Self { inner })
      .collect()
  }

  /// [`main`](https://developer.apple.com/documentation/appkit/nsscreen/main)
  pub fn main(mtm: MainThreadMarker) -> Option<Self> {
    ffi::ns_screen_main(mtm.inner).map(|inner| Self { inner })
  }
}

impl Clone for NSScreen {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NSScreen {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MainThreadMarker::new(), Err(Error::Unsupported)));
  }

  /// libtest runs each test on a thread of its own.
  #[cfg(target_os = "macos")]
  #[test]
  fn is_none_off_the_main_thread() {
    assert!(MainThreadMarker::new().unwrap().is_none());
  }
}
