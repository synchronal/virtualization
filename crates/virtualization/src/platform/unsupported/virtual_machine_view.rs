use std::ffi::c_void;

use super::{BridgedNil, GUARDED, Local, MainThreadMarker, Never, VZVirtualMachine};

pub(crate) struct VZVirtualMachineView {
  never: Never,
  local: Local,
}

pub(crate) fn vz_virtual_machine_view_init(mtm: MainThreadMarker) -> VZVirtualMachineView {
  mtm.never()
}

impl VZVirtualMachineView {
  pub(crate) fn bridge_retained(&self) -> VZVirtualMachineView {
    match self.never {}
  }

  pub(crate) fn bridge_borrowed(&self) -> *const c_void {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_virtual_machine(&self) -> Result<VZVirtualMachine, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_virtual_machine(&self, _virtual_machine: VZVirtualMachine) {
    match self.never {}
  }

  pub(crate) fn bridge_set_virtual_machine_nil(&self) {
    match self.never {}
  }

  pub(crate) fn vz_captures_system_keys(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_captures_system_keys(&self, _captures_system_keys: bool) {
    match self.never {}
  }

  pub(crate) fn vz_automatically_reconfigures_display(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_automatically_reconfigures_display(&self, _automatically_reconfigures_display: bool) {
    match self.never {}
  }

  pub(crate) fn vz_adaptor(&self) -> Result<BridgedVZVirtualMachineViewAdaptor, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_adaptor(&self, _adaptor: BridgedVZVirtualMachineViewAdaptor) {
    match self.never {}
  }

  pub(crate) fn bridge_set_adaptor_nil(&self) {
    match self.never {}
  }
}

pub(crate) struct BridgedVZVirtualMachineViewAdaptor {
  never: Never,
}

pub(crate) fn vz_virtual_machine_view_adaptor_init_virtual_machine(
  _virtual_machine: &VZVirtualMachine,
) -> BridgedVZVirtualMachineViewAdaptor {
  unreachable!("{GUARDED}")
}

impl BridgedVZVirtualMachineViewAdaptor {
  pub(crate) fn bridge_retained(&self) -> BridgedVZVirtualMachineViewAdaptor {
    match self.never {}
  }

  pub(crate) fn bridge_description(&self) -> String {
    match self.never {}
  }
}
