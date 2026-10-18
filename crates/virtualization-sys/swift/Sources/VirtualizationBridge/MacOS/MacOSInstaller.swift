//===----------------------------------------------------------------------===//
// `VZMacOSInstaller`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_macos_installer_init_restoring_from_image_at(
  virtual_machine: VZVirtualMachine,
  restore_image_file_url: UnsafeRawPointer
) -> VZMacOSInstaller {
  VZMacOSInstaller(
    virtualMachine: virtual_machine,
    restoringFromImageAt: bridging(borrowed: restore_image_file_url) as NSURL as URL)
}

func vz_macos_installer_install(this: VZMacOSInstaller, completion_handler: @escaping (UInt) -> Void) {
  let completionHandler = Bridged(value: completion_handler)

  this.install { completionHandler.value(bridging(retainedError: $0)) }
}

/// `progress`, which the installer holds, lent by address.
func vz_macos_installer_progress(this: VZMacOSInstaller) -> UnsafeRawPointer {
  UnsafeRawPointer(Unmanaged.passUnretained(this.progress).toOpaque())
}

func vz_macos_installer_restore_image_url(this: VZMacOSInstaller) -> UnsafeMutableRawPointer {
  bridging(retained: this.restoreImageURL as NSURL)
}

extension VZMacOSInstaller {
  func vz_virtual_machine() -> VZVirtualMachine {
    virtualMachine
  }
}
