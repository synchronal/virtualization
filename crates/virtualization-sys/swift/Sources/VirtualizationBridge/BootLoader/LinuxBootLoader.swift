//===----------------------------------------------------------------------===//
// `VZLinuxBootLoader`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_linux_boot_loader_init(kernel_url: NSURL) -> VZLinuxBootLoader {
  VZLinuxBootLoader(kernelURL: kernel_url as URL)
}

extension VZLinuxBootLoader {
  func vz_kernel_url() -> NSURL {
    kernelURL as NSURL
  }

  func vz_set_kernel_url(kernel_url: NSURL) {
    kernelURL = kernel_url as URL
  }

  func vz_command_line() -> RustString {
    commandLine.intoRustString()
  }

  func vz_set_command_line(command_line: RustStr) {
    commandLine = command_line.toString()
  }

  func vz_initial_ramdisk_url() throws(BridgedNil) -> NSURL {
    try bridging(initialRamdiskURL as NSURL?)
  }

  func vz_set_initial_ramdisk_url(initial_ramdisk_url: NSURL) {
    initialRamdiskURL = initial_ramdisk_url as URL
  }

  func bridge_set_initial_ramdisk_url_nil() {
    initialRamdiskURL = nil
  }
}
