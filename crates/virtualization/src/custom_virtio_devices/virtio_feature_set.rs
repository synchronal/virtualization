use std::fmt;

use crate::platform::ffi;

/// [`VZVirtioFeatureSet`](https://developer.apple.com/documentation/virtualization/vzvirtiofeatureset)
pub struct VirtioFeatureSet {
  pub(crate) inner: ffi::VZVirtioFeatureSet,
}

impl VirtioFeatureSet {
  /// [`subset0`](https://developer.apple.com/documentation/virtualization/vzvirtiofeatureset/subset0)
  pub fn subset0(&self) -> u32 {
    self.inner.vz_subset0()
  }

  /// [`subset0`](https://developer.apple.com/documentation/virtualization/vzvirtiofeatureset/subset0)
  pub fn set_subset0(&self, subset0: u32) {
    self.inner.vz_set_subset0(subset0);
  }

  /// [`subset1`](https://developer.apple.com/documentation/virtualization/vzvirtiofeatureset/subset1)
  pub fn subset1(&self) -> u32 {
    self.inner.vz_subset1()
  }

  /// [`subset1`](https://developer.apple.com/documentation/virtualization/vzvirtiofeatureset/subset1)
  pub fn set_subset1(&self, subset1: u32) {
    self.inner.vz_set_subset1(subset1);
  }
}

impl Clone for VirtioFeatureSet {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtioFeatureSet {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}
