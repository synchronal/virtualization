use std::collections::BTreeMap;
use std::fmt;

use crate::platform::{self, ffi};
use crate::{Error, PropertyListValue};

/// [`AAUSBAccessoryMatchingCriteria`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria)
pub struct USBAccessoryMatchingCriteria {
  pub(crate) inner: ffi::AAUSBAccessoryMatchingCriteria,
}

impl USBAccessoryMatchingCriteria {
  /// [`init(deviceMatchingDictionary:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/init(devicematchingdictionary:)):
  /// `None` when AccessoryAccess returns `nil`.
  pub fn new(device_matching_dictionary: &BTreeMap<String, PropertyListValue>) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(
      ffi::aa_usb_accessory_matching_criteria_init_device_matching_dictionary(device_matching_dictionary)
        .map(|inner| Self { inner }),
    )
  }

  /// [`init(deviceMatchingDictionary:interfaceMatchingDictionaries:interfaceMatchingOption:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/init(devicematchingdictionary:interfacematchingdictionaries:interfacematchingoption:)):
  /// `None` when AccessoryAccess returns `nil`. `device_matching_dictionary`
  /// `None` is `nil`.
  pub fn with_interface_matching_dictionaries_and_interface_matching_option(
    device_matching_dictionary: Option<&BTreeMap<String, PropertyListValue>>,
    interface_matching_dictionaries: &[BTreeMap<String, PropertyListValue>],
    interface_matching_option: USBAccessoryMatchingCriteriaInterfaceMatchingOption,
  ) -> Result<Option<Self>, Error> {
    platform::supported()?;

    Ok(
      ffi::aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
        device_matching_dictionary,
        interface_matching_dictionaries,
        interface_matching_option.into(),
      )
      .map(|inner| Self { inner }),
    )
  }

  /// [`init(deviceMatchingCriteria:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/init(devicematchingcriteria:))
  pub fn with_device_matching_criteria(
    device_matching_criteria: &USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::aa_usb_accessory_matching_criteria_init_device_matching_criteria(&bridged_device_matching_criteria(
        device_matching_criteria,
      )),
    })
  }

  /// [`init(deviceMatchingCriteria:interfaceMatchingCriteria:interfaceMatchingOption:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/init(devicematchingcriteria:interfacematchingcriteria:interfacematchingoption:)).
  /// `device_matching_criteria` `None` is `nil`.
  pub fn with_device_matching_criteria_and_interface_matching_criteria_and_interface_matching_option(
    device_matching_criteria: Option<&USBAccessoryMatchingCriteriaDeviceMatchingCriteria>,
    interface_matching_criteria: &[USBAccessoryMatchingCriteriaInterfaceMatchingCriteria],
    interface_matching_option: USBAccessoryMatchingCriteriaInterfaceMatchingOption,
  ) -> Result<Self, Error> {
    platform::supported()?;

    let device_matching_criteria = device_matching_criteria.map(bridged_device_matching_criteria);
    let interface_matching_criteria: Vec<_> = interface_matching_criteria
      .iter()
      .map(bridged_interface_matching_criteria)
      .collect();

    Ok(Self {
      inner: ffi::aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
        device_matching_criteria.as_ref(),
        &interface_matching_criteria,
        interface_matching_option.into(),
      ),
    })
  }
}

impl Clone for USBAccessoryMatchingCriteria {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBAccessoryMatchingCriteria {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria):
/// a struct of plain values, in Rust as in Swift. Each `None` is `nil`.
/// `Default` is [`init(vendorID:productID:deviceClass:deviceSubClass:deviceProtocol:speed:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/init(vendorid:productid:deviceclass:devicesubclass:deviceprotocol:speed:))
/// with each argument its default, `nil`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct USBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  /// [`vendorID`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/vendorid)
  pub vendor_id: Option<isize>,
  /// [`productID`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/productid)
  pub product_id: Option<isize>,
  /// [`deviceClass`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/deviceclass)
  pub device_class: Option<isize>,
  /// [`deviceSubClass`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/devicesubclass)
  pub device_sub_class: Option<isize>,
  /// [`deviceProtocol`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/deviceprotocol)
  pub device_protocol: Option<isize>,
  /// [`speed`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/devicematchingcriteria/speed)
  pub speed: Option<isize>,
}

/// The criteria as the sys crate takes them.
#[cfg(target_os = "macos")]
fn bridged_device_matching_criteria(
  criteria: &USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
) -> ffi::AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  ffi::AAUSBAccessoryMatchingCriteriaDeviceMatchingCriteria {
    vendor_id: criteria.vendor_id,
    product_id: criteria.product_id,
    device_class: criteria.device_class,
    device_sub_class: criteria.device_sub_class,
    device_protocol: criteria.device_protocol,
    speed: criteria.speed,
  }
}

