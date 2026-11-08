//! With no USB accessory to open, only failing to open a device is tested.

use io_usb_host::{
  Device, DeviceMatchingDictionaryArguments, Error, ObjectInitOptions, PropertyListValue, error_domain,
};
use virtualization_test::apple_device_matching_dictionary;

#[test]
fn has_iousbhosts_error_domain() {
  assert_eq!(error_domain().unwrap(), "IOUSBHostErrorDomain");
}

/// `0` is `IO_OBJECT_NULL`.
#[test]
fn fails_as_iousbhost_does_to_open_no_service() {
  for interest_handler in [None, Some(Box::new(|_: &Device, _, _| {}) as _)] {
    let error = match Device::new(0, ObjectInitOptions::new(0), None, interest_handler) {
      Err(Error::NSError(error)) => error,
      other => panic!("expected IOUSBHost's error, got {other:?}"),
    };

    assert_eq!(error.domain, "IOUSBHostErrorDomain");
    assert_eq!(error.localized_description, "Failed to create IOUSBHostObject.");
    assert_eq!(error.localized_failure_reason.as_deref(), Some("IOServiceOpen failed."));
  }
}

#[test]
fn makes_a_matching_dictionary_of_its_arguments() {
  assert_eq!(
    apple_device_matching_dictionary(),
    [
      (
        "IOProviderClass",
        PropertyListValue::String("IOUSBHostDevice".to_string())
      ),
      ("idProduct", PropertyListValue::Integer(0x1234)),
      ("idVendor", PropertyListValue::Integer(0x05ac)),
    ]
    .map(|(key, value)| (key.to_string(), value))
    .into()
  );
}

#[test]
fn makes_a_matching_dictionary_of_a_product_id_array() {
  let dictionary = Device::create_matching_dictionary(&DeviceMatchingDictionaryArguments {
    vendor_id: Some(0x05ac),
    product_id: None,
    bcd_device: None,
    device_class: None,
    device_subclass: None,
    device_protocol: None,
    speed: None,
    product_id_array: Some(vec![1, 2]),
  })
  .unwrap();

  assert_eq!(
    dictionary["idProductArray"],
    PropertyListValue::Array(vec![PropertyListValue::Integer(1), PropertyListValue::Integer(2)])
  );
}
