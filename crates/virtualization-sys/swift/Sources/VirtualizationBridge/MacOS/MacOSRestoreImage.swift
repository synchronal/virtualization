//===----------------------------------------------------------------------===//
// `VZMacOSRestoreImage`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

/// `object`, a `VZMacOSRestoreImage` Rust holds, as one.
func vz_macos_restore_image_bridge_from(object: UnsafeRawPointer) -> VZMacOSRestoreImage {
  bridging(borrowed: object)
}

/// `completion_handler` takes the restore image, which Rust takes over, then
/// what it failed with, each by address: one is `0`.
func vz_macos_restore_image_load(file_url: UnsafeRawPointer, completion_handler: @escaping (UInt, UInt) -> Void) {
  let completionHandler = Bridged(value: completion_handler)

  VZMacOSRestoreImage.load(from: bridging(borrowed: file_url) as NSURL as URL) { result in
    switch result {
    case .success(let restoreImage): completionHandler.value(bridging(retainedAddress: restoreImage), 0)
    case .failure(let error): completionHandler.value(0, bridging(retainedError: error))
    }
  }
}

/// As `vz_macos_restore_image_load`'s.
func vz_macos_restore_image_fetch_latest_supported(completion_handler: @escaping (UInt, UInt) -> Void) {
  let completionHandler = Bridged(value: completion_handler)

  VZMacOSRestoreImage.fetchLatestSupported { result in
    switch result {
    case .success(let restoreImage): completionHandler.value(bridging(retainedAddress: restoreImage), 0)
    case .failure(let error): completionHandler.value(0, bridging(retainedError: error))
    }
  }
}

func vz_macos_restore_image_url(this: VZMacOSRestoreImage) -> UnsafeMutableRawPointer {
  bridging(retained: this.url as NSURL)
}

extension VZMacOSRestoreImage {
  func vz_is_supported() -> Bool {
    isSupported
  }

  func vz_build_version() -> RustString {
    buildVersion.intoRustString()
  }

  func vz_operating_system_version() -> BridgedOperatingSystemVersion {
    BridgedOperatingSystemVersion(operatingSystemVersion)
  }

  func vz_most_featureful_supported_configuration() throws(BridgedNil) -> VZMacOSConfigurationRequirements {
    try bridging(mostFeaturefulSupportedConfiguration)
  }
}
