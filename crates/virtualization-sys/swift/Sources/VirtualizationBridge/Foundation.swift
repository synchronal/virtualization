//===----------------------------------------------------------------------===//
// The glue for crossing swift-bridge. Foundation's own objects cross as
// pointers, which Rust holds through objc2.
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

  /// For `bridging(borrowed:)` as an abstract class, while Rust holds this
  /// one.
  func bridge_borrowed() -> UnsafeRawPointer {
    UnsafeRawPointer(Unmanaged.passUnretained(self).toOpaque())
  }
}

/// A Foundation object Rust lends for the call: Rust keeps its reference.
func bridging<T: AnyObject>(borrowed pointer: UnsafeRawPointer) -> T {
  Unmanaged<T>.fromOpaque(pointer).takeUnretainedValue()
}

/// An array Rust lends for the call, as an `NSArray` of `T`s.
func bridging<T>(borrowedArray pointer: UnsafeRawPointer) -> [T] {
  (bridging(borrowed: pointer) as NSArray) as! [T]
}

/// A Foundation object for Rust, which takes over this reference.
func bridging(retained object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

/// An optional Foundation object Rust lends for the call, by address: `0` is
/// `nil`. (swift-bridge 0.1.59 can't pass an optional pointer.)
func bridging<T: AnyObject>(borrowedAddress address: UInt) -> T? {
  UnsafeRawPointer(bitPattern: address).map { bridging(borrowed: $0) }
}

/// An optional Foundation object for Rust, which takes over this reference,
/// by address: `0` is `nil`.
func bridging(retainedAddress object: AnyObject?) -> UInt {
  UInt(bitPattern: object.map { bridging(retained: $0) })
}

/// What a completion handler failed with, for Rust, by address: `0` is
/// success.
func bridging(retainedError error: (any Error)?) -> UInt {
  bridging(retainedAddress: error.map { $0 as NSError })
}

/// What a completion handler's `Result` failed with, for Rust, by address: `0`
/// is success.
func bridging(retainedError result: Result<Void, any Error>) -> UInt {
  guard case .failure(let error) = result else {
    return 0
  }

  return bridging(retainedError: error)
}

extension BridgedNil: @unchecked Sendable, Error {}

/// A Swift optional of an opaque type, as swift-bridge can return it.
func bridging<T>(_ value: T?) throws(BridgedNil) -> T {
  guard let value else {
    throw .Nil
  }

  return value
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

/// What the last `bridging` call on this thread threw, as an `NSError` Rust
/// takes over. Rust calls this only after one returned `Thrown`, so there is
/// always an error to take.
func bridge_take_thrown_error() -> UnsafeMutableRawPointer {
  let error = Thread.current.threadDictionary[thrownErrorKey] as! NSError
  Thread.current.threadDictionary.removeObject(forKey: thrownErrorKey)
  return bridging(retained: error)
}
