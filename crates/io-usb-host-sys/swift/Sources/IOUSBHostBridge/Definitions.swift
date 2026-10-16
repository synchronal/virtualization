//===----------------------------------------------------------------------===//
// `IOUSBHostDefinitions.h`, and the IOKit USB definitions its members take.
// An option set crosses as its raw value.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import IOUSBHost

func iousbhost_object_init_options_device_capture() -> UInt {
  IOUSBHostObjectInitOptions.deviceCapture.rawValue
}

func iousbhost_object_init_options_device_seize() -> UInt {
  IOUSBHostObjectInitOptions.deviceSeize.rawValue
}

func iousbhost_object_destroy_options_device_surrender() -> UInt {
  IOUSBHostObjectDestroyOptions.deviceSurrender.rawValue
}

func iousbhost_object_data_options_kernel_user_shared() -> UInt {
  IOUSBHostObjectDataOptions.kernelUserShared.rawValue
}

func iousbhost_device_request_type(
  direction: BridgedIOUSBDeviceRequestDirectionValue,
  request_type: BridgedIOUSBDeviceRequestTypeValue,
  recipient: BridgedIOUSBDeviceRequestRecipientValue
) -> UInt8 {
  IOUSBHostDeviceRequestType(
    tIOUSBDeviceRequestDirectionValue(direction),
    tIOUSBDeviceRequestTypeValue(request_type),
    tIOUSBDeviceRequestRecipientValue(recipient))
}

func iousbhost_default_control_completion_timeout() -> Double {
  IOUSBHostDefaultControlCompletionTimeout
}

func iousbhost_error_domain() -> RustString {
  IOUSBHostErrorDomain.intoRustString()
}

func iousbhost_property_key_location_id() -> RustString {
  IOUSBHostPropertyKeyLocationID.intoRustString()
}

func iousbhost_matching_property_key_vendor_id() -> RustString {
  IOUSBHostMatchingPropertyKey.vendorID.rawValue.intoRustString()
}

func iousbhost_matching_property_key_product_id() -> RustString {
  IOUSBHostMatchingPropertyKey.productID.rawValue.intoRustString()
}

func iousbhost_matching_property_key_product_id_mask() -> RustString {
  IOUSBHostMatchingPropertyKey.productIDMask.rawValue.intoRustString()
}

func iousbhost_matching_property_key_product_id_array() -> RustString {
  IOUSBHostMatchingPropertyKey.productIDArray.rawValue.intoRustString()
}

func iousbhost_matching_property_key_interface_number() -> RustString {
  IOUSBHostMatchingPropertyKey.interfaceNumber.rawValue.intoRustString()
}

func iousbhost_matching_property_key_configuration_value() -> RustString {
  IOUSBHostMatchingPropertyKey.configurationValue.rawValue.intoRustString()
}

func iousbhost_matching_property_key_device_release_number() -> RustString {
  IOUSBHostMatchingPropertyKey.deviceReleaseNumber.rawValue.intoRustString()
}

func iousbhost_matching_property_key_interface_class() -> RustString {
  IOUSBHostMatchingPropertyKey.interfaceClass.rawValue.intoRustString()
}

func iousbhost_matching_property_key_interface_sub_class() -> RustString {
  IOUSBHostMatchingPropertyKey.interfaceSubClass.rawValue.intoRustString()
}

func iousbhost_matching_property_key_interface_protocol() -> RustString {
  IOUSBHostMatchingPropertyKey.interfaceProtocol.rawValue.intoRustString()
}

func iousbhost_matching_property_key_device_class() -> RustString {
  IOUSBHostMatchingPropertyKey.deviceClass.rawValue.intoRustString()
}

func iousbhost_matching_property_key_device_sub_class() -> RustString {
  IOUSBHostMatchingPropertyKey.deviceSubClass.rawValue.intoRustString()
}

func iousbhost_matching_property_key_device_protocol() -> RustString {
  IOUSBHostMatchingPropertyKey.deviceProtocol.rawValue.intoRustString()
}

func iousbhost_matching_property_key_speed() -> RustString {
  IOUSBHostMatchingPropertyKey.speed.rawValue.intoRustString()
}

func iousbhost_device_property_key_vendor_string() -> RustString {
  IOUSBHostDevicePropertyKey.vendorString.rawValue.intoRustString()
}

func iousbhost_device_property_key_serial_number_string() -> RustString {
  IOUSBHostDevicePropertyKey.serialNumberString.rawValue.intoRustString()
}

