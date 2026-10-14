//===----------------------------------------------------------------------===//
// `VZFileHandleNetworkDeviceAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `file_handle` is a `FileHandle`, lent by address.
func vz_file_handle_network_device_attachment_init(file_handle: UnsafeRawPointer)
  -> VZFileHandleNetworkDeviceAttachment
{
  VZFileHandleNetworkDeviceAttachment(fileHandle: bridging(borrowed: file_handle))
}

/// `object as? VZFileHandleNetworkDeviceAttachment`, for an abstract class's
/// object.
func vz_file_handle_network_device_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZFileHandleNetworkDeviceAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZFileHandleNetworkDeviceAttachment)
}

func vz_file_handle_network_device_attachment_file_handle(this: VZFileHandleNetworkDeviceAttachment)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.fileHandle)
}

extension VZFileHandleNetworkDeviceAttachment {
  func vz_maximum_transmission_unit() -> Int {
    maximumTransmissionUnit
  }

  func vz_set_maximum_transmission_unit(maximum_transmission_unit: Int) {
    maximumTransmissionUnit = maximum_transmission_unit
  }
}
