use std::fmt;

use url::Url;

use crate::platform::{self, ffi};
use crate::{Error, MacOSConfigurationRequirements, OperatingSystemVersion, foundation};

/// [`VZMacOSRestoreImage`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage)
pub struct MacOSRestoreImage {
  inner: ffi::VZMacOSRestoreImage,
}

impl MacOSRestoreImage {
  /// [`load(from:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/load(from:completionhandler:)),
  /// whose `completion_handler` is called on an arbitrary thread. Fails,
  /// without calling `completion_handler`, if Foundation can't represent
  /// `file_url`.
  pub fn load(
    file_url: &Url,
    completion_handler: impl FnOnce(Result<Self, Error>) + Send + 'static,
  ) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_macos_restore_image_load(&*foundation::ns_url(file_url)?, move |result| {
      completion_handler(restore_image(result));
    });

    Ok(())
  }

  /// [`fetchLatestSupported(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/fetchlatestsupported(completionhandler:)),
  /// whose `completion_handler` is called on an arbitrary thread.
  pub fn fetch_latest_supported(
    completion_handler: impl FnOnce(Result<Self, Error>) + Send + 'static,
  ) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_macos_restore_image_fetch_latest_supported(move |result| completion_handler(restore_image(result)));

    Ok(())
  }

  /// [`isSupported`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/issupported)
  pub fn is_supported(&self) -> bool {
    self.inner.vz_is_supported()
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`buildVersion`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/buildversion)
  pub fn build_version(&self) -> String {
    self.inner.vz_build_version()
  }

  /// [`operatingSystemVersion`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/operatingsystemversion)
  pub fn operating_system_version(&self) -> OperatingSystemVersion {
    self.inner.vz_operating_system_version().into()
  }

  /// [`mostFeaturefulSupportedConfiguration`](https://developer.apple.com/documentation/virtualization/vzmacosrestoreimage/mostfeaturefulsupportedconfiguration)
  pub fn most_featureful_supported_configuration(&self) -> Option<MacOSConfigurationRequirements> {
    self
      .inner
      .vz_most_featureful_supported_configuration()
      .ok()
      .map(|inner| MacOSConfigurationRequirements { inner })
  }
}

/// What a completion handler was called with.
fn restore_image(
  result: Result<ffi::VZMacOSRestoreImage, ffi::Retained<ffi::NSError>>,
) -> Result<MacOSRestoreImage, Error> {
  match result {
    Ok(inner) => Ok(MacOSRestoreImage { inner }),
    Err(error) => Err(foundation::error(&error)),
  }
}

impl Clone for MacOSRestoreImage {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for MacOSRestoreImage {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/restore-image.ipsw").unwrap();

    assert!(matches!(
      MacOSRestoreImage::load(&url, |_| unreachable!()),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      MacOSRestoreImage::fetch_latest_supported(|_| unreachable!()),
      Err(Error::Unsupported)
    ));
  }
}
