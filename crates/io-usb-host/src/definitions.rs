//! `IOUSBHostDefinitions.h`, and the IOKit USB definitions its members take.

use crate::Error;
use crate::platform::{self, ffi};

/// [`IOUSBDeviceRequest`](https://developer.apple.com/documentation/kernel/iousbdevicerequest):
/// a C struct of plain values. `Default` is C's zero value, Swift's `init()`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeviceRequest {
  /// `bmRequestType`
  pub bm_request_type: u8,
  /// `bRequest`
  pub b_request: u8,
  /// `wValue`
  pub w_value: u16,
  /// `wIndex`
  pub w_index: u16,
  /// `wLength`
  pub w_length: u16,
}

/// The request as the sys crate takes it.
#[cfg(target_os = "macos")]
impl From<DeviceRequest> for ffi::BridgedIOUSBDeviceRequest {
  fn from(request: DeviceRequest) -> Self {
    Self {
      bm_request_type: request.bm_request_type,
      b_request: request.b_request,
      w_value: request.w_value,
      w_index: request.w_index,
      w_length: request.w_length,
    }
  }
}

/// [`IOUSBHostDeviceRequestType(_:_:_:)`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicerequesttype(_:_:_:)):
/// a `bmRequestType`.
pub fn device_request_type(
  direction: DeviceRequestDirectionValue,
  request_type: DeviceRequestTypeValue,
  recipient: DeviceRequestRecipientValue,
) -> Result<u8, Error> {
  platform::supported()?;

  Ok(ffi::iousbhost_device_request_type(
    direction.into(),
    request_type.into(),
    recipient.into(),
  ))
}

/// [`IOUSBHostDefaultControlCompletionTimeout`](https://developer.apple.com/documentation/iousbhost/iousbhostdefaultcontrolcompletiontimeout):
/// its seconds.
pub fn default_control_completion_timeout() -> Result<f64, Error> {
  platform::supported()?;

  Ok(ffi::iousbhost_default_control_completion_timeout())
}

/// [`IOUSBHostErrorDomain`](https://developer.apple.com/documentation/iousbhost/iousbhosterrordomain)
pub fn error_domain() -> Result<String, Error> {
  platform::supported()?;

  Ok(ffi::iousbhost_error_domain())
}

/// [`IOUSBHostPropertyKeyLocationID`](https://developer.apple.com/documentation/iousbhost/iousbhostpropertykeylocationid)
pub fn property_key_location_id() -> Result<String, Error> {
  platform::supported()?;

  Ok(ffi::iousbhost_property_key_location_id())
}

/// [`IOUSBHostMatchingPropertyKey`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey):
/// a key of a matching dictionary, which is its raw value. Each constant is
/// read from Swift.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchingPropertyKey {
  /// [`rawValue`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/rawvalue)
  pub raw_value: String,
}

impl MatchingPropertyKey {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/init(rawvalue:))
  pub fn new(raw_value: String) -> Self {
    Self { raw_value }
  }

  /// [`vendorID`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/vendorid)
  pub fn vendor_id() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_vendor_id()))
  }

  /// [`productID`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/productid)
  pub fn product_id() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_product_id()))
  }

  /// [`productIDMask`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/productidmask)
  pub fn product_id_mask() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_product_id_mask()))
  }

  /// [`productIDArray`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/productidarray)
  pub fn product_id_array() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_product_id_array()))
  }

  /// [`interfaceNumber`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/interfacenumber)
  pub fn interface_number() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_interface_number()))
  }

  /// [`configurationValue`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/configurationvalue)
  pub fn configuration_value() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_configuration_value()))
  }

  /// [`deviceReleaseNumber`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/devicereleasenumber)
  pub fn device_release_number() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_device_release_number()))
  }

  /// [`interfaceClass`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/interfaceclass)
  pub fn interface_class() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_interface_class()))
  }

  /// [`interfaceSubClass`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/interfacesubclass)
  pub fn interface_sub_class() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_interface_sub_class()))
  }

  /// [`interfaceProtocol`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/interfaceprotocol)
  pub fn interface_protocol() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_interface_protocol()))
  }

  /// [`deviceClass`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/deviceclass)
  pub fn device_class() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_device_class()))
  }

  /// [`deviceSubClass`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/devicesubclass)
  pub fn device_sub_class() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_device_sub_class()))
  }

  /// [`deviceProtocol`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/deviceprotocol)
  pub fn device_protocol() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_device_protocol()))
  }

  /// [`speed`](https://developer.apple.com/documentation/iousbhost/iousbhostmatchingpropertykey/speed)
  pub fn speed() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_matching_property_key_speed()))
  }
}

