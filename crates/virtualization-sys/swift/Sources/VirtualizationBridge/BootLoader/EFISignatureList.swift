//===----------------------------------------------------------------------===//
// `VZEFISignatureList`, and `VZEFISignature`, which crosses as an object: its
// hash as `NSData`, its `SecCertificate`, or `NSNull` for a case newer than
// this bridge.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZEFISignature {
  init(_ object: AnyObject) {
    if let data = object as? NSData {
      self = .sha256(data as Data)
    } else if CFGetTypeID(object) == SecCertificateGetTypeID() {
      self = .x509Certificate(object as! SecCertificate)
    } else {
      fatalError("VZEFISignature.Unknown is no case Virtualization takes")
    }
  }

  var bridged: AnyObject {
    switch self {
    case .sha256(let data): data as NSData
    case .x509Certificate(let certificate): certificate
    @unknown default: NSNull()
    }
  }
}

func vz_efi_signature_list_init_contents_of(url: UnsafeRawPointer) throws(BridgedThrown) -> VZEFISignatureList {
  try bridging {
    try VZEFISignatureList(contentsOf: bridging(borrowed: url) as NSURL as URL)
  }
}

func vz_efi_signature_list_init_signatures(signatures: UnsafeRawPointer) -> VZEFISignatureList {
  VZEFISignatureList(signatures: (bridging(borrowedArray: signatures) as [AnyObject]).map(VZEFISignature.init))
}

/// An element of an array of them, which Rust holds.
func vz_efi_signature_list_bridge_from(object: UnsafeRawPointer) -> VZEFISignatureList {
  bridging(borrowed: object)
}

func vz_efi_signature_list_signatures(this: VZEFISignatureList) -> UnsafeMutableRawPointer {
  bridging(retained: this.signatures.map(\.bridged) as NSArray)
}
