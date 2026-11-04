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

import DiskImageKit
import Foundation

/// A Foundation object Rust lends for the call: Rust keeps its reference.
func bridging<T: AnyObject>(borrowed pointer: UnsafeRawPointer) -> T {
  Unmanaged<T>.fromOpaque(pointer).takeUnretainedValue()
}

/// A Foundation object for Rust, which takes over this reference.
func bridging(retained object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

/// An optional Foundation object for Rust, which takes over this reference,
/// by address: `0` is `nil`. (swift-bridge 0.1.59 can't pass an optional
/// pointer.)
func bridging(retainedAddress object: AnyObject?) -> UInt {
  UInt(bitPattern: object.map { bridging(retained: $0) })
}

extension DiskImageKitNil: @unchecked Sendable, Error {}

/// A Swift optional of an opaque type, as swift-bridge can return it.
func bridging<T>(_ value: T?) throws(DiskImageKitNil) -> T {
  guard let value else {
    throw .Nil
  }

  return value
}

extension DiskImageKitThrown: @unchecked Sendable, Error {}

/// Runs a throwing DiskImageKit call, passing what it throws to Rust as the
/// error type DiskImageKit declares, or as an `NSError` for any other.
func bridging<T>(_ body: () throws -> T) throws(DiskImageKitThrown) -> T {
  do {
    return try body()
  } catch let error as IncompatibleStackingError {
    throw .IncompatibleStackingError(
      reason: error.reason.intoRustString(),
      error_description: error.errorDescription?.intoRustString()
    )
  } catch let error as InvalidBlockCountError {
    throw .InvalidBlockCountError(error_description: error.errorDescription?.intoRustString())
  } catch let error as CorruptedImageError {
    throw .CorruptedImageError(
      underlying_error: bridging(retainedAddress: error.underlyingError as NSError),
      error_description: error.errorDescription?.intoRustString()
    )
  } catch let error as UnsupportedFormatError {
    throw .UnsupportedFormatError(
      underlying_error: bridging(retainedAddress: error.underlyingError as NSError),
      error_description: error.errorDescription?.intoRustString()
    )
  } catch {
    throw .Error(bridging(retainedAddress: error as NSError))
  }
}
