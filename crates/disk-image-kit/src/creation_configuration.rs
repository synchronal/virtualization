use std::fmt;

use url::Url;

use crate::platform::{self, ffi};
use crate::{DiskImage, DiskImageBlockSize, DiskImageLayerType, Error, StackedImage, foundation, sealed};

/// [`DiskImage.CreationConfiguration`](https://developer.apple.com/documentation/diskimagekit/diskimage/creationconfiguration):
/// what `DiskImage::new` takes.
pub trait CreationConfiguration: sealed::CreationConfiguration {}

/// [`DiskImage.StackableLayer`](https://developer.apple.com/documentation/diskimagekit/diskimage/stackablelayer):
/// what `DiskImage::appending_configuration` takes.
pub trait StackableLayer: sealed::StackableLayer {}

/// [`ASIFCreationConfiguration`](https://developer.apple.com/documentation/diskimagekit/asifcreationconfiguration)
pub struct ASIFCreationConfiguration {
  inner: ffi::BridgedASIFCreationConfiguration,
}

impl ASIFCreationConfiguration {
  /// [`asif(url:blockCount:blockSize:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/creationconfiguration/asif(url:blockcount:blocksize:))
  pub fn asif(url: &Url, block_count: isize, block_size: DiskImageBlockSize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_asif_creation_configuration_asif(&*foundation::ns_url(url)?, block_count, block_size.into()),
    })
  }

  /// [`layer(url:type:)`](https://developer.apple.com/documentation/diskimagekit/asifcreationconfiguration/layer(url:type:))
  pub fn layer(url: &Url, layer_type: &DiskImageLayerType) -> Result<ASIFLayerCreationConfiguration, Error> {
    platform::supported()?;

    Ok(ASIFLayerCreationConfiguration {
      inner: ffi::dik_asif_creation_configuration_layer(&*foundation::ns_url(url)?, &layer_type.inner),
    })
  }
}

impl sealed::CreationConfiguration for ASIFCreationConfiguration {
  fn bridge_init_creating(&self) -> Result<DiskImage, Error> {
    Ok(DiskImage {
      inner: ffi::dik_disk_image_init_creating_asif(&self.inner)?,
    })
  }
}

impl CreationConfiguration for ASIFCreationConfiguration {}

impl Clone for ASIFCreationConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The value's `String(describing:)`.
impl fmt::Debug for ASIFCreationConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

/// [`ASIFLayerCreationConfiguration`](https://developer.apple.com/documentation/diskimagekit/asiflayercreationconfiguration)
pub struct ASIFLayerCreationConfiguration {
  inner: ffi::BridgedASIFLayerCreationConfiguration,
}

impl ASIFLayerCreationConfiguration {
  /// [`asifLayer(url:type:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/creationconfiguration/asiflayer(url:type:))
  pub fn asif_layer(url: &Url, layer_type: &DiskImageLayerType) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_asif_layer_creation_configuration_asif_layer(&*foundation::ns_url(url)?, &layer_type.inner),
    })
  }
}

impl sealed::CreationConfiguration for ASIFLayerCreationConfiguration {
  fn bridge_init_creating(&self) -> Result<DiskImage, Error> {
    Ok(DiskImage {
      inner: ffi::dik_disk_image_init_creating_asif_layer(&self.inner)?,
    })
  }
}

impl CreationConfiguration for ASIFLayerCreationConfiguration {}

impl sealed::StackableLayer for ASIFLayerCreationConfiguration {
  fn bridge_appending(&self, disk_image: DiskImage) -> Result<StackedImage, Error> {
    Ok(StackedImage {
      disk_image: DiskImage {
        inner: disk_image.inner.dik_appending_asif_layer(&self.inner)?,
      },
    })
  }
}

impl StackableLayer for ASIFLayerCreationConfiguration {}

impl Clone for ASIFLayerCreationConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The value's `String(describing:)`.
impl fmt::Debug for ASIFLayerCreationConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

/// [`RAWCreationConfiguration`](https://developer.apple.com/documentation/diskimagekit/rawcreationconfiguration)
pub struct RAWCreationConfiguration {
  inner: ffi::BridgedRAWCreationConfiguration,
}

impl RAWCreationConfiguration {
  /// [`raw(url:blockCount:)`](https://developer.apple.com/documentation/diskimagekit/diskimage/creationconfiguration/raw(url:blockcount:))
  pub fn raw(url: &Url, block_count: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::dik_raw_creation_configuration_raw(&*foundation::ns_url(url)?, block_count),
    })
  }
}

impl sealed::CreationConfiguration for RAWCreationConfiguration {
  fn bridge_init_creating(&self) -> Result<DiskImage, Error> {
    Ok(DiskImage {
      inner: ffi::dik_disk_image_init_creating_raw(&self.inner)?,
    })
  }
}

impl CreationConfiguration for RAWCreationConfiguration {}

impl Clone for RAWCreationConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The value's `String(describing:)`.
impl fmt::Debug for RAWCreationConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("file:///nowhere/disk.asif").unwrap();

    assert!(matches!(
      ASIFCreationConfiguration::asif(&url, 8, DiskImageBlockSize::Bytes512),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      RAWCreationConfiguration::raw(&url, 8),
      Err(Error::Unsupported)
    ));
  }
}
