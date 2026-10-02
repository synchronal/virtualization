use super::{GUARDED, Never};

pub(crate) type DiskImageKitNil = Never;
pub(crate) type DiskImageKitError = Never;

/// `crate::error` reads what DiskImageKit threw on macOS.
impl From<Never> for crate::Error {
  fn from(never: Never) -> Self {
    match never {}
  }
}

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

pub(crate) fn ns_uuid_uuid_string(uuid: &NSUUID) -> String {
  match uuid.never {}
}
