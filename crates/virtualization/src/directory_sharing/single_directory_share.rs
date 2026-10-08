use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{DirectoryShare, Error, SharedDirectory};

/// [`VZSingleDirectoryShare`](https://developer.apple.com/documentation/virtualization/vzsingledirectoryshare)
pub struct SingleDirectoryShare {
  inner: ffi::VZSingleDirectoryShare,
}

impl SingleDirectoryShare {
  /// [`init(directory:)`](https://developer.apple.com/documentation/virtualization/vzsingledirectoryshare/init(directory:))
  pub fn new(directory: &SharedDirectory) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_single_directory_share_init(&directory.inner),
    })
  }

  /// [`directory`](https://developer.apple.com/documentation/virtualization/vzsingledirectoryshare/directory)
  pub fn directory(&self) -> SharedDirectory {
    SharedDirectory::from_inner(self.inner.vz_directory())
  }
}

impl Clone for SingleDirectoryShare {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for SingleDirectoryShare {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for SingleDirectoryShare {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_single_directory_share_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl DirectoryShare for SingleDirectoryShare {}
