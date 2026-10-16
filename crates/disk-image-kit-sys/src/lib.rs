//! FFI to Apple's [DiskImageKit] framework, through swift-bridge. Use the
//! `disk-image-kit` crate instead; this one holds its `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them, the way
//! `virtualization-sys`'s are:
//!
//! - Types keep Apple's names; a nested one joins them (`DiskImage.Format` is
//!   [`DiskImageFormat`]). Members are methods `dik_<member>`; initializers
//!   and static members are free functions `dik_<type>_init[_<labels>]` and
//!   `dik_<type>_<member>`.
//! - DiskImageKit's structs cross in Swift classes: `Bridged<Name>`.
//! - Foundation's objects are objc2's, through `foundation-sys`, and cross as
//!   pointers. The bridge module is private, and each function that takes or
//!   returns one, or throws, is wrapped here.
//! - What DiskImageKit throws is [`DiskImageKitError`]: its own error types by
//!   name, anything else an `NSError`.
//! - Enums keep Apple's cases, plus `Unknown` for a case newer than this
//!   crate (`Unknown(rawValue)` for one with raw values).
//!
//! The glue's names differ from the other bridges': all link into one binary,
//! and swift-bridge names symbols for each bridge's types.
//!
//! macOS only: the build fails elsewhere.
//!
//! [DiskImageKit]: https://developer.apple.com/documentation/diskimagekit

use std::ffi::c_void;

pub use foundation_sys::objc::{
  NSError, NSURL, NSUUID, Retained, ns_url_absolute_string, ns_url_init_string, ns_uuid_uuid_string,
};
use foundation_sys::objc::{borrowed, retained, retained_or_nil};

pub use self::ffi::{
  BridgedASIFCreationConfiguration, BridgedASIFLayerCreationConfiguration, BridgedDiskImageLayerType,
  BridgedRAWCreationConfiguration, DiskImage, DiskImageBlockSize, DiskImageFormat, DiskImageKitNil,
  OpenConfigurationMode, dik_disk_image_layer_type_cache, dik_disk_image_layer_type_overlay,
  dik_disk_image_layer_type_overlay_block_count,
};

// No doc comments inside the module: swift-bridge can't parse them. Shared
// enums are renamed on the Swift side, where Apple's names are taken.
//
// A function with a Foundation pointer, or that throws, takes the object it
// is a member of by value: swift-bridge 0.1.59 releases an object a free
// function borrows.
#[swift_bridge::bridge]
mod ffi {
  // Foundation.swift

  enum DiskImageKitNil {
    Nil,
  }

  enum DiskImageKitThrown {
    IncompatibleStackingError {
      reason: String,
      error_description: Option<String>,
    },
    InvalidBlockCountError {
      error_description: Option<String>,
    },
    CorruptedImageError {
      underlying_error: usize,
      error_description: Option<String>,
    },
    UnsupportedFormatError {
      underlying_error: usize,
      error_description: Option<String>,
    },
    Error(usize),
  }

  // DiskImage.swift

  #[swift_bridge(swift_name = "BridgedDiskImageFormat")]
  enum DiskImageFormat {
    Asif,
    Raw,
    Stack,
    Unknown,
  }

  #[swift_bridge(swift_name = "BridgedDiskImageBlockSize")]
  enum DiskImageBlockSize {
    Bytes512,
    Bytes4096,
    Unknown(isize),
  }

  extern "Swift" {
    type DiskImage;

    fn dik_disk_image_init_creating_asif(
      configuration: BridgedASIFCreationConfiguration,
    ) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_init_creating_asif_layer(
      configuration: BridgedASIFLayerCreationConfiguration,
    ) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_init_creating_raw(
      configuration: BridgedRAWCreationConfiguration,
    ) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_init_opening(
      url: *const c_void,
      mode: OpenConfigurationMode,
    ) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_url(this: DiskImage) -> *mut c_void;
    fn dik_disk_image_layer_uuid(this: DiskImage) -> usize;
    fn dik_disk_image_parent_uuid(this: DiskImage) -> usize;
    fn dik_disk_image_appending_disk_image(this: DiskImage, layer: DiskImage) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_appending_asif_layer(
      this: DiskImage,
      configuration: BridgedASIFLayerCreationConfiguration,
    ) -> Result<DiskImage, DiskImageKitThrown>;
    fn dik_disk_image_truncate(this: DiskImage, block_count: isize) -> Result<(), DiskImageKitThrown>;
    fn bridge_retained(self: &DiskImage) -> DiskImage;
    fn bridge_description(self: &DiskImage) -> String;
    fn bridge_borrowed(self: &DiskImage) -> *const c_void;
    fn dik_block_count(self: &DiskImage) -> isize;
    fn dik_block_size(self: &DiskImage) -> DiskImageBlockSize;
    fn dik_size(self: &DiskImage) -> isize;
    fn dik_layer_type(self: &DiskImage) -> Result<BridgedDiskImageLayerType, DiskImageKitNil>;
    fn dik_format(self: &DiskImage) -> DiskImageFormat;
    fn dik_open_mode(self: &DiskImage) -> OpenConfigurationMode;
    fn bridge_layers_count(self: &DiskImage) -> usize;
    fn bridge_layer_at(self: &DiskImage, index: usize) -> DiskImage;
  }

