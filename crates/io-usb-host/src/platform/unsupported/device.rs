use std::collections::BTreeMap;

use dispatch2::DispatchQueue;

use super::{GUARDED, NSError, NSObject, Never, Retained};
use crate::{DeviceMatchingDictionaryArguments, PropertyListValue};

pub(crate) struct IOUSBHostDevice {
  never: Never,
}

pub(crate) type IOUSBHostInterestHandler = Box<dyn Fn(IOUSBHostDevice, u32, usize) + Send + Sync + 'static>;

pub(crate) fn iousbhost_device_init_io_service_options_queue_interest_handler(
  _io_service: u32,
  _options: usize,
  _queue: Option<&DispatchQueue>,
  _interest_handler: Option<IOUSBHostInterestHandler>,
) -> Result<IOUSBHostDevice, Retained<NSError>> {
  unreachable!("{GUARDED}")
}

pub(crate) fn iousbhost_device_create_matching_dictionary(
  _arguments: &DeviceMatchingDictionaryArguments,
) -> BTreeMap<String, PropertyListValue> {
  unreachable!("{GUARDED}")
}

impl IOUSBHostDevice {
  pub(crate) fn bridge_retained(&self) -> IOUSBHostDevice {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn iousbhost_configuration_descriptor(&self) -> Option<Vec<u8>> {
    match self.never {}
  }

  pub(crate) fn iousbhost_configure_with_value(
    &self,
    _value: isize,
    _match_interfaces: bool,
  ) -> Result<(), Retained<NSError>> {
    match self.never {}
  }

  pub(crate) fn iousbhost_reset(&self) -> Result<(), Retained<NSError>> {
    match self.never {}
  }
}
