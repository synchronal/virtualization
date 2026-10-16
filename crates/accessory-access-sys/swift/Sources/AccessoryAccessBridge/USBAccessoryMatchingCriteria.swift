//===----------------------------------------------------------------------===//
// `AAUSBAccessoryMatchingCriteria`, its `DeviceMatchingCriteria` and
// `InterfaceMatchingCriteria`, and its `InterfaceMatchingOption`. A matching
// criteria struct crosses as an `NSDictionary` of its non-`nil` fields, by
// their Swift names.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess

/// `dictionary` is an `NSDictionary`, lent for the call.
func aa_usb_accessory_matching_criteria_init_device_matching_dictionary(dictionary: UnsafeRawPointer)
  throws(AccessoryAccessNil) -> AAUSBAccessoryMatchingCriteria
{
  try bridging(
    AAUSBAccessoryMatchingCriteria(
      deviceMatchingDictionary: bridging(sendableDictionary: bridging(borrowed: dictionary))))
}

/// `device_matching_dictionary` is an `NSDictionary` lent by address, `0`
/// for `nil`; `interface_matching_dictionaries` an `NSArray` of them, lent.
func
  aa_usb_accessory_matching_criteria_init_device_matching_dictionary_interface_matching_dictionaries_interface_matching_option(
    device_matching_dictionary: UInt,
    interface_matching_dictionaries: UnsafeRawPointer,
    interface_matching_option: BridgedInterfaceMatchingOption
  ) throws(AccessoryAccessNil) -> AAUSBAccessoryMatchingCriteria
{
  let interfaceMatchingDictionaries: [NSDictionary] = bridging(borrowedArray: interface_matching_dictionaries)

  return try bridging(
    AAUSBAccessoryMatchingCriteria(
      deviceMatchingDictionary: (bridging(borrowedAddress: device_matching_dictionary) as NSDictionary?).map {
        bridging(sendableDictionary: $0)
      },
      interfaceMatchingDictionaries: interfaceMatchingDictionaries.map { bridging(sendableDictionary: $0) },
      interfaceMatchingOption: AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption(interface_matching_option)))
}

/// `device_matching_criteria` is an `NSDictionary`, lent for the call.
func aa_usb_accessory_matching_criteria_init_device_matching_criteria(device_matching_criteria: UnsafeRawPointer)
  -> AAUSBAccessoryMatchingCriteria
{
  AAUSBAccessoryMatchingCriteria(
    deviceMatchingCriteria: AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria(
      bridging: bridging(borrowed: device_matching_criteria)))
}

/// `device_matching_criteria` is an `NSDictionary` lent by address, `0` for
/// `nil`; `interface_matching_criteria` an `NSArray` of them, lent.
func
  aa_usb_accessory_matching_criteria_init_device_matching_criteria_interface_matching_criteria_interface_matching_option(
    device_matching_criteria: UInt,
    interface_matching_criteria: UnsafeRawPointer,
    interface_matching_option: BridgedInterfaceMatchingOption
  ) -> AAUSBAccessoryMatchingCriteria
{
  let interfaceMatchingCriteria: [NSDictionary] = bridging(borrowedArray: interface_matching_criteria)

  return AAUSBAccessoryMatchingCriteria(
    deviceMatchingCriteria: (bridging(borrowedAddress: device_matching_criteria) as NSDictionary?).map {
      AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria(bridging: $0)
    },
    interfaceMatchingCriteria: interfaceMatchingCriteria.map {
      AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria(bridging: $0)
    },
    interfaceMatchingOption: AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption(interface_matching_option))
}

/// A matching criteria field, from the `NSDictionary` it crosses in.
private func bridging(field: String, of dictionary: NSDictionary) -> Int? {
  (dictionary[field] as? NSNumber)?.intValue
}

extension AAUSBAccessoryMatchingCriteria.DeviceMatchingCriteria {
  init(bridging dictionary: NSDictionary) {
    self.init(
      vendorID: bridging(field: "vendorID", of: dictionary),
      productID: bridging(field: "productID", of: dictionary),
      deviceClass: bridging(field: "deviceClass", of: dictionary),
      deviceSubClass: bridging(field: "deviceSubClass", of: dictionary),
      deviceProtocol: bridging(field: "deviceProtocol", of: dictionary),
      speed: bridging(field: "speed", of: dictionary))
  }
}

extension AAUSBAccessoryMatchingCriteria.InterfaceMatchingCriteria {
  init(bridging dictionary: NSDictionary) {
    self.init(
      vendorID: bridging(field: "vendorID", of: dictionary),
      productID: bridging(field: "productID", of: dictionary),
      bcdDevice: bridging(field: "bcdDevice", of: dictionary),
      interfaceNumber: bridging(field: "interfaceNumber", of: dictionary),
      configurationValue: bridging(field: "configurationValue", of: dictionary),
      interfaceClass: bridging(field: "interfaceClass", of: dictionary),
      interfaceSubClass: bridging(field: "interfaceSubClass", of: dictionary),
      interfaceProtocol: bridging(field: "interfaceProtocol", of: dictionary),
      speed: bridging(field: "speed", of: dictionary))
  }
}

extension AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption {
  init(_ option: BridgedInterfaceMatchingOption) {
    switch option {
    case .MatchAll: self = .matchAll
    case .MatchAny: self = .matchAny
    case .Unknown(let rawValue): self = AAUSBAccessoryMatchingCriteria.InterfaceMatchingOption(rawValue: rawValue)!
    }
  }
}
