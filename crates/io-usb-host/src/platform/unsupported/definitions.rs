use super::GUARDED;

pub(crate) enum IOUSBHostAbortOption {
  Asynchronous,
  Synchronous,
  Unknown(usize),
}

pub(crate) enum IOUSBDescriptorType {
  Device,
  Configuration,
  String,
  Interface,
  Endpoint,
  DeviceQualifier,
  OtherSpeedConfiguration,
  InterfacePower,
  Otg,
  Debug,
  InterfaceAssociation,
  Bos,
  DeviceCapability,
  Hid,
  Report,
  Physical,
  Hub,
  SuperSpeedHub,
  SuperSpeedUSBEndpointCompanion,
  SuperSpeedPlusIsochronousEndpointCompanion,
  Unknown(u32),
}

pub(crate) enum IOUSBDeviceRequestDirectionValue {
  Out,
  In,
  Unknown(u32),
}

pub(crate) enum IOUSBDeviceRequestTypeValue {
  Standard,
  Class,
  Vendor,
  Unknown(u32),
}

pub(crate) enum IOUSBDeviceRequestRecipientValue {
  Device,
  Interface,
  Endpoint,
  Other,
  Unknown(u32),
}

pub(crate) fn iousbhost_object_init_options_device_capture() -> usize {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_object_init_options_device_seize() -> usize {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_object_destroy_options_device_surrender() -> usize {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_object_data_options_kernel_user_shared() -> usize {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_request_type(
  _direction: IOUSBDeviceRequestDirectionValue,
  _request_type: IOUSBDeviceRequestTypeValue,
  _recipient: IOUSBDeviceRequestRecipientValue,
) -> u8 {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_default_control_completion_timeout() -> f64 {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_error_domain() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_property_key_location_id() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_vendor_id() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_product_id() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_product_id_mask() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_product_id_array() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_interface_number() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_configuration_value() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_device_release_number() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_interface_class() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_interface_sub_class() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_interface_protocol() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_device_class() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_device_sub_class() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_device_protocol() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_matching_property_key_speed() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_property_key_vendor_string() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_property_key_serial_number_string() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_property_key_container_id() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_property_key_current_configuration() -> String {
  unreachable!("{GUARDED}")
}
