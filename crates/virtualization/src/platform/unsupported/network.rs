use super::{NSObject, Never, Retained};

pub(crate) struct VZNetworkDevice {
  never: Never,
}

impl VZNetworkDevice {
  pub(crate) fn bridge_retained(&self) -> VZNetworkDevice {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_attachment(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_attachment(&self, _attachment: Option<&NSObject>) {
    match self.never {}
  }
}
