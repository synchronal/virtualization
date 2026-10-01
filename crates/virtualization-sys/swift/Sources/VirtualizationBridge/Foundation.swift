//===----------------------------------------------------------------------===//
// Foundation, as far as Virtualization's API reaches it, and the glue for
// crossing swift-bridge.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Foundation

extension NSObject {
  func bridge_retained() -> Self {
    self
  }

  func ns_description() -> RustString {
    description.intoRustString()
  }
}

extension BridgedNil: @unchecked Sendable, Error {}

/// A Swift optional of an opaque type, as swift-bridge can return it.
func bridging<T>(_ value: T?) throws(BridgedNil) -> T {
  guard let value else {
    throw .Nil
  }

  return value
}

func ns_url_init_string(string: RustStr) throws(BridgedNil) -> NSURL {
  try bridging(NSURL(string: string.toString()))
}

extension NSURL {
  /// Through `URL`, whose `absoluteString` Swift declares non-optional.
  func ns_absolute_string() -> RustString {
    (self as URL).absoluteString.intoRustString()
  }
}

extension NSError {
  func ns_domain() -> RustString {
    domain.intoRustString()
  }

  func ns_code() -> Int {
    code
  }

  func ns_localized_description() -> RustString {
    localizedDescription.intoRustString()
  }

  func ns_localized_failure_reason() -> RustString? {
    localizedFailureReason?.intoRustString()
  }

  func ns_localized_recovery_suggestion() -> RustString? {
    localizedRecoverySuggestion?.intoRustString()
  }

  /// `userInfo`, sorted by key so an index means the same entry every time.
  private var bridgeUserInfo: [(key: String, value: Any)] {
    userInfo.sorted { $0.key < $1.key }
  }

  func bridge_user_info_count() -> UInt {
    UInt(userInfo.count)
  }

  func bridge_user_info_key_at(index: UInt) -> RustString {
    bridgeUserInfo[Int(index)].key.intoRustString()
  }

  func bridge_user_info_value_description_at(index: UInt) -> RustString {
    String(describing: bridgeUserInfo[Int(index)].value).intoRustString()
  }

  func bridge_underlying_errors_count() -> UInt {
    UInt(underlyingErrors.count)
  }

  func bridge_underlying_error_at(index: UInt) -> NSError {
    underlyingErrors[Int(index)] as NSError
  }
}

extension BridgedThrown: @unchecked Sendable, Error {}

/// Where `bridging` leaves a thrown error for `bridge_take_thrown_error`. Per
/// thread: Rust takes it on the thread that made the call, before any other.
private let thrownErrorKey = "VirtualizationBridge.thrownError"

/// Runs a throwing Virtualization call, keeping what it throws for Rust to
/// take.
func bridging<T>(_ body: () throws -> T) throws(BridgedThrown) -> T {
  do {
    return try body()
  } catch {
    Thread.current.threadDictionary[thrownErrorKey] = error as NSError
    throw .Thrown
  }
}

/// What the last `bridging` call on this thread threw. Rust calls this only
/// after one returned `Thrown`, so there is always an error to take.
func bridge_take_thrown_error() -> NSError {
  let error = Thread.current.threadDictionary[thrownErrorKey] as! NSError
  Thread.current.threadDictionary.removeObject(forKey: thrownErrorKey)
  return error
}
