use std::collections::BTreeMap;

use super::{GUARDED, Never};
use crate::{
  PropertyListValue, USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
  USBAccessoryMatchingCriteriaInterfaceMatchingCriteria,
};

pub(crate) struct AAUSBAccessoryMatchingCriteria {
  never: Never,
}

pub(crate) enum AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption {
  MatchAll,
  MatchAny,
  Unknown(isize),
}

pub(crate) fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary(
  _dictionary: &BTreeMap<String, PropertyListValue>,
) -> Option<AAUSBAccessoryMatchingCriteria> {
  unreachable!("{GUARDED}")
}

pub(crate) fn aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
  _device_matching_dictionary: Option<&BTreeMap<String, PropertyListValue>>,
  _interface_matching_dictionaries: &[BTreeMap<String, PropertyListValue>],
  _interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
) -> Option<AAUSBAccessoryMatchingCriteria> {
  unreachable!("{GUARDED}")
}

pub(crate) fn aa_usb_accessory_matching_criteria_init_device_matching_criteria(
  _device_matching_criteria: &USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
) -> AAUSBAccessoryMatchingCriteria {
  unreachable!("{GUARDED}")
}

pub(crate) fn aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
  _device_matching_criteria: Option<&USBAccessoryMatchingCriteriaDeviceMatchingCriteria>,
  _interface_matching_criteria: &[USBAccessoryMatchingCriteriaInterfaceMatchingCriteria],
  _interface_matching_option: AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption,
) -> AAUSBAccessoryMatchingCriteria {
  unreachable!("{GUARDED}")
}

impl AAUSBAccessoryMatchingCriteria {
  pub(crate) fn bridge_retained(&self) -> AAUSBAccessoryMatchingCriteria {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}
