use io_usb_host::MatchingPropertyKey;

#[test]
fn has_iousbhosts_raw_values() {
  for (key, raw_value) in [
    (MatchingPropertyKey::vendor_id(), "idVendor"),
    (MatchingPropertyKey::product_id(), "idProduct"),
    (MatchingPropertyKey::product_id_mask(), "idProductMask"),
    (MatchingPropertyKey::product_id_array(), "idProductArray"),
    (MatchingPropertyKey::interface_number(), "bInterfaceNumber"),
    (MatchingPropertyKey::configuration_value(), "bConfigurationValue"),
    (MatchingPropertyKey::device_release_number(), "bcdDevice"),
    (MatchingPropertyKey::interface_class(), "bInterfaceClass"),
    (MatchingPropertyKey::interface_sub_class(), "bInterfaceSubClass"),
    (MatchingPropertyKey::interface_protocol(), "bInterfaceProtocol"),
    (MatchingPropertyKey::device_class(), "bDeviceClass"),
    (MatchingPropertyKey::device_sub_class(), "bDeviceSubClass"),
    (MatchingPropertyKey::device_protocol(), "bDeviceProtocol"),
    (MatchingPropertyKey::speed(), "USBSpeed"),
  ] {
    assert_eq!(key.unwrap().raw_value, raw_value);
  }
}
