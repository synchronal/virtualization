use super::{GUARDED, NSURL, Never, Retained};

pub(crate) struct VZLinuxBootLoader {
  never: Never,
}

pub(crate) fn vz_linux_boot_loader_init(_kernel_url: &NSURL) -> VZLinuxBootLoader {
  unreachable!("{GUARDED}")
}

impl VZLinuxBootLoader {
  pub(crate) fn bridge_retained(&self) -> VZLinuxBootLoader {
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
