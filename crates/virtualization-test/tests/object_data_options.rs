use virtualization::io_usb_host::ObjectDataOptions;

#[test]
fn has_iousbhosts_raw_values() {
  assert_eq!(ObjectDataOptions::kernel_user_shared().unwrap().raw_value, 1);
}
