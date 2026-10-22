use std::fmt;

use crate::platform::ffi;
use crate::{Error, foundation};

/// [`VZVirtioQueueElement`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement)
pub struct VirtioQueueElement {
  pub(crate) inner: ffi::VZVirtioQueueElement,
}

impl VirtioQueueElement {
  /// [`readBuffersByteCount`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/readbuffersbytecount)
  pub fn read_buffers_byte_count(&self) -> isize {
    self.inner.vz_read_buffers_byte_count()
  }

  /// [`readBuffersAvailableByteCount`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/readbuffersavailablebytecount)
  pub fn read_buffers_available_byte_count(&self) -> isize {
    self.inner.vz_read_buffers_available_byte_count()
  }

  /// [`writeBuffersByteCount`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/writebuffersbytecount)
  pub fn write_buffers_byte_count(&self) -> isize {
    self.inner.vz_write_buffers_byte_count()
  }

  /// [`writeBuffersAvailableByteCount`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/writebuffersavailablebytecount)
  pub fn write_buffers_available_byte_count(&self) -> isize {
    self.inner.vz_write_buffers_available_byte_count()
  }

  /// [`writtenByteCount`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/writtenbytecount)
  pub fn written_byte_count(&self) -> isize {
    self.inner.vz_written_byte_count()
  }

  /// [`readBuffers()`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/readbuffers()):
  /// each `Data` is its bytes, copied out of the guest's memory.
  pub fn read_buffers(&self) -> Vec<Vec<u8>> {
    self.inner.vz_read_buffers()
  }

  /// [`peekIntoReadBuffers(withExactLength:)`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/peekintoreadbuffers(withexactlength:)):
  /// `Data` is its bytes.
  pub fn peek_into_read_buffers(&self, exact_length: isize) -> Result<Vec<u8>, Error> {
    self
      .inner
      .vz_peek_into_read_buffers(exact_length)
      .map_err(foundation::thrown)
  }

  /// [`readBytes(withExactLength:)`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/readbytes(withexactlength:)):
  /// `Data` is its bytes.
  pub fn read_bytes(&self, exact_length: isize) -> Result<Vec<u8>, Error> {
    self
      .inner
      .vz_read_bytes(exact_length)
      .map_err(foundation::thrown)
  }

  /// [`readBytes(intoBuffer:exactLength:)`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/readbytes(intobuffer:exactlength:)):
  /// the exact length is `buffer`'s.
  pub fn read_bytes_into_buffer(&self, buffer: &mut [u8]) -> Result<(), Error> {
    self
      .inner
      .vz_read_bytes_into_buffer(buffer)
      .map_err(foundation::thrown)
  }

  /// [`write(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/write(_:)):
  /// `Data` is its bytes.
  pub fn write(&self, data: &[u8]) -> Result<(), Error> {
    self.inner.vz_write(data).map_err(foundation::thrown)
  }

  /// [`writeBuffer(_:exactLength:)`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/writebuffer(_:exactlength:)):
  /// the exact length is `buffer`'s.
  pub fn write_buffer(&self, buffer: &[u8]) -> Result<(), Error> {
    self
      .inner
      .vz_write_buffer(buffer)
      .map_err(foundation::thrown)
  }

  /// [`returnToQueue()`](https://developer.apple.com/documentation/virtualization/vzvirtioqueueelement/returntoqueue()):
  /// an element freed without it raises an exception, which aborts the
  /// process.
  pub fn return_to_queue(&self) {
    self.inner.vz_return_to_queue();
  }
}

impl Clone for VirtioQueueElement {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioQueueElement {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
