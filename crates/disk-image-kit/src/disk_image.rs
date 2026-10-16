use std::fmt;
use std::ops::Deref;

use url::Url;
use uuid::Uuid;

use crate::platform::{self, ffi};
use crate::{
  CreationConfiguration, Error, OpenConfigurationMode, OpenConfigurationProtocol, StackableLayer, foundation,
};

/// [`DiskImage`](https://developer.apple.com/documentation/diskimagekit/diskimage)
pub struct DiskImage {
  pub(crate) inner: ffi::DiskImage,
}

impl DiskImage {
  /// [`init(creating:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/init(creating:))
  pub fn new(configuration: &impl CreationConfiguration) -> Result<Self, Error> {
    platform::supported()?;

    configuration.bridge_init_creating()
  }

  /// [`init(opening:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/init(opening:))
  pub fn with_opening(configuration: &impl OpenConfigurationProtocol) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_disk_image_init_opening(&*foundation::ns_url(configuration.url())?, configuration.mode().into())?,
    })
  }

  /// [`url`](https://developer.apple.com/documentation/diskimagekit/diskimage/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.dik_url())
  }

  /// [`blockCount`](https://developer.apple.com/documentation/diskimagekit/diskimage/blockcount)
  pub fn block_count(&self) -> isize {
    self.inner.dik_block_count()
  }

  /// [`blockSize`](https://developer.apple.com/documentation/diskimagekit/diskimage/blocksize-swift.property)
  pub fn block_size(&self) -> DiskImageBlockSize {
    self.inner.dik_block_size().into()
  }

  /// [`size`](https://developer.apple.com/documentation/diskimagekit/diskimage/size)
  pub fn size(&self) -> isize {
    self.inner.dik_size()
  }

  /// [`layerType`](https://developer.apple.com/documentation/diskimagekit/diskimage/layertype-swift.property)
  pub fn layer_type(&self) -> Option<DiskImageLayerType> {
    self
      .inner
      .dik_layer_type()
      .ok()
      .map(|inner| DiskImageLayerType { inner })
  }

  /// [`layerUUID`](https://developer.apple.com/documentation/diskimagekit/diskimage/layeruuid)
  pub fn layer_uuid(&self) -> Option<Uuid> {
    self
      .inner
      .dik_layer_uuid()
      .map(|uuid| foundation::uuid(&uuid))
  }

  /// [`parentUUID`](https://developer.apple.com/documentation/diskimagekit/diskimage/parentuuid)
  pub fn parent_uuid(&self) -> Option<Uuid> {
    self
      .inner
      .dik_parent_uuid()
      .map(|uuid| foundation::uuid(&uuid))
  }

  /// [`format`](https://developer.apple.com/documentation/diskimagekit/diskimage/format-swift.property)
  pub fn format(&self) -> DiskImageFormat {
    self.inner.dik_format().into()
  }

  /// [`openMode`](https://developer.apple.com/documentation/diskimagekit/diskimage/openmode)
  pub fn open_mode(&self) -> OpenConfigurationMode {
    self.inner.dik_open_mode().into()
  }

  /// [`appending(_:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/appending(_:)-(diskimage)),
  /// with an existing image. Consuming, as in Swift.
  pub fn appending_layer(self, layer: DiskImage) -> Result<StackedImage, Error> {
    Ok(StackedImage {
      disk_image: DiskImage {
        inner: self.inner.dik_appending_disk_image(layer.inner)?,
      },
    })
  }

  /// [`appending(_:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/appending(_:)-(creationconfiguration&stackablelayer)),
  /// with a new layer. Consuming, as in Swift.
  pub fn appending_configuration(
    self,
    configuration: &(impl CreationConfiguration + StackableLayer),
  ) -> Result<StackedImage, Error> {
    configuration.bridge_appending(self)
  }

  /// [`truncate(blockCount:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/truncate(blockcount:))
  pub fn truncate(&self, block_count: isize) -> Result<(), Error> {
    Ok(self.inner.dik_truncate(block_count)?)
  }

  #[cfg(target_os = "macos")]
  pub(crate) fn bridged(&self) -> &ffi::DiskImage {
    &self.inner
  }
}

impl Clone for DiskImage {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The image's `String(describing:)`.
impl fmt::Debug for DiskImage {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

/// [`StackedImage`](https://developer.apple.com/documentation/diskimagekit/stackedimage):
/// what `appending(_:)` returns. A `DiskImage`, as in Swift.
#[derive(Clone)]
pub struct StackedImage {
  pub(crate) disk_image: DiskImage,
}

impl StackedImage {
  /// [`layers`](https://developer.apple.com/documentation/diskimagekit/stackedimage/layers)
  pub fn layers(&self) -> Vec<DiskImage> {
    (0..self.disk_image.inner.bridge_layers_count())
      .map(|index| DiskImage {
        inner: self.disk_image.inner.bridge_layer_at(index),
      })
      .collect()
  }
}

impl Deref for StackedImage {
  type Target = DiskImage;

