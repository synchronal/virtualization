//===----------------------------------------------------------------------===//
// `SecCertificate`, which Virtualization's Secure Boot members take and
// return. It crosses as a pointer, as Foundation's objects do.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Foundation
import Security

/// `0` is `nil`: data that is no X.509 certificate.
func sec_certificate_create_with_data(data: UnsafeRawPointer) -> UInt {
  bridging(retainedAddress: SecCertificateCreateWithData(nil, bridging(borrowed: data) as NSData))
}

func sec_certificate_copy_data(certificate: UnsafeRawPointer) -> UnsafeMutableRawPointer {
  bridging(retained: SecCertificateCopyData(bridging(borrowed: certificate)) as NSData)
}
