use super::{BridgedDiskImageLayerType, DiskImageBlockSize, GUARDED, NSURL, Never};

pub(crate) struct BridgedASIFCreationConfiguration {
  never: Never,
}

pub(crate) fn dik_asif_creation_configuration_asif(
  _url: &NSURL,
  _block_count: isize,
  _block_size: DiskImageBlockSize,
) -> BridgedASIFCreationConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn dik_asif_creation_configuration_layer(
  _url: &NSURL,
  _layer_type: &BridgedDiskImageLayerType,
) -> BridgedASIFLayerCreationConfiguration {
  unreachable!("{GUARDED}")
}

impl BridgedASIFCreationConfiguration {
  pub(crate) fn bridge_retained(&self) -> BridgedASIFCreationConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct BridgedASIFLayerCreationConfiguration {
  never: Never,
}

pub(crate) fn dik_asif_layer_creation_configuration_asif_layer(
  _url: &NSURL,
  _layer_type: &BridgedDiskImageLayerType,
) -> BridgedASIFLayerCreationConfiguration {
  unreachable!("{GUARDED}")
}

impl BridgedASIFLayerCreationConfiguration {
  pub(crate) fn bridge_retained(&self) -> BridgedASIFLayerCreationConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct BridgedRAWCreationConfiguration {
  never: Never,
}

pub(crate) fn dik_raw_creation_configuration_raw(_url: &NSURL, _block_count: isize) -> BridgedRAWCreationConfiguration {
  unreachable!("{GUARDED}")
}

impl BridgedRAWCreationConfiguration {
  pub(crate) fn bridge_retained(&self) -> BridgedRAWCreationConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }
}
