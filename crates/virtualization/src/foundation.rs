//! Foundation's types across the bridge: `URL`, `UUID`, thrown errors,
//! `OperatingSystemVersion`, and the memory pages a shared memory region maps.

use std::fmt;

use url::Url;
use uuid::Uuid;

use crate::Error;
use crate::platform::{self, ffi};

/// `NSURL(string:)`.
pub(crate) fn ns_url(url: &Url) -> Result<ffi::Retained<ffi::NSURL>, Error> {
  ffi::ns_url_init_string(url.as_str()).ok_or_else(|| Error::UnrepresentableUrl(url.clone()))
}

/// `absoluteString`, parsed.
pub(crate) fn url(ns_url: &ffi::NSURL) -> Result<Url, Error> {
  Url::parse(&ffi::ns_url_absolute_string(ns_url)).map_err(Error::UnparseableUrl)
}

/// `NSUUID(uuidString:)`.
pub(crate) fn ns_uuid(uuid: &Uuid) -> ffi::Retained<ffi::NSUUID> {
  ffi::ns_uuid_init_uuid_string(&uuid.to_string()).expect("Foundation reads a UUID's string")
}

/// `uuidString`, parsed.
pub(crate) fn uuid(ns_uuid: &ffi::NSUUID) -> Uuid {
  Uuid::parse_str(&ffi::ns_uuid_uuid_string(ns_uuid)).expect("Foundation's uuidString is a UUID")
}

/// What a throwing call threw.
pub(crate) fn thrown(_thrown: ffi::BridgedThrown) -> Error {
  error(&ffi::bridge_take_thrown_error())
}

/// An `NSError` Virtualization failed with.
pub(crate) fn error(error: &ffi::NSError) -> Error {
  Error::NSError(Box::new(error.into()))
}

/// What a completion handler was called with: an error, or `nil` for success.
pub(crate) fn completed(error: Option<ffi::Retained<ffi::NSError>>) -> Result<(), Error> {
  error.map_or(Ok(()), |error| Err(self::error(&error)))
}

/// [`OperatingSystemVersion`](https://developer.apple.com/documentation/foundation/operatingsystemversion)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperatingSystemVersion {
  pub major_version: isize,
  pub minor_version: isize,
  pub patch_version: isize,
}

impl From<ffi::BridgedOperatingSystemVersion> for OperatingSystemVersion {
  fn from(version: ffi::BridgedOperatingSystemVersion) -> Self {
    Self {
      major_version: version.major_version,
      minor_version: version.minor_version,
      patch_version: version.patch_version,
    }
  }
}

/// [`NSPageSize()`](https://developer.apple.com/documentation/foundation/nspagesize()):
/// what [`VirtioSharedMemoryRegion`](crate::VirtioSharedMemoryRegion) aligns
/// offsets and sizes to.
pub fn ns_page_size() -> Result<usize, Error> {
  platform::supported()?;

  Ok(ffi::ns_page_size())
}

/// Host memory from
/// [`NSAllocateMemoryPages(_:)`](https://developer.apple.com/documentation/foundation/nsallocatememorypages(_:)),
/// page-aligned, for
/// [`VirtioSharedMemoryRegion::map_memory`](crate::VirtioSharedMemoryRegion::map_memory).
/// `NSDeallocateMemoryPages(_:_:)` frees it once nothing holds it, a region it
/// is mapped into included.
///
/// Not Foundation's type: Rust's, so that the guest never sees mapped memory
/// freed. A shared reference: [`Clone`] is the same pages. A guest may change
/// mapped pages at any moment, so they're read and written only by copying.
#[derive(Clone)]
pub struct MemoryPages {
  pub(crate) inner: ffi::MemoryPages,
}

impl MemoryPages {
  /// [`NSAllocateMemoryPages(_:)`](https://developer.apple.com/documentation/foundation/nsallocatememorypages(_:)):
  /// `bytes`, zeroed.
  pub fn new(bytes: usize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::ns_allocate_memory_pages(bytes),
    })
  }

  /// How many bytes were asked for.
  pub fn length(&self) -> usize {
    self.inner.length()
  }

  /// Copies the bytes from `offset` into `buffer`, as many as it holds.
  ///
  /// # Panics
  ///
  /// When they run past [`length`](Self::length).
  pub fn read(&self, offset: usize, buffer: &mut [u8]) {
    self.inner.read(offset, buffer);
  }

  /// Copies `bytes` in from `offset`.
  ///
  /// # Panics
  ///
  /// When they run past [`length`](Self::length).
  pub fn write(&self, offset: usize, bytes: &[u8]) {
    self.inner.write(offset, bytes);
  }
}

impl fmt::Debug for MemoryPages {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("MemoryPages")
      .field("length", &self.length())
      .finish()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn memory_pages_are_unsupported_off_macos() {
    assert!(matches!(ns_page_size(), Err(Error::Unsupported)));
    assert!(matches!(MemoryPages::new(0), Err(Error::Unsupported)));
  }

  #[cfg(target_os = "macos")]
  #[test]
  fn memory_pages_start_zeroed() {
    let memory = MemoryPages::new(3).unwrap();
    let mut bytes = [0xff; 3];

    memory.read(0, &mut bytes);

    assert_eq!(bytes, [0; 3]);
  }

  #[cfg(target_os = "macos")]
  #[test]
  fn memory_pages_read_what_was_written_through_any_clone() {
    let memory = MemoryPages::new(8).unwrap();
    let mut bytes = [0; 3];

    memory.clone().write(4, &[1, 2, 3]);
    memory.read(4, &mut bytes);

    assert_eq!(bytes, [1, 2, 3]);
  }

  #[cfg(target_os = "macos")]
  #[test]
  #[should_panic]
  fn memory_pages_refuse_writes_past_their_length() {
    MemoryPages::new(2).unwrap().write(1, &[1, 2]);
  }

  #[cfg(target_os = "macos")]
  #[test]
  #[should_panic]
  fn memory_pages_refuse_reads_past_their_length() {
    MemoryPages::new(2).unwrap().read(1, &mut [0; 2]);
  }

  #[test]
  fn an_operating_system_version_keeps_each_part_from_virtualization_sys() {
    assert_eq!(
      OperatingSystemVersion::from(ffi::BridgedOperatingSystemVersion {
        major_version: 27,
        minor_version: 1,
        patch_version: 2,
      }),
      OperatingSystemVersion {
        major_version: 27,
        minor_version: 1,
        patch_version: 2,
      }
    );
  }
}
