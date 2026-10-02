use super::{
  BridgedASIFCreationConfiguration, BridgedASIFLayerCreationConfiguration, BridgedRAWCreationConfiguration,
  DiskImageKitError, DiskImageKitNil, GUARDED, NSURL, NSUUID, Never, OpenConfigurationMode, Retained,
};

pub(crate) enum DiskImageFormat {
  Asif,
  Raw,
  Stack,
  Unknown,
}

pub(crate) enum DiskImageBlockSize {
  Bytes512,
  Bytes4096,
  Unknown(isize),
}

pub(crate) struct DiskImage {
  never: Never,
}

pub(crate) fn dik_disk_image_init_creating_asif(
  _configuration: &BridgedASIFCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_disk_image_init_creating_asif_layer(
  _configuration: &BridgedASIFLayerCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_disk_image_init_creating_raw(
  _configuration: &BridgedRAWCreationConfiguration,
) -> Result<DiskImage, DiskImageKitError> {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_disk_image_init_opening(
  _url: &NSURL,
  _mode: OpenConfigurationMode,
) -> Result<DiskImage, DiskImageKitError> {
  unreachable!("{GUARDED}")
}

impl DiskImage {
  pub(crate) fn bridge_retained(&self) -> DiskImage {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn dik_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn dik_block_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn dik_block_size(&self) -> DiskImageBlockSize {
    match self.never {}
  }

  pub(crate) fn dik_size(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn dik_layer_type(&self) -> Result<BridgedDiskImageLayerType, DiskImageKitNil> {
    match self.never {}
  }

  pub(crate) fn dik_layer_uuid(&self) -> Option<Retained<NSUUID>> {
    match self.never {}
  }

  pub(crate) fn dik_parent_uuid(&self) -> Option<Retained<NSUUID>> {
    match self.never {}
  }

  pub(crate) fn dik_format(&self) -> DiskImageFormat {
    match self.never {}
  }

  pub(crate) fn dik_open_mode(&self) -> OpenConfigurationMode {
    match self.never {}
  }

  pub(crate) fn dik_appending_disk_image(self, _layer: DiskImage) -> Result<DiskImage, DiskImageKitError> {
    match self.never {}
  }

  pub(crate) fn dik_appending_asif_layer(
    self,
    _configuration: &BridgedASIFLayerCreationConfiguration,
  ) -> Result<DiskImage, DiskImageKitError> {
    match self.never {}
  }

  pub(crate) fn dik_truncate(&self, _block_count: isize) -> Result<(), DiskImageKitError> {
    match self.never {}
  }

  pub(crate) fn bridge_layers_count(&self) -> usize {
    match self.never {}
  }

  pub(crate) fn bridge_layer_at(&self, _index: usize) -> DiskImage {
    match self.never {}
  }
}

pub(crate) struct BridgedDiskImageLayerType {
  never: Never,
}

pub(crate) fn dik_disk_image_layer_type_cache() -> BridgedDiskImageLayerType {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_disk_image_layer_type_overlay() -> BridgedDiskImageLayerType {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_disk_image_layer_type_overlay_block_count(_block_count: isize) -> BridgedDiskImageLayerType {
  unreachable!("{GUARDED}")
}

impl BridgedDiskImageLayerType {
  pub(crate) fn bridge_retained(&self) -> BridgedDiskImageLayerType {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }
}

impl PartialEq for BridgedDiskImageLayerType {
  fn eq(&self, _other: &Self) -> bool {
    match self.never {}
  }
}