/// Off macOS, `platform::unsupported` takes the criteria as they are.
#[cfg(not(target_os = "macos"))]
fn bridged_device_matching_criteria(
  criteria: &USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
) -> USBAccessoryMatchingCriteriaDeviceMatchingCriteria {
  *criteria
}

/// [`AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria):
/// a struct of plain values, in Rust as in Swift. Each `None` is `nil`.
/// `Default` is [`init(vendorID:productID:bcdDevice:interfaceNumber:configurationValue:interfaceClass:interfaceSubClass:interfaceProtocol:speed:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/init(vendorid:productid:bcddevice:interfacenumber:configurationvalue:interfaceclass:interfacesubclass:interfaceprotocol:speed:))
/// with each argument its default, `nil`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct USBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
  /// [`vendorID`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/vendorid)
  pub vendor_id: Option<isize>,
  /// [`productID`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/productid)
  pub product_id: Option<isize>,
  /// [`bcdDevice`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/bcddevice)
  pub bcd_device: Option<isize>,
  /// [`interfaceNumber`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/interfacenumber)
  pub interface_number: Option<isize>,
  /// [`configurationValue`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/configurationvalue)
  pub configuration_value: Option<isize>,
  /// [`interfaceClass`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/interfaceclass)
  pub interface_class: Option<isize>,
  /// [`interfaceSubClass`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/interfacesubclass)
  pub interface_sub_class: Option<isize>,
  /// [`interfaceProtocol`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/interfaceprotocol)
  pub interface_protocol: Option<isize>,
  /// [`speed`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingcriteria/speed)
  pub speed: Option<isize>,
}

/// The criteria as the sys crate takes them.
#[cfg(target_os = "macos")]
fn bridged_interface_matching_criteria(
  criteria: &USBAccessoryMatchingCriteriaInterfaceMatchingCriteria,
) -> ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
  ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
    vendor_id: criteria.vendor_id,
    product_id: criteria.product_id,
    bcd_device: criteria.bcd_device,
    interface_number: criteria.interface_number,
    configuration_value: criteria.configuration_value,
    interface_class: criteria.interface_class,
    interface_sub_class: criteria.interface_sub_class,
    interface_protocol: criteria.interface_protocol,
    speed: criteria.speed,
  }
}

/// Off macOS, `platform::unsupported` takes the criteria as they are.
#[cfg(not(target_os = "macos"))]
fn bridged_interface_matching_criteria(
  criteria: &USBAccessoryMatchingCriteriaInterfaceMatchingCriteria,
) -> USBAccessoryMatchingCriteriaInterfaceMatchingCriteria {
  *criteria
}

/// [`AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingoption)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum USBAccessoryMatchingCriteriaInterfaceMatchingOption {
  /// [`matchAll`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingoption/matchall)
  MatchAll,
  /// [`matchAny`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymatchingcriteria/interfacematchingoption/matchany)
  MatchAny,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl From<ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption>
  for USBAccessoryMatchingCriteriaInterfaceMatchingOption
{
  fn from(option: ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption) -> Self {
    match option {
      ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAll => Self::MatchAll,
      ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAny => Self::MatchAny,
      ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<USBAccessoryMatchingCriteriaInterfaceMatchingOption>
  for ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption
{
  fn from(option: USBAccessoryMatchingCriteriaInterfaceMatchingOption) -> Self {
    match option {
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAll => Self::MatchAll,
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAny => Self::MatchAny,
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_interface_matching_options_both_ways() {
    for option in [
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAll,
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAny,
      USBAccessoryMatchingCriteriaInterfaceMatchingOption::Unknown(7),
    ] {
      assert_eq!(
        USBAccessoryMatchingCriteriaInterfaceMatchingOption::from(
          ffi::AAUSBAccessoryMatchingCriteriaInterfaceMatchingOption::from(option)
        ),
        option
      );
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      USBAccessoryMatchingCriteria::new(&BTreeMap::new()),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      USBAccessoryMatchingCriteria::with_interface_matching_dictionaries_and_interface_matching_option(
        None,
        &[],
        USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAll
      ),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      USBAccessoryMatchingCriteria::with_device_matching_criteria(
        &USBAccessoryMatchingCriteriaDeviceMatchingCriteria::default()
      ),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      USBAccessoryMatchingCriteria::with_device_matching_criteria_and_interface_matching_criteria_and_interface_matching_option(
        None,
        &[],
        USBAccessoryMatchingCriteriaInterfaceMatchingOption::MatchAny
      ),
      Err(Error::Unsupported)
    ));
  }
}
