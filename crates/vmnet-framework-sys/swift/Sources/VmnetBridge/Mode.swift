//===----------------------------------------------------------------------===//
// `vmnet_mode_t`, which is `operating_modes_t`.
//===----------------------------------------------------------------------===//

import vmnet

// A C enum's `init(rawValue:)` accepts any value: this unwrap can't fail.

extension vmnet_mode_t {
  init(_ mode: BridgedVmnetMode) {
    switch mode {
    case .Host: self = .VMNET_HOST_MODE
    case .Shared: self = .VMNET_SHARED_MODE
    case .Bridged: self = .VMNET_BRIDGED_MODE
    case .Unknown(let rawValue): self = vmnet_mode_t(rawValue: rawValue)!
    }
  }
}
