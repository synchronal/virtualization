use std::fmt;

use url::Url;

use crate::Error;
use crate::foundation;
use crate::platform::{self, ffi};

/// [`VZSharedDirectory`](https://developer.apple.com/documentation/virtualization/vzshareddirectory)
pub struct SharedDirectory {
  pub(crate) inner: ffi::VZSharedDirectory,
}

impl SharedDirectory {
  /// [`init(url:readOnly:)`](https://developer.apple.com/documentation/virtualization/vzshareddirectory/init(url:readonly:))
  pub fn new(url: &Url, read_only: bool) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_shared_directory_init(&*foundation::ns_url(url)?, read_only),
    })
  }

  pub(crate) fn from_inner(inner: ffi::VZSharedDirectory) -> Self {
    Self { inner }
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzshareddirectory/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`isReadOnly`](https://developer.apple.com/documentation/virtualization/vzshareddirectory/isreadonly)
  pub fn is_read_only(&self) -> bool {
    self.inner.vz_is_read_only()
  }
}

impl Clone for SharedDirectory {
  fn clone(&self) -> Self {
    Self::from_inner(self.inner.bridge_retained())
  }
}

/// The object's `description`.
impl fmt::Debug for SharedDirectory {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      SharedDirectory::new(&Url::parse("file:///tmp").unwrap(), false),
      Err(Error::Unsupported)
    ));
  }
}
