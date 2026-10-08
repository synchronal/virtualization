//===----------------------------------------------------------------------===//
// `VZVirtioConsolePortConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_console_port_configuration_init() -> VZVirtioConsolePortConfiguration {
  VZVirtioConsolePortConfiguration()
}

/// `object as? VZVirtioConsolePortConfiguration`, for an abstract class's
/// object.
func vz_virtio_console_port_configuration_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZVirtioConsolePortConfiguration
{
  try bridging(bridging(borrowed: object) as NSObject as? VZVirtioConsolePortConfiguration)
}

extension VZVirtioConsolePortConfiguration {
  func vz_name() -> RustString? {
    name?.intoRustString()
  }

  func vz_set_name(name: RustStr?) {
    self.name = name?.toString()
  }

  func vz_is_console() -> Bool {
    isConsole
  }

  func vz_set_is_console(is_console: Bool) {
    isConsole = is_console
  }
}
