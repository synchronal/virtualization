//! What a call into vmnet failed with.

use std::fmt;

use crate::platform::ffi;

/// What a call into vmnet failed with: a [`vmnet_return_t`] other than
/// `VMNET_SUCCESS`, or vmnet's absence.
///
/// [`vmnet_return_t`]: https://developer.apple.com/documentation/vmnet/vmnet_return_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
  /// vmnet exists only on macOS.
  Unsupported,
  /// What vmnet returned for a call that failed.
  Return(Return),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "vmnet is only available on macOS"),
      Self::Return(value) => write!(formatter, "vmnet returned {value:?}"),
    }
  }
}

impl std::error::Error for Error {}

/// [`vmnet_return_t`](https://developer.apple.com/documentation/vmnet/vmnet_return_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Return {
  /// [`VMNET_SUCCESS`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_success)
  Success,
  /// [`VMNET_FAILURE`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_failure)
  Failure,
  /// [`VMNET_MEM_FAILURE`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_mem_failure)
  MemFailure,
  /// [`VMNET_INVALID_ARGUMENT`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_invalid_argument)
  InvalidArgument,
  /// [`VMNET_SETUP_INCOMPLETE`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_setup_incomplete)
  SetupIncomplete,
  /// [`VMNET_INVALID_ACCESS`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_invalid_access)
  InvalidAccess,
  /// [`VMNET_PACKET_TOO_BIG`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_packet_too_big)
  PacketTooBig,
  /// [`VMNET_BUFFER_EXHAUSTED`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_buffer_exhausted)
  BufferExhausted,
  /// [`VMNET_TOO_MANY_PACKETS`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_too_many_packets)
  TooManyPackets,
  /// [`VMNET_SHARING_SERVICE_BUSY`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_sharing_service_busy)
  SharingServiceBusy,
  /// [`VMNET_NOT_AUTHORIZED`](https://developer.apple.com/documentation/vmnet/vmnet_return_t/vmnet_not_authorized)
  NotAuthorized,
  /// A value newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<ffi::VmnetReturn> for Return {
  fn from(value: ffi::VmnetReturn) -> Self {
    match value {
      ffi::VmnetReturn::Success => Self::Success,
      ffi::VmnetReturn::Failure => Self::Failure,
      ffi::VmnetReturn::MemFailure => Self::MemFailure,
      ffi::VmnetReturn::InvalidArgument => Self::InvalidArgument,
      ffi::VmnetReturn::SetupIncomplete => Self::SetupIncomplete,
      ffi::VmnetReturn::InvalidAccess => Self::InvalidAccess,
      ffi::VmnetReturn::PacketTooBig => Self::PacketTooBig,
      ffi::VmnetReturn::BufferExhausted => Self::BufferExhausted,
      ffi::VmnetReturn::TooManyPackets => Self::TooManyPackets,
      ffi::VmnetReturn::SharingServiceBusy => Self::SharingServiceBusy,
      ffi::VmnetReturn::NotAuthorized => Self::NotAuthorized,
      ffi::VmnetReturn::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<Return> for ffi::VmnetReturn {
  fn from(value: Return) -> Self {
    match value {
      Return::Success => Self::Success,
      Return::Failure => Self::Failure,
      Return::MemFailure => Self::MemFailure,
      Return::InvalidArgument => Self::InvalidArgument,
      Return::SetupIncomplete => Self::SetupIncomplete,
      Return::InvalidAccess => Self::InvalidAccess,
      Return::PacketTooBig => Self::PacketTooBig,
      Return::BufferExhausted => Self::BufferExhausted,
      Return::TooManyPackets => Self::TooManyPackets,
      Return::SharingServiceBusy => Self::SharingServiceBusy,
      Return::NotAuthorized => Self::NotAuthorized,
      Return::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// `Success` is `Ok`; anything else, an [`Error::Return`].
pub(crate) fn returned(value: ffi::VmnetReturn) -> Result<(), Error> {
  match Return::from(value) {
    Return::Success => Ok(()),
    value => Err(Error::Return(value)),
  }
}

/// A status vmnet set when it returned `NULL`.
impl From<ffi::VmnetReturn> for Error {
  fn from(value: ffi::VmnetReturn) -> Self {
    Self::Return(value.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_returns_both_ways() {
    for value in [
      Return::Success,
      Return::Failure,
      Return::MemFailure,
      Return::InvalidArgument,
      Return::SetupIncomplete,
      Return::InvalidAccess,
      Return::PacketTooBig,
      Return::BufferExhausted,
      Return::TooManyPackets,
      Return::SharingServiceBusy,
      Return::NotAuthorized,
      Return::Unknown(2000),
    ] {
      assert_eq!(Return::from(ffi::VmnetReturn::from(value)), value);
    }
  }

  #[test]
  fn success_is_ok() {
    assert_eq!(returned(ffi::VmnetReturn::Success), Ok(()));
  }

  #[test]
  fn anything_else_is_its_error() {
    assert_eq!(
      returned(ffi::VmnetReturn::NotAuthorized),
      Err(Error::Return(Return::NotAuthorized))
    );
    assert_eq!(
      returned(ffi::VmnetReturn::Unknown(2000)),
      Err(Error::Return(Return::Unknown(2000)))
    );
  }

  #[test]
  fn displays_what_vmnet_returned() {
    assert_eq!(
      Error::Return(Return::InvalidArgument).to_string(),
      "vmnet returned InvalidArgument"
    );
  }
}
