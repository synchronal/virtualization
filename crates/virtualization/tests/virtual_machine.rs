#![cfg(all(target_os = "macos", feature = "integration"))]

use virtualization::VirtualMachine;

#[test]
fn is_supported_on_the_host() {
  assert!(VirtualMachine::is_supported());
}
