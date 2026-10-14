//===----------------------------------------------------------------------===//
// The glue for crossing swift-bridge: vmnet's references, held as the
// objects they are, and the C structs Rust passes as integers.
//
// Names here differ from VirtualizationBridge's and DiskImageKitBridge's: they
// link into one binary, and swift-bridge's symbols are named for each bridge's
// types.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import vmnet

/// A reference vmnet returned retained, for Swift to release with its last
/// use. Swift doesn't manage vmnet's references; they are Core Foundation
/// objects, which `CFRelease` releases.
func bridging(retained reference: OpaquePointer) -> AnyObject {
  Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(reference)).takeRetainedValue()
}

/// An object as the reference vmnet takes, for as long as the object lives.
func bridging(reference object: AnyObject) -> OpaquePointer {
  OpaquePointer(Unmanaged.passUnretained(object).toOpaque())
}

/// An `in_addr` Rust passes as its address in host byte order.
func bridging(inAddr address: UInt32) -> in_addr {
  in_addr(s_addr: address.bigEndian)
}

/// An `in_addr` for Rust, as its address in host byte order.
func bridging(inAddr address: in_addr) -> UInt32 {
  UInt32(bigEndian: address.s_addr)
}

/// An `in6_addr` Rust passes as its first and last eight octets, each a
/// big-endian `u64`. (swift-bridge has no fixed-size arrays.)
func bridging(in6AddrHigh high: UInt64, low: UInt64) -> in6_addr {
  var address = in6_addr()

  withUnsafeMutableBytes(of: &address) { octets in
    withUnsafeBytes(of: high.bigEndian) { octets.baseAddress!.copyMemory(from: $0.baseAddress!, byteCount: 8) }
    withUnsafeBytes(of: low.bigEndian) { (octets.baseAddress! + 8).copyMemory(from: $0.baseAddress!, byteCount: 8) }
  }

  return address
}

/// An `in6_addr` for Rust, as its first and last eight octets, each a
/// big-endian `u64`.
func bridging(in6Addr address: in6_addr) -> (high: UInt64, low: UInt64) {
  withUnsafeBytes(of: address) { octets in
    (
      UInt64(bigEndian: octets.loadUnaligned(fromByteOffset: 0, as: UInt64.self)),
      UInt64(bigEndian: octets.loadUnaligned(fromByteOffset: 8, as: UInt64.self))
    )
  }
}

/// An `ether_addr_t` Rust passes as its six octets in a `u64`, first octet
/// lowest.
func bridging(ethernetAddress packed: UInt64) -> ether_addr_t {
  func octet(_ index: UInt64) -> UInt8 {
    UInt8(truncatingIfNeeded: packed >> (8 * index))
  }

  return ether_addr_t(octet: (octet(0), octet(1), octet(2), octet(3), octet(4), octet(5)))
}
