use std::collections::BTreeMap;
use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{DirectoryShare, Error, SharedDirectory, foundation};

/// [`VZMultipleDirectoryShare`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare)
///
/// `[String: VZSharedDirectory]` is a [`BTreeMap`].
pub struct MultipleDirectoryShare {
  inner: ffi::VZMultipleDirectoryShare,
}

impl MultipleDirectoryShare {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_multiple_directory_share_init(),
    })
  }

  /// [`init(directories:)`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare/init(directories:))
  pub fn with_directories(directories: &BTreeMap<String, SharedDirectory>) -> Result<Self, Error> {
    platform::supported()?;

    let directories: Vec<_> = directories
      .iter()
      .map(|(name, directory)| (name.as_str(), &directory.inner))
      .collect();

    Ok(Self {
      inner: ffi::vz_multiple_directory_share_init_directories(&directories),
    })
  }

  /// [`validateName(_:)`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare/validatename(_:))
  pub fn validate_name(name: &str) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_multiple_directory_share_validate_name(name).map_err(foundation::thrown)
  }

  /// [`canonicalizedName(from:)`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare/canonicalizedname(from:))
  pub fn canonicalized_name(name: &str) -> Result<Option<String>, Error> {
    platform::supported()?;

    Ok(ffi::vz_multiple_directory_share_canonicalized_name(name))
  }

  /// [`directories`](https://developer.apple.com/documentation/virtualization/vzmultipledirectoryshare/directories)
  pub fn directories(&self) -> BTreeMap<String, SharedDirectory> {
    self
      .inner
      .vz_directories()
      .into_iter()
      .map(|(name, directory)| (name, SharedDirectory::from_inner(directory)))
      .collect()
  }
}

impl Clone for MultipleDirectoryShare {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MultipleDirectoryShare {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for MultipleDirectoryShare {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_multiple_directory_share_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl DirectoryShare for MultipleDirectoryShare {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(MultipleDirectoryShare::new(), Err(Error::Unsupported)));
    assert!(matches!(
      MultipleDirectoryShare::with_directories(&BTreeMap::new()),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      MultipleDirectoryShare::validate_name("name"),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      MultipleDirectoryShare::canonicalized_name("name"),
      Err(Error::Unsupported)
    ));
  }
}
