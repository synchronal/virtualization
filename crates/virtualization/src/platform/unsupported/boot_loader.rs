use super::{BridgedNil, BridgedThrown, GUARDED, NSObject, NSURL, Never, Retained, SecCertificate};

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

pub(crate) struct VZEFIBootLoader {
  never: Never,
}

pub(crate) fn vz_efi_boot_loader_init() -> VZEFIBootLoader {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_efi_boot_loader_bridge_cast(object: &NSObject) -> Option<VZEFIBootLoader> {
  object.never()
}

impl VZEFIBootLoader {
  pub(crate) fn bridge_retained(&self) -> VZEFIBootLoader {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_variable_store(&self) -> Result<VZEFIVariableStore, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_variable_store(&self, _variable_store: VZEFIVariableStore) {
    match self.never {}
  }

  pub(crate) fn bridge_set_variable_store_nil(&self) {
    match self.never {}
  }
}

pub(crate) struct VZEFIVariableStore {
  never: Never,
}

pub(crate) fn vz_efi_variable_store_init(_url: &NSURL) -> VZEFIVariableStore {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_efi_variable_store_init_creating_variable_store_at_options(
  _url: &NSURL,
  _options: usize,
) -> Result<VZEFIVariableStore, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_efi_variable_store_initialization_options_allow_overwrite() -> usize {
  unreachable!("{GUARDED}")
}

impl VZEFIVariableStore {
  pub(crate) fn bridge_retained(&self) -> VZEFIVariableStore {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_enable_secure_boot_using_default_platform_key(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_enable_secure_boot(&self, _platform_key: &SecCertificate) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_disable_secure_boot(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_reset_secure_boot(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_is_secure_boot_enabled(&self) -> Result<bool, BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_enroll_default_secure_boot_signatures(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_enroll_secure_boot_signatures(
    &self,
    _signatures: VZEFISignatureDatabaseConfiguration,
  ) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_enrolled_secure_boot_signatures(
    &self,
  ) -> Result<VZEFISignatureDatabaseConfiguration, BridgedThrown> {
    match self.never {}
  }
}

pub(crate) enum VZEFISignature {
  Sha256(Vec<u8>),
  X509Certificate(SecCertificate),
  Unknown,
}

pub(crate) struct VZEFISignatureList {
  never: Never,
}

pub(crate) fn vz_efi_signature_list_init_contents_of(_url: &NSURL) -> Result<VZEFISignatureList, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_efi_signature_list_init_signatures(_signatures: &[VZEFISignature]) -> VZEFISignatureList {
  unreachable!("{GUARDED}")
}

impl VZEFISignatureList {
  pub(crate) fn bridge_retained(&self) -> VZEFISignatureList {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_signatures(&self) -> Vec<VZEFISignature> {
    match self.never {}
  }
}

pub(crate) struct VZEFISignatureDatabaseConfiguration {
  never: Never,
}

pub(crate) fn vz_efi_signature_database_configuration_init(
  _key_exchange_keys: &[&VZEFISignatureList],
  _db_signatures: &[&VZEFISignatureList],
  _dbx_signatures: &[&VZEFISignatureList],
) -> VZEFISignatureDatabaseConfiguration {
  unreachable!("{GUARDED}")
}

impl VZEFISignatureDatabaseConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZEFISignatureDatabaseConfiguration {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_key_exchange_keys(&self) -> Vec<VZEFISignatureList> {
    match self.never {}
  }

  pub(crate) fn vz_db_signatures(&self) -> Vec<VZEFISignatureList> {
    match self.never {}
  }

  pub(crate) fn vz_dbx_signatures(&self) -> Vec<VZEFISignatureList> {
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
