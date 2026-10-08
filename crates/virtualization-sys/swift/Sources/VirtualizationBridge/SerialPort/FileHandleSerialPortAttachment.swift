//===----------------------------------------------------------------------===//
// `VZFileHandleSerialPortAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// Each handle is a `FileHandle`, lent by address: `0` for `nil`.
func vz_file_handle_serial_port_attachment_init(
  file_handle_for_reading: UInt,
  file_handle_for_writing: UInt
) -> VZFileHandleSerialPortAttachment {
  VZFileHandleSerialPortAttachment(
    fileHandleForReading: bridging(borrowedAddress: file_handle_for_reading),
    fileHandleForWriting: bridging(borrowedAddress: file_handle_for_writing)
  )
}

/// `object as? VZFileHandleSerialPortAttachment`, for an abstract class's
/// object.
func vz_file_handle_serial_port_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZFileHandleSerialPortAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZFileHandleSerialPortAttachment)
}

func vz_file_handle_serial_port_attachment_file_handle_for_reading(this: VZFileHandleSerialPortAttachment) -> UInt {
  bridging(retainedAddress: this.fileHandleForReading)
}

func vz_file_handle_serial_port_attachment_file_handle_for_writing(this: VZFileHandleSerialPortAttachment) -> UInt {
  bridging(retainedAddress: this.fileHandleForWriting)
}
