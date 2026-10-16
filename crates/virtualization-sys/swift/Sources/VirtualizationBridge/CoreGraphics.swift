//===----------------------------------------------------------------------===//
// `CGSize`, which Virtualization's displays take and return. It crosses as
// the shared struct `BridgedCGSize`.
//===----------------------------------------------------------------------===//

import CoreGraphics

extension BridgedCGSize {
  init(_ size: CGSize) {
    self.init(width: Double(size.width), height: Double(size.height))
  }
}

extension CGSize {
  init(_ size: BridgedCGSize) {
    self.init(width: size.width, height: size.height)
  }
}
