use std::sync::Arc;

use super::{AAUSBAccessory, AAUSBAccessoryMatchingCriteria, GUARDED, NSError, Never, Retained};
use crate::USBAccessoryListener;

pub(crate) struct AAUSBAccessoryManager {
  never: Never,
}

pub(crate) fn aa_usb_accessory_manager_shared() -> AAUSBAccessoryManager {
  unreachable!("{GUARDED}")
}

impl AAUSBAccessoryManager {
  pub(crate) fn bridge_retained(&self) -> AAUSBAccessoryManager {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn aa_register_listener(
    &self,
    _listener: Arc<dyn USBAccessoryListener>,
    _key: usize,
    _matching_criteria: &[&AAUSBAccessoryMatchingCriteria],
    _completion_handler: impl FnOnce(Result<Vec<AAUSBAccessory>, Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }

  pub(crate) fn aa_unregister_listener(
    &self,
    _listener: Arc<dyn USBAccessoryListener>,
    _key: usize,
    _completion_handler: impl FnOnce() + Send + 'static,
  ) {
    match self.never {}
  }
}
