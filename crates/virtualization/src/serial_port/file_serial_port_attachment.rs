use std::fmt;

use url::Url;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, SerialPortAttachment, foundation};

/// [`VZFileSerialPortAttachment`](https://developer.apple.com/documentation/virtualization/vzfileserialportattachment)
pub struct FileSerialPortAttachment {
  inner: ffi::VZFileSerialPortAttachment,
}

impl FileSerialPortAttachment {
  /// [`init(url:append:)`](https://developer.apple.com/documentation/virtualization/vzfileserialportattachment/init(url:append:))
  pub fn new(url: &Url, append: bool) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_file_serial_port_attachment_init(&*foundation::ns_url(url)?, append)
        .map_err(foundation::thrown)?,
    })
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vzfileserialportattachment/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`append`](https://developer.apple.com/documentation/virtualization/vzfileserialportattachment/append)
  pub fn append(&self) -> bool {
    self.inner.vz_append()
  }
}

impl Clone for FileSerialPortAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for FileSerialPortAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for FileSerialPortAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_file_serial_port_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl SerialPortAttachment for FileSerialPortAttachment {}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      FileSerialPortAttachment::new(&Url::parse("file:///console.log").unwrap(), false),
      Err(Error::Unsupported)
    ));
  }
}
