//===----------------------------------------------------------------------===//
// `VZSpiceAgentPortAttachment`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_spice_agent_port_attachment_init() -> VZSpiceAgentPortAttachment {
  VZSpiceAgentPortAttachment()
}

/// `object as? VZSpiceAgentPortAttachment`, for an abstract class's object.
func vz_spice_agent_port_attachment_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZSpiceAgentPortAttachment
{
  try bridging(bridging(borrowed: object) as NSObject as? VZSpiceAgentPortAttachment)
}

func vz_spice_agent_port_attachment_spice_agent_port_name() -> RustString {
  VZSpiceAgentPortAttachment.spiceAgentPortName.intoRustString()
}

extension VZSpiceAgentPortAttachment {
  func vz_shares_clipboard() -> Bool {
    sharesClipboard
  }

  func vz_set_shares_clipboard(shares_clipboard: Bool) {
    sharesClipboard = shares_clipboard
  }
}
