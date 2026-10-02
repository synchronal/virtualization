//===----------------------------------------------------------------------===//
// `OpenConfiguration.Mode`. `OpenConfiguration` itself is plain data, `url`
// and `mode`, which Rust holds and `dik_disk_image_init_opening` takes.
//===----------------------------------------------------------------------===//

import DiskImageKit

// `Mode` has no raw values: a case newer than this crate has no value to
// carry back, so passing `Unknown` to DiskImageKit stops the process.

extension OpenConfiguration.Mode {
  init(_ mode: BridgedOpenConfigurationMode) {
    switch mode {
    case .Automatic: self = .automatic
    case .ReadOnly: self = .readOnly
    case .ReadWrite: self = .readWrite
    case .Unknown: fatalError("an OpenConfiguration.Mode newer than this crate can't be passed back")
    }
  }
}

extension BridgedOpenConfigurationMode {
  init(_ mode: OpenConfiguration.Mode) {
    switch mode {
    case .automatic: self = .Automatic
    case .readOnly: self = .ReadOnly
    case .readWrite: self = .ReadWrite
    @unknown default: self = .Unknown
    }
  }
}
