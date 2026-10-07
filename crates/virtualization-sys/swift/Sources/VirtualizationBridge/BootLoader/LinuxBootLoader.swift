//===----------------------------------------------------------------------===//
// `VZLinuxBootLoader`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_linux_boot_loader_init(kernel_url: UnsafeRawPointer) -> VZLinuxBootLoader {
  VZLinuxBootLoader(kernelURL: bridging(borrowed: kernel_url) as NSURL as URL)
}

/// `object as? VZLinuxBootLoader`, for an abstract class's object.
func vz_linux_boot_loader_bridge_cast(object: UnsafeRawPointer) throws(BridgedNil) -> VZLinuxBootLoader {
  try bridging(bridging(borrowed: object) as NSObject as? VZLinuxBootLoader)
}

func vz_linux_boot_loader_kernel_url(this: VZLinuxBootLoader) -> UnsafeMutableRawPointer {
  bridging(retained: this.kernelURL as NSURL)
}

func vz_linux_boot_loader_set_kernel_url(this: VZLinuxBootLoader, kernel_url: UnsafeRawPointer) {
  this.kernelURL = bridging(borrowed: kernel_url) as NSURL as URL
}

func vz_linux_boot_loader_initial_ramdisk_url(this: VZLinuxBootLoader) -> UInt {
  bridging(retainedAddress: this.initialRamdiskURL as NSURL?)
}

func vz_linux_boot_loader_set_initial_ramdisk_url(this: VZLinuxBootLoader, initial_ramdisk_url: UInt) {
  this.initialRamdiskURL = (bridging(borrowedAddress: initial_ramdisk_url) as NSURL?) as URL?
}

extension VZLinuxBootLoader {
  func vz_command_line() -> RustString {
    commandLine.intoRustString()
  }

  func vz_set_command_line(command_line: RustStr) {
    commandLine = command_line.toString()
  }
}
