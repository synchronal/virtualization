use super::{GUARDED, NSObject, NSURL, Never, Retained};

pub(crate) struct VZLinuxBootLoader {
  never: Never,
}

pub(crate) fn vz_linux_boot_loader_init(_kernel_url: &NSURL) -> VZLinuxBootLoader {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_linux_boot_loader_bridge_cast(object: &NSObject) -> Option<VZLinuxBootLoader> {
  object.never()
}

impl VZLinuxBootLoader {
  pub(crate) fn bridge_retained(&self) -> VZLinuxBootLoader {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_kernel_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_set_kernel_url(&self, _kernel_url: &NSURL) {
    match self.never {}
  }

  pub(crate) fn vz_command_line(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_command_line(&self, _command_line: &str) {
    match self.never {}
  }

  pub(crate) fn vz_initial_ramdisk_url(&self) -> Option<Retained<NSURL>> {
    match self.never {}
  }

  pub(crate) fn vz_set_initial_ramdisk_url(&self, _initial_ramdisk_url: Option<&NSURL>) {
    match self.never {}
  }
}

pub(crate) struct VZGenericPlatformConfiguration {
  never: Never,
}

pub(crate) fn vz_generic_platform_configuration_init() -> VZGenericPlatformConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_generic_platform_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZGenericPlatformConfiguration> {
  object.never()
}

pub(crate) fn vz_generic_platform_configuration_is_nested_virtualization_supported() -> bool {
  unreachable!("{GUARDED}")
}

impl VZGenericPlatformConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZGenericPlatformConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_machine_identifier(&self) -> VZGenericMachineIdentifier {
    match self.never {}
  }

  pub(crate) fn vz_set_machine_identifier(&self, _machine_identifier: VZGenericMachineIdentifier) {
    match self.never {}
  }

  pub(crate) fn vz_is_nested_virtualization_enabled(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_nested_virtualization_enabled(&self, _nested_virtualization_enabled: bool) {
    match self.never {}
  }
}

pub(crate) struct VZGenericMachineIdentifier {
  never: Never,
}

pub(crate) fn vz_generic_machine_identifier_init() -> VZGenericMachineIdentifier {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_generic_machine_identifier_init_data_representation(
  _data_representation: &[u8],
) -> Option<VZGenericMachineIdentifier> {
  unreachable!("{GUARDED}")
}

impl VZGenericMachineIdentifier {
  pub(crate) fn bridge_retained(&self) -> VZGenericMachineIdentifier {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn ns_is_equal(&self, _object: VZGenericMachineIdentifier) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_data_representation(&self) -> Vec<u8> {
    match self.never {}
  }
}
