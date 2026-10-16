use std::fmt;

use dispatch2::DispatchQueue;

use crate::platform::ffi;
use crate::{Error, foundation};

/// [`AAUSBAccessory`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory)
pub struct USBAccessory {
  pub(crate) inner: ffi::AAUSBAccessory,
}

impl USBAccessory {
  /// [`registryID`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory/registryid)
  pub fn registry_id(&self) -> u64 {
    self.inner.aa_registry_id()
  }

  /// [`deviceDescriptorData`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory/devicedescriptordata)
  pub fn device_descriptor_data(&self) -> Vec<u8> {
    self.inner.aa_device_descriptor_data()
  }

  /// [`configurationDescriptorData`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory/configurationdescriptordata)
  pub fn configuration_descriptor_data(&self) -> Option<Vec<u8>> {
    self.inner.aa_configuration_descriptor_data()
  }

  /// [`open(serviceQueue:completionHandler:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory/open(servicequeue:completionhandler:)).
  /// `service_queue` `None` is `nil`.
  pub fn open(
    &self,
    service_queue: Option<&DispatchQueue>,
    completion_handler: impl FnOnce(Result<io_usb_host::Device, Error>) + Send + 'static,
  ) {
    self.inner.aa_open(service_queue, move |result| {
      completion_handler(
        result
          .map(io_usb_host::bridge::device_from)
          .map_err(|error| foundation::error(&error)),
      );
    });
  }

  /// [`close(completionHandler:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessory/close(completionhandler:))
  pub fn close(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .aa_close(move |error| completion_handler(foundation::completed(error)));
  }
}

impl Clone for USBAccessory {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBAccessory {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
