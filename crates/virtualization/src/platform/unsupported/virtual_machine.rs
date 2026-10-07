use super::{BridgedThrown, GUARDED, NSObject, Never, Retained};

pub(crate) fn vz_virtual_machine_is_supported() -> bool {
  unreachable!("{GUARDED}")
}

pub(crate) struct VZVirtualMachineConfiguration {
  never: Never,
}

pub(crate) fn vz_virtual_machine_configuration_init() -> VZVirtualMachineConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtual_machine_configuration_minimum_allowed_memory_size() -> u64 {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtual_machine_configuration_maximum_allowed_memory_size() -> u64 {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtual_machine_configuration_minimum_allowed_cpu_count() -> isize {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtual_machine_configuration_maximum_allowed_cpu_count() -> isize {
  unreachable!("{GUARDED}")
}

impl VZVirtualMachineConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtualMachineConfiguration {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_boot_loader(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_boot_loader(&self, _boot_loader: Option<&NSObject>) {
    match self.never {}
  }

  pub(crate) fn vz_memory_size(&self) -> u64 {
    match self.never {}
  }

  pub(crate) fn vz_set_memory_size(&self, _memory_size: u64) {
    match self.never {}
  }

  pub(crate) fn vz_cpu_count(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn vz_set_cpu_count(&self, _cpu_count: isize) {
    match self.never {}
  }

  pub(crate) fn vz_platform(&self) -> Retained<NSObject> {
    match self.never {}
  }

  pub(crate) fn vz_set_platform(&self, _platform: &NSObject) {
    match self.never {}
  }

  pub(crate) fn vz_audio_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_audio_devices(&self, _audio_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_console_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_console_devices(&self, _console_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_directory_sharing_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_directory_sharing_devices(&self, _directory_sharing_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_entropy_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_entropy_devices(&self, _entropy_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_memory_balloon_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_memory_balloon_devices(&self, _memory_balloon_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_network_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_network_devices(&self, _network_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_serial_ports(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_serial_ports(&self, _serial_ports: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_socket_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_socket_devices(&self, _socket_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_storage_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_storage_devices(&self, _storage_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_keyboards(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_keyboards(&self, _keyboards: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_pointing_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_pointing_devices(&self, _pointing_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_graphics_devices(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_graphics_devices(&self, _graphics_devices: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_usb_controllers(&self) -> Vec<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_usb_controllers(&self, _usb_controllers: &[&NSObject]) {
    match self.never {}
  }

  pub(crate) fn vz_label(&self) -> Option<String> {
    match self.never {}
  }

  pub(crate) fn vz_set_label(&self, _label: Option<&str>) {
    match self.never {}
  }

  pub(crate) fn vz_validate(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }

  pub(crate) fn vz_validate_save_restore_support(&self) -> Result<(), BridgedThrown> {
    match self.never {}
  }
}
