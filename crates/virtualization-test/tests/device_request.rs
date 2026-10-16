use virtualization::io_usb_host::{
  DeviceRequestDirectionValue, DeviceRequestRecipientValue, DeviceRequestTypeValue, default_control_completion_timeout,
  device_request_type,
};

#[test]
fn makes_a_bm_request_type() {
  assert_eq!(
    device_request_type(
      DeviceRequestDirectionValue::In,
      DeviceRequestTypeValue::Standard,
      DeviceRequestRecipientValue::Device
    )
    .unwrap(),
    0x80
  );
  assert_eq!(
    device_request_type(
      DeviceRequestDirectionValue::Out,
      DeviceRequestTypeValue::Vendor,
      DeviceRequestRecipientValue::Interface
    )
    .unwrap(),
    0x41
  );
}

#[test]
fn has_iousbhosts_default_completion_timeout() {
  assert_eq!(default_control_completion_timeout().unwrap(), 5.0);
}
