//===----------------------------------------------------------------------===//
// `VZMacGuestProvisioningOptions`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_mac_guest_provisioning_options_init() -> VZMacGuestProvisioningOptions {
  VZMacGuestProvisioningOptions()
}

/// `object as? VZMacGuestProvisioningOptions`, for an abstract class's object.
func vz_mac_guest_provisioning_options_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil)
  -> VZMacGuestProvisioningOptions
{
  try bridging(bridging(borrowed: object) as NSObject as? VZMacGuestProvisioningOptions)
}

extension VZMacGuestProvisioningOptions {
  func vz_full_name() -> RustString {
    fullName.intoRustString()
  }

  func vz_set_full_name(full_name: RustStr) {
    fullName = full_name.toString()
  }

  func vz_username() -> RustString {
    username.intoRustString()
  }

  func vz_set_username(username: RustStr) {
    self.username = username.toString()
  }

  func vz_password() -> RustString {
    password.intoRustString()
  }

  func vz_set_password(password: RustStr) {
    self.password = password.toString()
  }

  func vz_logs_in_automatically() -> Bool {
    logsInAutomatically
  }

  func vz_set_logs_in_automatically(logs_in_automatically: Bool) {
    logsInAutomatically = logs_in_automatically
  }

  func vz_enables_remote_login() -> Bool {
    enablesRemoteLogin
  }

  func vz_set_enables_remote_login(enables_remote_login: Bool) {
    enablesRemoteLogin = enables_remote_login
  }
}
