use super::{GUARDED, Never};

pub(crate) type BridgedNil = Never;

pub(crate) type BridgedThrown = Never;

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

/// `pub`, not `pub(crate)`: `Sealed`'s methods name it, and are reachable
/// wherever the traits it seals are.
pub struct NSObject {
  never: Never,
}

impl NSObject {
  pub(crate) fn never(&self) -> ! {
    match self.never {}
  }
}

pub(crate) struct NSURL {
  never: Never,
}

pub(crate) fn ns_url_init_string(_string: &str) -> Option<Retained<NSURL>> {
  unreachable!("{GUARDED}")
}

pub(crate) fn ns_url_absolute_string(url: &NSURL) -> String {
  match url.never {}
}

pub(crate) struct NSUUID {
  never: Never,
}

pub(crate) fn ns_uuid_init_uuid_string(_string: &str) -> Option<Retained<NSUUID>> {
  unreachable!("{GUARDED}")
}

pub(crate) fn ns_uuid_uuid_string(uuid: &NSUUID) -> String {
  match uuid.never {}
}

pub(crate) struct NSError {
  never: Never,
}

pub(crate) fn bridge_take_thrown_error() -> Retained<NSError> {
  unreachable!("{GUARDED}")
}

/// `crate::error` reads an `NSError` through objc2 on macOS.
impl From<&NSError> for crate::NSError {
  fn from(error: &NSError) -> Self {
    match error.never {}
  }
}
