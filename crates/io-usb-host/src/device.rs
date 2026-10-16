use std::collections::BTreeMap;
use std::fmt;

use dispatch2::DispatchQueue;

use crate::foundation;
use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{Error, Object, ObjectInitOptions, PropertyListValue};

/// [`IOUSBHostDevice`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice)
pub struct Device {
  pub(crate) inner: ffi::IOUSBHostDevice,
}

impl Device {
  /// [`initWithIOService:options:queue:error:interestHandler:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/initwithioservice:options:queue:error:interesthandler:),
  /// `IOUSBHostObject`'s. `io_service` is an `io_service_t`; `queue` and
  /// `interest_handler` `None` are `nil`.
  pub fn new(
    io_service: u32,
    options: ObjectInitOptions,
    queue: Option<&DispatchQueue>,
    interest_handler: Option<InterestHandler>,
  ) -> Result<Self, Error> {
    platform::supported()?;

    let interest_handler = interest_handler.map(|handler| {
      Box::new(move |inner, message_type, message_argument| {
        handler(&Device { inner }, message_type, message_argument);
      }) as ffi::IOUSBHostInterestHandler
    });

    Ok(Self {
      inner: ffi::iousbhost_device_init_io_service_options_queue_interest_handler(
        io_service,
        options.raw_value,
        queue,
        interest_handler,
      )
      .map_err(foundation::thrown)?,
    })
  }

  /// [`createMatchingDictionaryWithVendorID:productID:bcdDevice:deviceClass:deviceSubclass:deviceProtocol:speed:productIDArray:`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice/creatematchingdictionarywithvendorid:productid:bcddevice:deviceclass:devicesubclass:deviceprotocol:speed:productidarray:):
  /// the dictionary's entries.
  pub fn create_matching_dictionary(
    arguments: &DeviceMatchingDictionaryArguments,
  ) -> Result<BTreeMap<String, PropertyListValue>, Error> {
    platform::supported()?;

    Ok(ffi::iousbhost_device_create_matching_dictionary(&bridged(arguments)))
  }

  /// [`configurationDescriptor`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice/configurationdescriptor):
  /// its `wTotalLength` bytes, its interface and endpoint descriptors with
  /// it.
  pub fn configuration_descriptor(&self) -> Option<Vec<u8>> {
    self.inner.iousbhost_configuration_descriptor()
  }

  /// [`configureWithValue:matchInterfaces:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice/configurewithvalue:matchinterfaces:error:)
  pub fn configure_with_value(&self, value: isize, match_interfaces: bool) -> Result<(), Error> {
    self
      .inner
      .iousbhost_configure_with_value(value, match_interfaces)
      .map_err(foundation::thrown)
  }

  /// [`reset()`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice/reset())
  pub fn reset(&self) -> Result<(), Error> {
    self.inner.iousbhost_reset().map_err(foundation::thrown)
  }
}

impl Clone for Device {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for Device {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for Device {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }
}

impl Object for Device {}

/// [`IOUSBHostInterestHandler`](https://developer.apple.com/documentation/iousbhost/iousbhostinteresthandler):
/// called on IOUSBHost's internal queue with the device, an IOKit message
/// type, and its argument: the address of its data, or the data itself if
/// it fits.
pub type InterestHandler = Box<dyn Fn(&Device, u32, usize) + Send + Sync + 'static>;

/// [`createMatchingDictionaryWithVendorID:…`](https://developer.apple.com/documentation/iousbhost/iousbhostdevice/creatematchingdictionarywithvendorid:productid:bcddevice:deviceclass:devicesubclass:deviceprotocol:speed:productidarray:)'s
/// arguments, each `None` for `nil`: more than clippy lets a function take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceMatchingDictionaryArguments {
  pub vendor_id: Option<i64>,
  pub product_id: Option<i64>,
  pub bcd_device: Option<i64>,
  pub device_class: Option<i64>,
  pub device_subclass: Option<i64>,
  pub device_protocol: Option<i64>,
  pub speed: Option<i64>,
  pub product_id_array: Option<Vec<i64>>,
}

/// The arguments as the sys crate takes them.
#[cfg(target_os = "macos")]
fn bridged(arguments: &DeviceMatchingDictionaryArguments) -> ffi::IOUSBHostDeviceMatchingDictionaryArguments<'_> {
  ffi::IOUSBHostDeviceMatchingDictionaryArguments {
    vendor_id: arguments.vendor_id,
    product_id: arguments.product_id,
    bcd_device: arguments.bcd_device,
    device_class: arguments.device_class,
    device_subclass: arguments.device_subclass,
    device_protocol: arguments.device_protocol,
    speed: arguments.speed,
    product_id_array: arguments.product_id_array.as_deref(),
  }
}

/// Off macOS, `platform::unsupported` takes the arguments as they are.
#[cfg(not(target_os = "macos"))]
fn bridged(arguments: &DeviceMatchingDictionaryArguments) -> DeviceMatchingDictionaryArguments {
  arguments.clone()
}

#[cfg(test)]
mod tests {
  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    use super::*;

    assert!(matches!(
      Device::new(0, ObjectInitOptions::new(0), None, None),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      Device::create_matching_dictionary(&DeviceMatchingDictionaryArguments {
        vendor_id: Some(0x05ac),
        product_id: None,
        bcd_device: None,
        device_class: None,
        device_subclass: None,
        device_protocol: None,
        speed: None,
        product_id_array: None,
      }),
      Err(Error::Unsupported)
    ));
  }
}
