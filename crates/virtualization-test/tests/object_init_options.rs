//! IOUSBHost's `IOUSBHostObjectInitOptions`.

use virtualization::io_usb_host::ObjectInitOptions;

#[test]
fn has_iousbhosts_raw_values() {
  assert_eq!(ObjectInitOptions::device_capture().unwrap().raw_value, 1);
  assert_eq!(ObjectInitOptions::device_seize().unwrap().raw_value, 2);
}
