//! IOUSBHost's `IOUSBHostDevicePropertyKey`, and
//! `IOUSBHostPropertyKeyLocationID`.

use virtualization::io_usb_host::{DevicePropertyKey, property_key_location_id};

#[test]
fn has_iousbhosts_raw_values() {
  for (key, raw_value) in [
    (DevicePropertyKey::vendor_string(), "kUSBVendorString"),
    (DevicePropertyKey::serial_number_string(), "kUSBSerialNumberString"),
    (DevicePropertyKey::container_id(), "kUSBContainerID"),
    (DevicePropertyKey::current_configuration(), "kUSBCurrentConfiguration"),
  ] {
    assert_eq!(key.unwrap().raw_value, raw_value);
  }
}

#[test]
fn has_iousbhosts_location_id_key() {
  assert_eq!(property_key_location_id().unwrap(), "locationID");
}