  extern "Swift" {
    type BridgedDiskImageLayerType;

    fn dik_disk_image_layer_type_cache() -> BridgedDiskImageLayerType;
    fn dik_disk_image_layer_type_overlay() -> BridgedDiskImageLayerType;
    fn dik_disk_image_layer_type_overlay_block_count(block_count: isize) -> BridgedDiskImageLayerType;
    fn bridge_retained(self: &BridgedDiskImageLayerType) -> BridgedDiskImageLayerType;
    fn bridge_description(self: &BridgedDiskImageLayerType) -> String;
    fn bridge_eq(self: &BridgedDiskImageLayerType, other: BridgedDiskImageLayerType) -> bool;
  }

  // CreationConfiguration.swift

  extern "Swift" {
    type BridgedASIFCreationConfiguration;

    fn dik_asif_creation_configuration_asif(
      url: *const c_void,
      block_count: isize,
      block_size: DiskImageBlockSize,
    ) -> BridgedASIFCreationConfiguration;
    fn dik_asif_creation_configuration_layer(
      url: *const c_void,
      layer_type: BridgedDiskImageLayerType,
    ) -> BridgedASIFLayerCreationConfiguration;
    fn bridge_retained(self: &BridgedASIFCreationConfiguration) -> BridgedASIFCreationConfiguration;
    fn bridge_description(self: &BridgedASIFCreationConfiguration) -> String;
  }

  extern "Swift" {
    type BridgedASIFLayerCreationConfiguration;

    fn dik_asif_layer_creation_configuration_asif_layer(
      url: *const c_void,
      layer_type: BridgedDiskImageLayerType,
    ) -> BridgedASIFLayerCreationConfiguration;
    fn bridge_retained(self: &BridgedASIFLayerCreationConfiguration) -> BridgedASIFLayerCreationConfiguration;
    fn bridge_description(self: &BridgedASIFLayerCreationConfiguration) -> String;
  }

  extern "Swift" {
    type BridgedRAWCreationConfiguration;

    fn dik_raw_creation_configuration_raw(url: *const c_void, block_count: isize) -> BridgedRAWCreationConfiguration;
    fn bridge_retained(self: &BridgedRAWCreationConfiguration) -> BridgedRAWCreationConfiguration;
    fn bridge_description(self: &BridgedRAWCreationConfiguration) -> String;
  }

  // OpenConfiguration.swift

  #[swift_bridge(swift_name = "BridgedOpenConfigurationMode")]
  enum OpenConfigurationMode {
    Automatic,
    ReadOnly,
    ReadWrite,
    Unknown,
  }
}

// Foundation.swift

/// What DiskImageKit threw: one of the error types it declares, by name,
/// with its fields, or anything else as an `NSError`.
pub enum DiskImageKitError {
  IncompatibleStackingError {
    reason: String,
    error_description: Option<String>,
  },
  InvalidBlockCountError {
    error_description: Option<String>,
  },
  CorruptedImageError {
    underlying_error: Retained<NSError>,
    error_description: Option<String>,
  },
  UnsupportedFormatError {
    underlying_error: Retained<NSError>,
    error_description: Option<String>,
  },
  Error(Retained<NSError>),
}

/// An `NSError` Swift handed over through `bridging(retainedAddress:)`.
fn thrown_ns_error(address: usize) -> Retained<NSError> {
  // SAFETY: Swift's `bridging(_ body:)` passes each `NSError` it throws
  // retained, by address, once.
  unsafe { retained_or_nil(address) }.expect("Swift throws an NSError, never nil")
}

