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

pub(crate) struct NSError {
  never: Never,
}

/// `crate::foundation` reads an `NSError` through objc2 on macOS.
impl From<&NSError> for crate::NSError {
  fn from(error: &NSError) -> Self {
    match error.never {}
  }
}
