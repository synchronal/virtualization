use super::Never;

/// objc2's `Retained<T>`.
pub(crate) struct Retained<T> {
  object: T,
}

impl<T> std::ops::Deref for Retained<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.object
  }
}

/// `pub`, not `pub(crate)`: `Sealed`'s method names it, and is reachable
/// wherever `Object` is.
pub struct NSObject {
  never: Never,
}

impl NSObject {
  pub(crate) fn never(&self) -> ! {
    match self.never {}
  }
}

pub(crate) struct NSError {
  never: Never,
}

/// `crate::foundation` reads an `NSError` through objc2 on macOS.
impl From<&NSError> for crate::NSError {
  fn from(error: &NSError) -> Self {
    match error.never {}
  }
}
