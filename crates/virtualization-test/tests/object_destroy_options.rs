//! IOUSBHost's `IOUSBHostObjectDestroyOptions`.

use virtualization::io_usb_host::ObjectDestroyOptions;

#[test]
fn has_iousbhosts_raw_values() {
  assert_eq!(ObjectDestroyOptions::device_surrender().unwrap().raw_value, 1);
}
