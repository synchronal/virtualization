use std::fmt;

use url::Url;

use crate::foundation;
use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{BootLoader, Error};

/// [`VZLinuxBootLoader`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader)
pub struct LinuxBootLoader {
  inner: ffi::VZLinuxBootLoader,
}

impl LinuxBootLoader {
  /// [`init(kernelURL:)`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/init(kernelurl:))
  pub fn new(kernel_url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_linux_boot_loader_init(&*foundation::ns_url(kernel_url)?),
    })
  }

  /// [`kernelURL`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/kernelurl)
  pub fn kernel_url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_kernel_url())
  }

  /// [`kernelURL`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/kernelurl)
  pub fn set_kernel_url(&self, kernel_url: &Url) -> Result<(), Error> {
    self
      .inner
      .vz_set_kernel_url(&*foundation::ns_url(kernel_url)?);
    Ok(())
  }

  /// [`commandLine`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/commandline)
  pub fn command_line(&self) -> String {
    self.inner.vz_command_line()
  }

  /// [`commandLine`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/commandline)
  pub fn set_command_line(&self, command_line: &str) {
    self.inner.vz_set_command_line(command_line);
  }

  /// [`initialRamdiskURL`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/initialramdiskurl)
  pub fn initial_ramdisk_url(&self) -> Result<Option<Url>, Error> {
    self
      .inner
      .vz_initial_ramdisk_url()
      .map(|url| foundation::url(&url))
      .transpose()
  }

  /// [`initialRamdiskURL`](https://developer.apple.com/documentation/virtualization/vzlinuxbootloader/initialramdiskurl)
  pub fn set_initial_ramdisk_url(&self, initial_ramdisk_url: Option<&Url>) -> Result<(), Error> {
    let initial_ramdisk_url = initial_ramdisk_url.map(foundation::ns_url).transpose()?;

    self
      .inner
      .vz_set_initial_ramdisk_url(initial_ramdisk_url.as_deref());
    Ok(())
  }
}

impl Clone for LinuxBootLoader {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for LinuxBootLoader {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for LinuxBootLoader {}

impl BootLoader for LinuxBootLoader {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let kernel_url = Url::parse("file:///nowhere/vmlinuz").unwrap();

    assert!(matches!(LinuxBootLoader::new(&kernel_url), Err(Error::Unsupported)));
  }
}
