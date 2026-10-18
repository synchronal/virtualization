use std::ffi::c_void;
use std::fmt;

use url::Url;

use crate::platform::{self, ffi};
use crate::{Error, VirtualMachine, foundation};

/// [`VZMacOSInstaller`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller)
///
/// `Send` and `Sync`, as [`VirtualMachine`] is: Virtualization takes every
/// call on it on its machine's queue, which is the caller's to keep to, as in
/// Swift. `completion_handler` is called on that queue.
pub struct MacOSInstaller {
  inner: ffi::VZMacOSInstaller,
}

impl MacOSInstaller {
  /// [`init(virtualMachine:restoringFromImageAt:)`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller/init(virtualmachine:restoringfromimageat:))
  pub fn new(virtual_machine: &VirtualMachine, restore_image_file_url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_macos_installer_init_restoring_from_image_at(
        &virtual_machine.inner,
        &*foundation::ns_url(restore_image_file_url)?,
      ),
    })
  }

  /// [`install(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller/install(completionhandler:))
  pub fn install(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .vz_install(move |error| completion_handler(foundation::completed(error)));
  }

  /// [`progress`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller/progress):
  /// its `NSProgress *`, for Foundation through objc2-foundation, valid as
  /// long as this installer, which holds it.
  pub fn progress(&self) -> *mut c_void {
    self.inner.vz_progress().cast_mut()
  }

  /// [`virtualMachine`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller/virtualmachine)
  pub fn virtual_machine(&self) -> VirtualMachine {
    VirtualMachine {
      inner: self.inner.vz_virtual_machine(),
    }
  }

  /// [`restoreImageURL`](https://developer.apple.com/documentation/virtualization/vzmacosinstaller/restoreimageurl)
  pub fn restore_image_url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_restore_image_url())
  }
}

impl Clone for MacOSInstaller {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacOSInstaller {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
