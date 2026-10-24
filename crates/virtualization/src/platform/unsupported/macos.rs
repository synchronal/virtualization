use std::ffi::c_void;

use super::{
  BridgedNil, BridgedOperatingSystemVersion, BridgedThrown, GUARDED, Local, NSError, NSObject, NSURL, Never, Retained,
  VZVirtualMachine,
};

pub(crate) struct VZMacAuxiliaryStorage {
  never: Never,
  local: Local,
}

pub(crate) fn vz_mac_auxiliary_storage_init(_url: &NSURL) -> VZMacAuxiliaryStorage {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_auxiliary_storage_init_creating_storage_at_hardware_model_options(
  _url: &NSURL,
  _hardware_model: &VZMacHardwareModel,
  _options: usize,
) -> Result<VZMacAuxiliaryStorage, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_auxiliary_storage_initialization_options_allow_overwrite() -> usize {
  unreachable!("{GUARDED}")
}

impl VZMacAuxiliaryStorage {
  pub(crate) fn bridge_retained(&self) -> VZMacAuxiliaryStorage {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }
}

pub(crate) struct VZMacGuestProvisioningOptions {
  never: Never,
}

pub(crate) fn vz_mac_guest_provisioning_options_init() -> VZMacGuestProvisioningOptions {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_guest_provisioning_options_bridge_cast(
  object: &NSObject,
) -> Option<VZMacGuestProvisioningOptions> {
  object.never()
}

impl VZMacGuestProvisioningOptions {
  pub(crate) fn bridge_retained(&self) -> VZMacGuestProvisioningOptions {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_full_name(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_full_name(&self, _full_name: &str) {
    match self.never {}
  }

  pub(crate) fn vz_username(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_username(&self, _username: &str) {
    match self.never {}
  }

  pub(crate) fn vz_password(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_password(&self, _password: &str) {
    match self.never {}
  }

  pub(crate) fn vz_logs_in_automatically(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_logs_in_automatically(&self, _logs_in_automatically: bool) {
    match self.never {}
  }

  pub(crate) fn vz_enables_remote_login(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_enables_remote_login(&self, _enables_remote_login: bool) {
    match self.never {}
  }
}

pub(crate) struct VZMacHardwareModel {
  never: Never,
  local: Local,
}

pub(crate) fn vz_mac_hardware_model_init_data_representation(
  _data_representation: &[u8],
) -> Option<VZMacHardwareModel> {
  unreachable!("{GUARDED}")
}

impl VZMacHardwareModel {
  pub(crate) fn bridge_retained(&self) -> VZMacHardwareModel {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_data_representation(&self) -> Vec<u8> {
    match self.never {}
  }

  pub(crate) fn vz_is_supported(&self) -> bool {
    match self.never {}
  }
}

pub(crate) struct VZMacMachineIdentifier {
  never: Never,
  local: Local,
}

pub(crate) fn vz_mac_machine_identifier_init() -> VZMacMachineIdentifier {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_machine_identifier_init_data_representation(
  _data_representation: &[u8],
) -> Option<VZMacMachineIdentifier> {
  unreachable!("{GUARDED}")
}

impl VZMacMachineIdentifier {
  pub(crate) fn bridge_retained(&self) -> VZMacMachineIdentifier {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn ns_is_equal(&self, _object: VZMacMachineIdentifier) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_data_representation(&self) -> Vec<u8> {
    match self.never {}
  }
}

pub(crate) struct VZMacOSBootLoader {
  never: Never,
  local: Local,
}

pub(crate) fn vz_macos_boot_loader_init() -> VZMacOSBootLoader {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_macos_boot_loader_bridge_cast(object: &NSObject) -> Option<VZMacOSBootLoader> {
  object.never()
}

impl VZMacOSBootLoader {
  pub(crate) fn bridge_retained(&self) -> VZMacOSBootLoader {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZMacOSConfigurationRequirements {
  never: Never,
  local: Local,
}

impl VZMacOSConfigurationRequirements {
  pub(crate) fn bridge_retained(&self) -> VZMacOSConfigurationRequirements {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_hardware_model(&self) -> VZMacHardwareModel {
    match self.never {}
  }

  pub(crate) fn vz_minimum_supported_cpu_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_minimum_supported_memory_size(&self) -> u64 {
    match self.never {}
  }
}

pub(crate) struct VZMacOSInstaller {
  never: Never,
}

pub(crate) fn vz_macos_installer_init_restoring_from_image_at(
  _virtual_machine: &VZVirtualMachine,
  _restore_image_file_url: &NSURL,
) -> VZMacOSInstaller {
  unreachable!("{GUARDED}")
}

impl VZMacOSInstaller {
  pub(crate) fn bridge_retained(&self) -> VZMacOSInstaller {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_install(&self, _completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    match self.never {}
  }

  pub(crate) fn vz_progress(&self) -> *const c_void {
    match self.never {}
  }

  pub(crate) fn vz_virtual_machine(&self) -> VZVirtualMachine {
    match self.never {}
  }

  pub(crate) fn vz_restore_image_url(&self) -> Retained<NSURL> {
    match self.never {}
  }
}

pub(crate) struct VZMacOSRestoreImage {
  never: Never,
  local: Local,
}

pub(crate) fn vz_macos_restore_image_load(
  _file_url: &NSURL,
  _completion_handler: impl FnOnce(Result<VZMacOSRestoreImage, Retained<NSError>>) + Send + 'static,
) {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_macos_restore_image_fetch_latest_supported(
  _completion_handler: impl FnOnce(Result<VZMacOSRestoreImage, Retained<NSError>>) + Send + 'static,
) {
  unreachable!("{GUARDED}")
}

impl VZMacOSRestoreImage {
  pub(crate) fn bridge_retained(&self) -> VZMacOSRestoreImage {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_is_supported(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_build_version(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_operating_system_version(&self) -> BridgedOperatingSystemVersion {
    match self.never {}
  }

  pub(crate) fn vz_most_featureful_supported_configuration(
    &self,
  ) -> Result<VZMacOSConfigurationRequirements, BridgedNil> {
    match self.never {}
  }
}

pub(crate) struct VZMacOSVirtualMachineStartOptions {
  never: Never,
  local: Local,
}

pub(crate) fn vz_macos_virtual_machine_start_options_init() -> VZMacOSVirtualMachineStartOptions {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_macos_virtual_machine_start_options_bridge_cast(
  object: &NSObject,
) -> Option<VZMacOSVirtualMachineStartOptions> {
  object.never()
}

impl VZMacOSVirtualMachineStartOptions {
  pub(crate) fn bridge_retained(&self) -> VZMacOSVirtualMachineStartOptions {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_start_up_from_macos_recovery(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_start_up_from_macos_recovery(&self, _start_up_from_macos_recovery: bool) {
    match self.never {}
  }

  pub(crate) fn vz_guest_provisioning_options(&self) -> Result<VZMacGuestProvisioningOptions, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_guest_provisioning(
    &self,
    _guest_provisioning_options: VZMacGuestProvisioningOptions,
  ) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn bridge_set_guest_provisioning_nil(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }
}

pub(crate) struct VZMacPlatformConfiguration {
  never: Never,
  local: Local,
}

pub(crate) fn vz_mac_platform_configuration_init() -> VZMacPlatformConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_mac_platform_configuration_bridge_cast(object: &NSObject) -> Option<VZMacPlatformConfiguration> {
  object.never()
}

impl VZMacPlatformConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZMacPlatformConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_hardware_model(&self) -> VZMacHardwareModel {
    match self.never {}
  }

  pub(crate) fn vz_set_hardware_model(&self, _hardware_model: VZMacHardwareModel) {
    match self.never {}
  }

  pub(crate) fn vz_machine_identifier(&self) -> VZMacMachineIdentifier {
    match self.never {}
  }

  pub(crate) fn vz_set_machine_identifier(&self, _machine_identifier: VZMacMachineIdentifier) {
    match self.never {}
  }

  pub(crate) fn vz_auxiliary_storage(&self) -> Result<VZMacAuxiliaryStorage, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_auxiliary_storage(&self, _auxiliary_storage: VZMacAuxiliaryStorage) {
    match self.never {}
  }

  pub(crate) fn bridge_set_auxiliary_storage_nil(&self) {
    match self.never {}
  }
}
