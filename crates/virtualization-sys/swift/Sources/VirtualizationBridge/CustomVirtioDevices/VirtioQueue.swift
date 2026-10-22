//===----------------------------------------------------------------------===//
// `VZVirtioQueue`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

extension VZVirtioQueue {
  func vz_next_element() throws(BridgedNil) -> VZVirtioQueueElement {
    try bridging(nextElement())
  }

  func vz_queue_index() -> UInt16 {
    queueIndex
  }

  func vz_queue_size() -> UInt16 {
    queueSize
  }
}
