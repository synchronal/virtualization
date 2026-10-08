//===----------------------------------------------------------------------===//
// `VZFileSerialPortAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_file_serial_port_attachment_init(url: UnsafeRawPointer, append: Bool) throws(BridgedThrown)
  -> VZFileSerialPortAttachment
{
  try bridging {
    try VZFileSerialPortAttachment(url: bridging(borrowed: url) as NSURL as URL, append: append)
  }
}

/// `object as? VZFileSerialPortAttachment`, for an abstract class's object.
func vz_file_serial_port_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZFileSerialPortAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZFileSerialPortAttachment)
}

func vz_file_serial_port_attachment_url(this: VZFileSerialPortAttachment) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

extension VZFileSerialPortAttachment {
  func vz_append() -> Bool {
    append
  }
}
