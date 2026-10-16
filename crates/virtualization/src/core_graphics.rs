//! Core Graphics' `CGSize`, which Virtualization's displays take and return.

use crate::platform::ffi;

/// [`CGSize`](https://developer.apple.com/documentation/corefoundation/cgsize)
///
/// `CGFloat` is an `f64`, as it is on every Mac.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGSize {
  pub width: f64,
  pub height: f64,
}

impl From<CGSize> for ffi::BridgedCGSize {
  fn from(size: CGSize) -> Self {
    Self {
      width: size.width,
      height: size.height,
    }
  }
}

impl From<ffi::BridgedCGSize> for CGSize {
  fn from(size: ffi::BridgedCGSize) -> Self {
    Self {
      width: size.width,
      height: size.height,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn crosses_virtualization_sys_and_back_unchanged() {
    let size = CGSize {
      width: 1920.0,
      height: 1080.5,
    };

    assert_eq!(CGSize::from(ffi::BridgedCGSize::from(size)), size);
  }
}