  fn deref(&self) -> &DiskImage {
    &self.disk_image
  }
}

/// A `StackedImage` is a `DiskImage`, so it can be appended to.
impl From<StackedImage> for DiskImage {
  fn from(stacked_image: StackedImage) -> Self {
    stacked_image.disk_image
  }
}

impl fmt::Debug for StackedImage {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.disk_image.fmt(formatter)
  }
}

/// [`DiskImage.Format`](https://developer.apple.com/documentation/diskimagekit/diskimage/format-swift.enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiskImageFormat {
  /// [`asif`](https://developer.apple.com/documentation/diskimagekit/diskimage/format-swift.enum/asif)
  Asif,
  /// [`raw`](https://developer.apple.com/documentation/diskimagekit/diskimage/format-swift.enum/raw)
  Raw,
  /// [`stack`](https://developer.apple.com/documentation/diskimagekit/diskimage/format-swift.enum/stack)
  Stack,
  /// A case newer than this crate. Swift's `Format` has no raw values.
  Unknown,
}

impl From<DiskImageFormat> for ffi::DiskImageFormat {
  fn from(format: DiskImageFormat) -> Self {
    match format {
      DiskImageFormat::Asif => Self::Asif,
      DiskImageFormat::Raw => Self::Raw,
      DiskImageFormat::Stack => Self::Stack,
      DiskImageFormat::Unknown => Self::Unknown,
    }
  }
}

impl From<ffi::DiskImageFormat> for DiskImageFormat {
  fn from(format: ffi::DiskImageFormat) -> Self {
    match format {
      ffi::DiskImageFormat::Asif => Self::Asif,
      ffi::DiskImageFormat::Raw => Self::Raw,
      ffi::DiskImageFormat::Stack => Self::Stack,
      ffi::DiskImageFormat::Unknown => Self::Unknown,
    }
  }
}

/// [`DiskImage.BlockSize`](https://developer.apple.com/documentation/diskimagekit/diskimage/blocksize-swift.enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiskImageBlockSize {
  /// [`bytes512`](https://developer.apple.com/documentation/diskimagekit/diskimage/blocksize-swift.enum/bytes512)
  Bytes512,
  /// [`bytes4096`](https://developer.apple.com/documentation/diskimagekit/diskimage/blocksize-swift.enum/bytes4096)
  Bytes4096,
  /// A case newer than this crate, by its raw value. Passing one DiskImageKit
  /// has no case for stops the process.
  Unknown(isize),
}

impl From<DiskImageBlockSize> for ffi::DiskImageBlockSize {
  fn from(block_size: DiskImageBlockSize) -> Self {
    match block_size {
      DiskImageBlockSize::Bytes512 => Self::Bytes512,
      DiskImageBlockSize::Bytes4096 => Self::Bytes4096,
      DiskImageBlockSize::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::DiskImageBlockSize> for DiskImageBlockSize {
  fn from(block_size: ffi::DiskImageBlockSize) -> Self {
    match block_size {
      ffi::DiskImageBlockSize::Bytes512 => Self::Bytes512,
      ffi::DiskImageBlockSize::Bytes4096 => Self::Bytes4096,
      ffi::DiskImageBlockSize::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`DiskImage.LayerType`](https://developer.apple.com/documentation/diskimagekit/diskimage/layertype-swift.struct)
pub struct DiskImageLayerType {
  pub(crate) inner: ffi::BridgedDiskImageLayerType,
}

impl DiskImageLayerType {
  /// [`cache`](https://developer.apple.com/documentation/diskimagekit/diskimage/layertype-swift.struct/cache)
  pub fn cache() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_disk_image_layer_type_cache(),
    })
  }

  /// [`overlay`](https://developer.apple.com/documentation/diskimagekit/diskimage/layertype-swift.struct/overlay)
  pub fn overlay() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_disk_image_layer_type_overlay(),
    })
  }

  /// [`overlay(blockCount:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/layertype-swift.struct/overlay(blockcount:))
  pub fn overlay_block_count(block_count: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_disk_image_layer_type_overlay_block_count(block_count),
    })
  }
}

impl Clone for DiskImageLayerType {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// `==`.
impl PartialEq for DiskImageLayerType {
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl Eq for DiskImageLayerType {}

/// The value's `String(describing:)`.
impl fmt::Debug for DiskImageLayerType {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_formats_both_ways() {
    for format in [
      DiskImageFormat::Asif,
      DiskImageFormat::Raw,
      DiskImageFormat::Stack,
      DiskImageFormat::Unknown,
    ] {
      assert_eq!(DiskImageFormat::from(ffi::DiskImageFormat::from(format)), format);
    }
  }

  #[test]
  fn maps_block_sizes_both_ways() {
    for block_size in [
      DiskImageBlockSize::Bytes512,
      DiskImageBlockSize::Bytes4096,
      DiskImageBlockSize::Unknown(7),
    ] {
      assert_eq!(
        DiskImageBlockSize::from(ffi::DiskImageBlockSize::from(block_size)),
        block_size
      );
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(DiskImageLayerType::cache(), Err(Error::Unsupported)));
    assert!(matches!(DiskImageLayerType::overlay(), Err(Error::Unsupported)));
    assert!(matches!(
      DiskImageLayerType::overlay_block_count(8),
      Err(Error::Unsupported)
    ));
  }
}
