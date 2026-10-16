use dispatch2::DispatchQueue;

use super::{NSError, Never, Retained};

pub(crate) struct AAUSBAccessory {
  never: Never,
}

impl AAUSBAccessory {
  pub(crate) fn bridge_retained(&self) -> AAUSBAccessory {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn aa_registry_id(&self) -> u64 {
    match self.never {}
  }

  pub(crate) fn aa_device_descriptor_data(&self) -> Vec<u8> {
    match self.never {}
  }

  pub(crate) fn aa_configuration_descriptor_data(&self) -> Option<Vec<u8>> {
    match self.never {}
  }

  pub(crate) fn aa_open(
    &self,
    _service_queue: Option<&DispatchQueue>,
    _completion_handler: impl FnOnce(Result<io_usb_host::Device, Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }

  pub(crate) fn aa_close(&self, _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    match self.never {}
  }
}
