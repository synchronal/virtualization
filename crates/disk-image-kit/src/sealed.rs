//! Closes the traits for protocols only DiskImageKit's own types can conform
//! to usefully: their requirements are DiskImageKit's, out of Rust's reach.
//! Each says how its conforming types cross the bridge.

use crate::{DiskImage, Error, StackedImage};

pub trait CreationConfiguration {
  /// `DiskImage(creating: self)`.
  fn bridge_init_creating(&self) -> Result<DiskImage, Error>;
}

pub trait StackableLayer {
  /// `disk_image.appending(self)`.
  fn bridge_appending(&self, disk_image: DiskImage) -> Result<StackedImage, Error>;
}
