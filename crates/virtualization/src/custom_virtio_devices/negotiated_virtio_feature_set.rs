use std::fmt;

use crate::platform::ffi;

/// [`VZNegotiatedVirtioFeatureSet`](https://developer.apple.com/documentation/virtualization/vznegotiatedvirtiofeatureset)
pub struct NegotiatedVirtioFeatureSet {
  pub(crate) inner: ffi::VZNegotiatedVirtioFeatureSet,
}

impl NegotiatedVirtioFeatureSet {
  /// [`subset0`](https://developer.apple.com/documentation/virtualization/vznegotiatedvirtiofeatureset/subset0)
  pub fn subset0(&self) -> u32 {
    self.inner.vz_subset0()
  }

  /// [`subset1`](https://developer.apple.com/documentation/virtualization/vznegotiatedvirtiofeatureset/subset1)
  pub fn subset1(&self) -> u32 {
    self.inner.vz_subset1()
  }
}

impl Clone for NegotiatedVirtioFeatureSet {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NegotiatedVirtioFeatureSet {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
