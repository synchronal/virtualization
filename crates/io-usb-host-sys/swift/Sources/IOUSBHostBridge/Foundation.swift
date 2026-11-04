//===----------------------------------------------------------------------===//
// The glue for crossing swift-bridge. Foundation's own objects, and
// IOUSBHost's abstract `IOUSBHostObject`, cross as pointers, which Rust holds
// through objc2 (`apple_bridge_foundation::objc`).
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

  /// For `bridging(borrowed:)` as an abstract class, or for another bridge,
  /// while Rust holds this one.
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

/// A Foundation object for Rust, which takes over this reference.
func bridging(retained object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

/// An optional Foundation object for Rust, which takes over this reference,
/// by address: `0` is `nil`.
func bridging(retainedAddress object: AnyObject?) -> UInt {
  UInt(bitPattern: object.map { bridging(retained: $0) })
}

/// A descriptor IOUSBHost owns, as a copy of its `length` bytes.
func bridging(descriptor: UnsafeRawPointer, length: Int) -> UnsafeMutableRawPointer {
  bridging(retained: NSData(bytes: descriptor, length: length))
}

extension IOUSBHostNil: @unchecked Sendable, Error {}

/// A Swift optional of an opaque type, as swift-bridge can return it.
func bridging<T>(_ value: T?) throws(IOUSBHostNil) -> T {
  guard let value else {
    throw .Nil
  }

  return value
}

extension IOUSBHostThrown: @unchecked Sendable, Error {}

/// Runs a throwing IOUSBHost call, passing what it throws to Rust as an
/// `NSError`.
func bridging<T>(_ body: () throws -> T) throws(IOUSBHostThrown) -> T {
  do {
    return try body()
  } catch {
    throw .Error(bridging(retainedAddress: error as NSError))
  }
}

/// Runs a throwing IOUSBHost call, passing what it throws to Rust as an
/// `NSError` by address: `0` for nothing. (Not `bridging`: a trailing closure
/// couldn't tell the two apart.)
func thrownAddress(of body: () throws -> Void) -> UInt {
  do {
    try body()
    return 0
  } catch {
    return bridging(retainedAddress: error as NSError)
  }
}

/// Throws what a call Swift doesn't import as throwing left in its `error`
/// out parameter, if anything.
func bridging(error: NSError?) throws(IOUSBHostThrown) {
  if let error {
    throw .Error(bridging(retainedAddress: error))
  }
}