/// [`IOUSBHostDevicePropertyKey`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey):
/// a key of a device's properties, which is its raw value. Each constant is
/// read from Swift.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DevicePropertyKey {
  /// [`rawValue`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/rawvalue)
  pub raw_value: String,
}

impl DevicePropertyKey {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/init(rawvalue:))
  pub fn new(raw_value: String) -> Self {
    Self { raw_value }
  }

  /// [`vendorString`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/vendorstring)
  pub fn vendor_string() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_device_property_key_vendor_string()))
  }

  /// [`serialNumberString`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/serialnumberstring)
  pub fn serial_number_string() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_device_property_key_serial_number_string()))
  }

  /// [`containerID`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/containerid)
  pub fn container_id() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_device_property_key_container_id()))
  }

  /// [`currentConfiguration`](https://developer.apple.com/documentation/iousbhost/iousbhostdevicepropertykey/currentconfiguration)
  pub fn current_configuration() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_device_property_key_current_configuration()))
  }
}

/// [`tIOUSBDescriptorType`](https://developer.apple.com/documentation/kernel/tiousbdescriptortype)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DescriptorType {
  /// `kIOUSBDescriptorTypeDevice`
  Device,
  /// `kIOUSBDescriptorTypeConfiguration`
  Configuration,
  /// `kIOUSBDescriptorTypeString`
  String,
  /// `kIOUSBDescriptorTypeInterface`
  Interface,
  /// `kIOUSBDescriptorTypeEndpoint`
  Endpoint,
  /// `kIOUSBDescriptorTypeDeviceQualifier`
  DeviceQualifier,
  /// `kIOUSBDescriptorTypeOtherSpeedConfiguration`
  OtherSpeedConfiguration,
  /// `kIOUSBDescriptorTypeInterfacePower`
  InterfacePower,
  /// `kIOUSBDescriptorTypeOTG`
  Otg,
  /// `kIOUSBDescriptorTypeDebug`
  Debug,
  /// `kIOUSBDescriptorTypeInterfaceAssociation`
  InterfaceAssociation,
  /// `kIOUSBDescriptorTypeBOS`
  Bos,
  /// `kIOUSBDescriptorTypeDeviceCapability`
  DeviceCapability,
  /// `kIOUSBDecriptorTypeHID`, as Apple spells it.
  Hid,
  /// `kIOUSBDecriptorTypeReport`, as Apple spells it.
  Report,
  /// `kIOUSBDescriptorTypePhysical`
  Physical,
  /// `kIOUSBDescriptorTypeHub`
  Hub,
  /// `kIOUSBDescriptorTypeSuperSpeedHub`
  SuperSpeedHub,
  /// `kIOUSBDescriptorTypeSuperSpeedUSBEndpointCompanion`
  SuperSpeedUSBEndpointCompanion,
  /// `kIOUSBDescriptorTypeSuperSpeedPlusIsochronousEndpointCompanion`
  SuperSpeedPlusIsochronousEndpointCompanion,
  /// A value newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<ffi::IOUSBDescriptorType> for DescriptorType {
  fn from(value: ffi::IOUSBDescriptorType) -> Self {
    match value {
      ffi::IOUSBDescriptorType::Device => Self::Device,
      ffi::IOUSBDescriptorType::Configuration => Self::Configuration,
      ffi::IOUSBDescriptorType::String => Self::String,
      ffi::IOUSBDescriptorType::Interface => Self::Interface,
      ffi::IOUSBDescriptorType::Endpoint => Self::Endpoint,
      ffi::IOUSBDescriptorType::DeviceQualifier => Self::DeviceQualifier,
      ffi::IOUSBDescriptorType::OtherSpeedConfiguration => Self::OtherSpeedConfiguration,
      ffi::IOUSBDescriptorType::InterfacePower => Self::InterfacePower,
      ffi::IOUSBDescriptorType::Otg => Self::Otg,
      ffi::IOUSBDescriptorType::Debug => Self::Debug,
      ffi::IOUSBDescriptorType::InterfaceAssociation => Self::InterfaceAssociation,
      ffi::IOUSBDescriptorType::Bos => Self::Bos,
      ffi::IOUSBDescriptorType::DeviceCapability => Self::DeviceCapability,
      ffi::IOUSBDescriptorType::Hid => Self::Hid,
      ffi::IOUSBDescriptorType::Report => Self::Report,
      ffi::IOUSBDescriptorType::Physical => Self::Physical,
      ffi::IOUSBDescriptorType::Hub => Self::Hub,
      ffi::IOUSBDescriptorType::SuperSpeedHub => Self::SuperSpeedHub,
      ffi::IOUSBDescriptorType::SuperSpeedUSBEndpointCompanion => Self::SuperSpeedUSBEndpointCompanion,
      ffi::IOUSBDescriptorType::SuperSpeedPlusIsochronousEndpointCompanion => {
        Self::SuperSpeedPlusIsochronousEndpointCompanion
      }
      ffi::IOUSBDescriptorType::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<DescriptorType> for ffi::IOUSBDescriptorType {
  fn from(value: DescriptorType) -> Self {
    match value {
      DescriptorType::Device => Self::Device,
      DescriptorType::Configuration => Self::Configuration,
      DescriptorType::String => Self::String,
      DescriptorType::Interface => Self::Interface,
      DescriptorType::Endpoint => Self::Endpoint,
      DescriptorType::DeviceQualifier => Self::DeviceQualifier,
      DescriptorType::OtherSpeedConfiguration => Self::OtherSpeedConfiguration,
      DescriptorType::InterfacePower => Self::InterfacePower,
      DescriptorType::Otg => Self::Otg,
      DescriptorType::Debug => Self::Debug,
      DescriptorType::InterfaceAssociation => Self::InterfaceAssociation,
      DescriptorType::Bos => Self::Bos,
      DescriptorType::DeviceCapability => Self::DeviceCapability,
      DescriptorType::Hid => Self::Hid,
      DescriptorType::Report => Self::Report,
      DescriptorType::Physical => Self::Physical,
      DescriptorType::Hub => Self::Hub,
      DescriptorType::SuperSpeedHub => Self::SuperSpeedHub,
      DescriptorType::SuperSpeedUSBEndpointCompanion => Self::SuperSpeedUSBEndpointCompanion,
      DescriptorType::SuperSpeedPlusIsochronousEndpointCompanion => Self::SuperSpeedPlusIsochronousEndpointCompanion,
      DescriptorType::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`tIOUSBDeviceRequestDirectionValue`](https://developer.apple.com/documentation/kernel/tiousbdevicerequestdirectionvalue)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceRequestDirectionValue {
  /// `kIOUSBDeviceRequestDirectionValueOut`
  Out,
  /// `kIOUSBDeviceRequestDirectionValueIn`
  In,
  /// A value newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<ffi::IOUSBDeviceRequestDirectionValue> for DeviceRequestDirectionValue {
  fn from(value: ffi::IOUSBDeviceRequestDirectionValue) -> Self {
    match value {
      ffi::IOUSBDeviceRequestDirectionValue::Out => Self::Out,
      ffi::IOUSBDeviceRequestDirectionValue::In => Self::In,
      ffi::IOUSBDeviceRequestDirectionValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<DeviceRequestDirectionValue> for ffi::IOUSBDeviceRequestDirectionValue {
  fn from(value: DeviceRequestDirectionValue) -> Self {
    match value {
      DeviceRequestDirectionValue::Out => Self::Out,
      DeviceRequestDirectionValue::In => Self::In,
      DeviceRequestDirectionValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`tIOUSBDeviceRequestTypeValue`](https://developer.apple.com/documentation/kernel/tiousbdevicerequesttypevalue)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceRequestTypeValue {
  /// `kIOUSBDeviceRequestTypeValueStandard`
  Standard,
  /// `kIOUSBDeviceRequestTypeValueClass`
  Class,
  /// `kIOUSBDeviceRequestTypeValueVendor`
  Vendor,
  /// A value newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<ffi::IOUSBDeviceRequestTypeValue> for DeviceRequestTypeValue {
  fn from(value: ffi::IOUSBDeviceRequestTypeValue) -> Self {
    match value {
      ffi::IOUSBDeviceRequestTypeValue::Standard => Self::Standard,
      ffi::IOUSBDeviceRequestTypeValue::Class => Self::Class,
      ffi::IOUSBDeviceRequestTypeValue::Vendor => Self::Vendor,
      ffi::IOUSBDeviceRequestTypeValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<DeviceRequestTypeValue> for ffi::IOUSBDeviceRequestTypeValue {
  fn from(value: DeviceRequestTypeValue) -> Self {
    match value {
      DeviceRequestTypeValue::Standard => Self::Standard,
      DeviceRequestTypeValue::Class => Self::Class,
      DeviceRequestTypeValue::Vendor => Self::Vendor,
      DeviceRequestTypeValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`tIOUSBDeviceRequestRecipientValue`](https://developer.apple.com/documentation/kernel/tiousbdevicerequestrecipientvalue)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceRequestRecipientValue {
  /// `kIOUSBDeviceRequestRecipientValueDevice`
  Device,
  /// `kIOUSBDeviceRequestRecipientValueInterface`
  Interface,
  /// `kIOUSBDeviceRequestRecipientValueEndpoint`
  Endpoint,
  /// `kIOUSBDeviceRequestRecipientValueOther`
  Other,
  /// A value newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<ffi::IOUSBDeviceRequestRecipientValue> for DeviceRequestRecipientValue {
  fn from(value: ffi::IOUSBDeviceRequestRecipientValue) -> Self {
    match value {
      ffi::IOUSBDeviceRequestRecipientValue::Device => Self::Device,
      ffi::IOUSBDeviceRequestRecipientValue::Interface => Self::Interface,
      ffi::IOUSBDeviceRequestRecipientValue::Endpoint => Self::Endpoint,
      ffi::IOUSBDeviceRequestRecipientValue::Other => Self::Other,
      ffi::IOUSBDeviceRequestRecipientValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<DeviceRequestRecipientValue> for ffi::IOUSBDeviceRequestRecipientValue {
  fn from(value: DeviceRequestRecipientValue) -> Self {
    match value {
      DeviceRequestRecipientValue::Device => Self::Device,
      DeviceRequestRecipientValue::Interface => Self::Interface,
      DeviceRequestRecipientValue::Endpoint => Self::Endpoint,
      DeviceRequestRecipientValue::Other => Self::Other,
      DeviceRequestRecipientValue::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_descriptor_types_both_ways() {
    for value in [
      DescriptorType::Device,
      DescriptorType::Configuration,
      DescriptorType::String,
      DescriptorType::Interface,
      DescriptorType::Endpoint,
      DescriptorType::DeviceQualifier,
      DescriptorType::OtherSpeedConfiguration,
      DescriptorType::InterfacePower,
      DescriptorType::Otg,
      DescriptorType::Debug,
      DescriptorType::InterfaceAssociation,
      DescriptorType::Bos,
      DescriptorType::DeviceCapability,
      DescriptorType::Hid,
      DescriptorType::Report,
      DescriptorType::Physical,
      DescriptorType::Hub,
      DescriptorType::SuperSpeedHub,
      DescriptorType::SuperSpeedUSBEndpointCompanion,
      DescriptorType::SuperSpeedPlusIsochronousEndpointCompanion,
      DescriptorType::Unknown(99),
    ] {
      assert_eq!(DescriptorType::from(ffi::IOUSBDescriptorType::from(value)), value);
    }
  }

  #[test]
  fn maps_device_request_values_both_ways() {
    for value in [
      DeviceRequestDirectionValue::Out,
      DeviceRequestDirectionValue::In,
      DeviceRequestDirectionValue::Unknown(9),
    ] {
      assert_eq!(
        DeviceRequestDirectionValue::from(ffi::IOUSBDeviceRequestDirectionValue::from(value)),
        value
      );
    }

    for value in [
      DeviceRequestTypeValue::Standard,
      DeviceRequestTypeValue::Class,
      DeviceRequestTypeValue::Vendor,
      DeviceRequestTypeValue::Unknown(9),
    ] {
      assert_eq!(
        DeviceRequestTypeValue::from(ffi::IOUSBDeviceRequestTypeValue::from(value)),
        value
      );
    }

    for value in [
      DeviceRequestRecipientValue::Device,
      DeviceRequestRecipientValue::Interface,
      DeviceRequestRecipientValue::Endpoint,
      DeviceRequestRecipientValue::Other,
      DeviceRequestRecipientValue::Unknown(9),
    ] {
      assert_eq!(
        DeviceRequestRecipientValue::from(ffi::IOUSBDeviceRequestRecipientValue::from(value)),
        value
      );
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(
      device_request_type(
        DeviceRequestDirectionValue::In,
        DeviceRequestTypeValue::Standard,
        DeviceRequestRecipientValue::Device
      ),
      Err(Error::Unsupported)
    ));
    assert!(matches!(default_control_completion_timeout(), Err(Error::Unsupported)));
    assert!(matches!(error_domain(), Err(Error::Unsupported)));
    assert!(matches!(property_key_location_id(), Err(Error::Unsupported)));
    assert!(matches!(MatchingPropertyKey::vendor_id(), Err(Error::Unsupported)));
    assert!(matches!(DevicePropertyKey::vendor_string(), Err(Error::Unsupported)));
  }
}
