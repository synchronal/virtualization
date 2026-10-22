use dispatch2::{DispatchQueue, DispatchRetained};

use super::{
  IOUSBDescriptorType, IOUSBDeviceRequestRecipientValue, IOUSBDeviceRequestTypeValue, IOUSBHostAbortOption, NSError,
  NSObject, Retained,
};
use crate::{CompletionHandler, DeviceRequest, NSMutableData};

pub(crate) fn iousbhost_object_destroy(this: &NSObject) {
  this.never()
}

pub(crate) fn iousbhost_object_destroy_options(this: &NSObject, _options: usize) {
  this.never()
}

pub(crate) fn iousbhost_object_io_service(this: &NSObject) -> u32 {
  this.never()
}

pub(crate) fn iousbhost_object_queue(this: &NSObject) -> DispatchRetained<DispatchQueue> {
  this.never()
}

pub(crate) fn iousbhost_object_device_descriptor(this: &NSObject) -> Option<Vec<u8>> {
  this.never()
}

pub(crate) fn iousbhost_object_capability_descriptors(this: &NSObject) -> Option<Vec<u8>> {
  this.never()
}

pub(crate) fn iousbhost_object_configuration_descriptor_with(
  this: &NSObject,
  _index: isize,
) -> Result<Vec<u8>, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_configuration_descriptor_with_configuration_value(
  this: &NSObject,
  _configuration_value: isize,
) -> Result<Vec<u8>, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_descriptor_with_type(
  this: &NSObject,
  _descriptor_type: IOUSBDescriptorType,
  _length: isize,
  _index: isize,
  _language_id: isize,
  _request_type: IOUSBDeviceRequestTypeValue,
  _request_recipient: IOUSBDeviceRequestRecipientValue,
) -> Result<Vec<u8>, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_string_with_index(
  this: &NSObject,
  _index: isize,
  _language_id: isize,
) -> Result<String, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_io_data_with_capacity(
  this: &NSObject,
  _capacity: isize,
) -> Result<NSMutableData, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_data_with_capacity_options(
  this: &NSObject,
  _capacity: isize,
  _options: usize,
) -> Result<NSMutableData, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_send_device_request(
  this: &NSObject,
  _request: DeviceRequest,
  _data: Option<&NSMutableData>,
  _completion_timeout: f64,
) -> Result<isize, Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_enqueue_device_request(
  this: &NSObject,
  _request: DeviceRequest,
  _data: Option<&NSMutableData>,
  _completion_timeout: f64,
  _completion_handler: Option<CompletionHandler>,
) -> Result<(), Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_abort_device_requests_with_option(
  this: &NSObject,
  _option: IOUSBHostAbortOption,
) -> Result<(), Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_device_address(this: &NSObject) -> isize {
  this.never()
}

pub(crate) fn iousbhost_object_frame_number_with_time(this: &NSObject) -> (u64, u64) {
  this.never()
}

pub(crate) fn iousbhost_object_current_microframe_with_time(this: &NSObject) -> Result<(u64, u64), Retained<NSError>> {
  this.never()
}

pub(crate) fn iousbhost_object_reference_microframe_with_time(
  this: &NSObject,
) -> Result<(u64, u64), Retained<NSError>> {
  this.never()
}