impl From<ffi::DiskImageKitThrown> for DiskImageKitError {
  fn from(thrown: ffi::DiskImageKitThrown) -> Self {
    match thrown {
      ffi::DiskImageKitThrown::IncompatibleStackingError {
        reason,
        error_description,
      } => Self::IncompatibleStackingError {
        reason,
        error_description,
      },
      ffi::DiskImageKitThrown::InvalidBlockCountError { error_description } => {
        Self::InvalidBlockCountError { error_description }
      }
      ffi::DiskImageKitThrown::CorruptedImageError {
        underlying_error,
        error_description,
      } => Self::CorruptedImageError {
        underlying_error: thrown_ns_error(underlying_error),
        error_description,
      },
      ffi::DiskImageKitThrown::UnsupportedFormatError {
        underlying_error,
        error_description,
      } => Self::UnsupportedFormatError {
        underlying_error: thrown_ns_error(underlying_error),
        error_description,
      },
      ffi::DiskImageKitThrown::Error(error) => Self::Error(thrown_ns_error(error)),
    }
  }
}

// DiskImage.swift

pub fn dik_disk_image_init_creating_asif(
  configuration: &BridgedASIFCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  Ok(ffi::dik_disk_image_init_creating_asif(configuration.bridge_retained())?)
}

pub fn dik_disk_image_init_creating_asif_layer(
  configuration: &BridgedASIFLayerCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  Ok(ffi::dik_disk_image_init_creating_asif_layer(
    configuration.bridge_retained(),
  )?)
}

pub fn dik_disk_image_init_creating_raw(
  configuration: &BridgedRAWCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  Ok(ffi::dik_disk_image_init_creating_raw(configuration.bridge_retained())?)
}

/// `init(opening:)` with an `OpenConfiguration` of `url` and `mode`.
pub fn dik_disk_image_init_opening(url: &NSURL, mode: OpenConfigurationMode) -> Result<DiskImage, DiskImageKitError> {
  Ok(ffi::dik_disk_image_init_opening(borrowed(url), mode)?)
}

impl DiskImage {
  pub fn dik_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe { retained(ffi::dik_disk_image_url(self.bridge_retained())) }
  }

  pub fn dik_layer_uuid(&self) -> Option<Retained<NSUUID>> {
    // SAFETY: `layerUUID`, a retained `NSUUID` or `nil`.
    unsafe { retained_or_nil(ffi::dik_disk_image_layer_uuid(self.bridge_retained())) }
  }

  pub fn dik_parent_uuid(&self) -> Option<Retained<NSUUID>> {
    // SAFETY: `parentUUID`, a retained `NSUUID` or `nil`.
    unsafe { retained_or_nil(ffi::dik_disk_image_parent_uuid(self.bridge_retained())) }
  }

  /// `appending(_:)`, consuming, as in Swift: the result is a `StackedImage`.
  pub fn dik_appending_disk_image(self, layer: DiskImage) -> Result<DiskImage, DiskImageKitError> {
    Ok(ffi::dik_disk_image_appending_disk_image(self, layer)?)
  }

  /// `appending(_:)`, consuming, as in Swift: the result is a `StackedImage`.
  pub fn dik_appending_asif_layer(
    self,
    configuration: &BridgedASIFLayerCreationConfiguration,
  ) -> Result<DiskImage, DiskImageKitError> {
    Ok(ffi::dik_disk_image_appending_asif_layer(
      self,
      configuration.bridge_retained(),
    )?)
  }

  pub fn dik_truncate(&self, block_count: isize) -> Result<(), DiskImageKitError> {
    Ok(ffi::dik_disk_image_truncate(self.bridge_retained(), block_count)?)
  }
}

/// `==`.
impl PartialEq for BridgedDiskImageLayerType {
  fn eq(&self, other: &Self) -> bool {
    self.bridge_eq(other.bridge_retained())
  }
}

impl Eq for BridgedDiskImageLayerType {}

// CreationConfiguration.swift

pub fn dik_asif_creation_configuration_asif(
  url: &NSURL,
  block_count: isize,
  block_size: DiskImageBlockSize,
) -> BridgedASIFCreationConfiguration {
  ffi::dik_asif_creation_configuration_asif(borrowed(url), block_count, block_size)
}

pub fn dik_asif_creation_configuration_layer(
  url: &NSURL,
  layer_type: &BridgedDiskImageLayerType,
) -> BridgedASIFLayerCreationConfiguration {
  ffi::dik_asif_creation_configuration_layer(borrowed(url), layer_type.bridge_retained())
}

pub fn dik_asif_layer_creation_configuration_asif_layer(
  url: &NSURL,
  layer_type: &BridgedDiskImageLayerType,
) -> BridgedASIFLayerCreationConfiguration {
  ffi::dik_asif_layer_creation_configuration_asif_layer(borrowed(url), layer_type.bridge_retained())
}

pub fn dik_raw_creation_configuration_raw(url: &NSURL, block_count: isize) -> BridgedRAWCreationConfiguration {
  ffi::dik_raw_creation_configuration_raw(borrowed(url), block_count)
}
