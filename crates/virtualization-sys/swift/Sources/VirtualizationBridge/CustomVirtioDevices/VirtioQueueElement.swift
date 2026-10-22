//===----------------------------------------------------------------------===//
// `VZVirtioQueueElement`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtio_queue_element_read_buffers(this: VZVirtioQueueElement) -> UnsafeMutableRawPointer {
  bridging(retained: this.readBuffers() as NSArray)
}

/// The `NSData` it copied, by address. (swift-bridge 0.1.59 can't parse a
/// `Result` of a pointer.)
func vz_virtio_queue_element_peek_into_read_buffers(this: VZVirtioQueueElement, exact_length: Int)
  throws(BridgedThrown) -> UInt
{
  try bridging {
    try bridging(retainedAddress: this.peekIntoReadBuffers(withExactLength: exact_length) as NSData)
  }
}

/// The `NSData` it copied, by address. (swift-bridge 0.1.59 can't parse a
/// `Result` of a pointer.)
func vz_virtio_queue_element_read_bytes(this: VZVirtioQueueElement, exact_length: Int) throws(BridgedThrown) -> UInt {
  try bridging {
    try bridging(retainedAddress: this.readBytes(withExactLength: exact_length) as NSData)
  }
}

/// `buffer` is `exact_length` bytes Rust lends for the call.
func vz_virtio_queue_element_read_bytes_into_buffer(
  this: VZVirtioQueueElement,
  buffer: UnsafeMutableRawPointer,
  exact_length: Int
) throws(BridgedThrown) {
  try bridging {
    try this.readBytes(intoBuffer: buffer, exactLength: exact_length)
  }
}

/// `data` is an `NSData`, lent for the call.
func vz_virtio_queue_element_write(this: VZVirtioQueueElement, data: UnsafeRawPointer) throws(BridgedThrown) {
  try bridging {
    try this.write(bridging(borrowed: data) as NSData as Data)
  }
}

/// `buffer` is `exact_length` bytes Rust lends for the call, which
/// Virtualization only reads.
func vz_virtio_queue_element_write_buffer(
  this: VZVirtioQueueElement,
  buffer: UnsafeRawPointer,
  exact_length: Int
) throws(BridgedThrown) {
  try bridging {
    try this.writeBuffer(UnsafeMutableRawPointer(mutating: buffer), exactLength: exact_length)
  }
}

extension VZVirtioQueueElement {
  func vz_read_buffers_byte_count() -> Int {
    readBuffersByteCount
  }

  func vz_read_buffers_available_byte_count() -> Int {
    readBuffersAvailableByteCount
  }

  func vz_write_buffers_byte_count() -> Int {
    writeBuffersByteCount
  }

  func vz_write_buffers_available_byte_count() -> Int {
    writeBuffersAvailableByteCount
  }

  func vz_written_byte_count() -> Int {
    writtenByteCount
  }

  func vz_return_to_queue() {
    returnToQueue()
  }
}
