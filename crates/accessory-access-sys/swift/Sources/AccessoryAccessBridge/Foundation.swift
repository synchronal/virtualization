//===----------------------------------------------------------------------===//
// The glue for crossing swift-bridge. Foundation's own objects cross as
// pointers, which Rust holds through objc2
// (`apple_bridge_foundation::objc`).
//
// Names here differ from the other bridges': all link into one binary, and
// swift-bridge's symbols are named for each bridge's types.
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

  /// For another bridge, while Rust holds this one.
  func bridge_borrowed() -> UnsafeRawPointer {
    UnsafeRawPointer(Unmanaged.passUnretained(self).toOpaque())
  }
}

/// A Foundation object Rust lends for the call: Rust keeps its reference.
func bridging<T: AnyObject>(borrowed pointer: UnsafeRawPointer) -> T {
  Unmanaged<T>.fromOpaque(pointer).takeUnretainedValue()
}

/// An optional Foundation object Rust lends for the call, by address: `0` is
/// `nil`. (swift-bridge 0.1.59 can't pass an optional pointer.)
func bridging<T: AnyObject>(borrowedAddress address: UInt) -> T? {
  UnsafeRawPointer(bitPattern: address).map { bridging(borrowed: $0) }
}

/// An array Rust lends for the call, as an `NSArray` of `T`s.
func bridging<T>(borrowedArray pointer: UnsafeRawPointer) -> [T] {
  (bridging(borrowed: pointer) as NSArray) as! [T]
}

/// A property list dictionary Rust lends for the call, as AccessoryAccess
/// takes one. `any Sendable` is a marker protocol, which a cast can't reach,
/// and has `Any`'s layout.
func bridging(sendableDictionary dictionary: NSDictionary) -> [String: any Sendable] {
  unsafeBitCast(dictionary as! [String: Any], to: [String: any Sendable].self)
}

/// A Foundation object for Rust, which takes over this reference.
func bridging(retained object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
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

extension AccessoryAccessNil: @unchecked Sendable, Error {}

/// A Swift optional of an opaque type, as swift-bridge can return it.
func bridging<T>(_ value: T?) throws(AccessoryAccessNil) -> T {
  guard let value else {
    throw .Nil
  }

  return value
}

/// A value Swift's concurrency checking can't see is safe to send: a
/// swift-bridge closure, which Rust declares `Send`.
struct Bridged<T>: @unchecked Sendable {
  let value: T
}
