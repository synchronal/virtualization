use crate::platform::ffi;

/// [`vmnet_mode_t`](https://developer.apple.com/documentation/vmnet/vmnet_mode_t),
/// which is [`operating_modes_t`](https://developer.apple.com/documentation/vmnet/operating_modes_t).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Mode {
  /// [`VMNET_HOST_MODE`](https://developer.apple.com/documentation/vmnet/operating_modes_t/vmnet_host_mode)
  Host,
  /// [`VMNET_SHARED_MODE`](https://developer.apple.com/documentation/vmnet/operating_modes_t/vmnet_shared_mode)
  Shared,
  /// [`VMNET_BRIDGED_MODE`](https://developer.apple.com/documentation/vmnet/operating_modes_t/vmnet_bridged_mode)
  Bridged,
  /// A mode newer than this crate, by its raw value.
  Unknown(u32),
}

impl From<Mode> for ffi::VmnetMode {
  fn from(mode: Mode) -> Self {
    match mode {
      Mode::Host => Self::Host,
      Mode::Shared => Self::Shared,
      Mode::Bridged => Self::Bridged,
      Mode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::VmnetMode> for Mode {
  fn from(mode: ffi::VmnetMode) -> Self {
    match mode {
      ffi::VmnetMode::Host => Self::Host,
      ffi::VmnetMode::Shared => Self::Shared,
      ffi::VmnetMode::Bridged => Self::Bridged,
      ffi::VmnetMode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_modes_both_ways() {
    for mode in [Mode::Host, Mode::Shared, Mode::Bridged, Mode::Unknown(2000)] {
      assert_eq!(Mode::from(ffi::VmnetMode::from(mode)), mode);
    }
  }
}