func iousbhost_device_property_key_container_id() -> RustString {
  IOUSBHostDevicePropertyKey.containerID.rawValue.intoRustString()
}

func iousbhost_device_property_key_current_configuration() -> RustString {
  IOUSBHostDevicePropertyKey.currentConfiguration.rawValue.intoRustString()
}

extension IOUSBDeviceRequest {
  init(_ request: BridgedIOUSBDeviceRequest) {
    self.init(
      bmRequestType: request.bm_request_type,
      bRequest: request.b_request,
      wValue: request.w_value,
      wIndex: request.w_index,
      wLength: request.w_length)
  }
}

extension IOUSBHostAbortOption {
  init(_ option: BridgedIOUSBHostAbortOption) {
    switch option {
    case .Asynchronous: self = .asynchronous
    case .Synchronous: self = .synchronous
    case .Unknown(let rawValue): self = IOUSBHostAbortOption(rawValue: rawValue)!
    }
  }
}

extension tIOUSBDescriptorType {
  init(_ type: BridgedIOUSBDescriptorType) {
    switch type {
    case .Device: self = kIOUSBDescriptorTypeDevice
    case .Configuration: self = kIOUSBDescriptorTypeConfiguration
    case .String: self = kIOUSBDescriptorTypeString
    case .Interface: self = kIOUSBDescriptorTypeInterface
    case .Endpoint: self = kIOUSBDescriptorTypeEndpoint
    case .DeviceQualifier: self = kIOUSBDescriptorTypeDeviceQualifier
    case .OtherSpeedConfiguration: self = kIOUSBDescriptorTypeOtherSpeedConfiguration
    case .InterfacePower: self = kIOUSBDescriptorTypeInterfacePower
    case .Otg: self = kIOUSBDescriptorTypeOTG
    case .Debug: self = kIOUSBDescriptorTypeDebug
    case .InterfaceAssociation: self = kIOUSBDescriptorTypeInterfaceAssociation
    case .Bos: self = kIOUSBDescriptorTypeBOS
    case .DeviceCapability: self = kIOUSBDescriptorTypeDeviceCapability
    case .Hid: self = kIOUSBDecriptorTypeHID
    case .Report: self = kIOUSBDecriptorTypeReport
    case .Physical: self = kIOUSBDescriptorTypePhysical
    case .Hub: self = kIOUSBDescriptorTypeHub
    case .SuperSpeedHub: self = kIOUSBDescriptorTypeSuperSpeedHub
    case .SuperSpeedUSBEndpointCompanion: self = kIOUSBDescriptorTypeSuperSpeedUSBEndpointCompanion
    case .SuperSpeedPlusIsochronousEndpointCompanion:
      self = kIOUSBDescriptorTypeSuperSpeedPlusIsochronousEndpointCompanion
    case .Unknown(let rawValue): self = tIOUSBDescriptorType(rawValue: rawValue)
    }
  }
}

extension tIOUSBDeviceRequestDirectionValue {
  init(_ direction: BridgedIOUSBDeviceRequestDirectionValue) {
    switch direction {
    case .Out: self = kIOUSBDeviceRequestDirectionValueOut
    case .In: self = kIOUSBDeviceRequestDirectionValueIn
    case .Unknown(let rawValue): self = tIOUSBDeviceRequestDirectionValue(rawValue: rawValue)
    }
  }
}

extension tIOUSBDeviceRequestTypeValue {
  init(_ type: BridgedIOUSBDeviceRequestTypeValue) {
    switch type {
    case .Standard: self = kIOUSBDeviceRequestTypeValueStandard
    case .Class: self = kIOUSBDeviceRequestTypeValueClass
    case .Vendor: self = kIOUSBDeviceRequestTypeValueVendor
    case .Unknown(let rawValue): self = tIOUSBDeviceRequestTypeValue(rawValue: rawValue)
    }
  }
}

extension tIOUSBDeviceRequestRecipientValue {
  init(_ recipient: BridgedIOUSBDeviceRequestRecipientValue) {
    switch recipient {
    case .Device: self = kIOUSBDeviceRequestRecipientValueDevice
    case .Interface: self = kIOUSBDeviceRequestRecipientValueInterface
    case .Endpoint: self = kIOUSBDeviceRequestRecipientValueEndpoint
    case .Other: self = kIOUSBDeviceRequestRecipientValueOther
    case .Unknown(let rawValue): self = tIOUSBDeviceRequestRecipientValue(rawValue: rawValue)
    }
  }
}
