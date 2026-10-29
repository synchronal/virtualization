//===----------------------------------------------------------------------===//
// `AAError`: its codes, and `AAErrorDomain`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import AccessoryAccess

func aa_error_domain() -> RustString {
  AAErrorDomain.intoRustString()
}

/// `AAError.Code(rawValue:)`: `Unknown` for a raw value with no case.
func aa_error_code_init_raw_value(raw_value: Int) -> BridgedAAErrorCode {
  BridgedAAErrorCode(AAError.Code(rawValue: raw_value)!)
}

func aa_error_code_raw_value(code: BridgedAAErrorCode) -> Int {
  AAError.Code(code).rawValue
}

// An Objective-C enum's `init(rawValue:)` accepts any value: these unwraps
// can't fail.

extension AAError.Code {
  init(_ code: BridgedAAErrorCode) {
    switch code {
    case .InternalError: self = .internalError
    case .AccessoryListenerAlreadyRegistered: self = .accessoryListenerAlreadyRegistered
    case .AccessoryNotAccessible: self = .accessoryNotAccessible
    case .InvalidAccessoryState: self = .invalidAccessoryState
    case .Unknown(let rawValue): self = AAError.Code(rawValue: rawValue)!
    }
  }
}

extension BridgedAAErrorCode {
  init(_ code: AAError.Code) {
    switch code {
    case .internalError: self = .InternalError
    case .accessoryListenerAlreadyRegistered: self = .AccessoryListenerAlreadyRegistered
    case .accessoryNotAccessible: self = .AccessoryNotAccessible
    case .invalidAccessoryState: self = .InvalidAccessoryState
    @unknown default: self = .Unknown(code.rawValue)
    }
  }
}
