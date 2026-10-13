use super::{GUARDED, Never};

pub(crate) struct SecCertificate {
  never: Never,
}

pub(crate) fn sec_certificate_create_with_data(_data: &[u8]) -> Option<SecCertificate> {
  unreachable!("{GUARDED}")
}

impl SecCertificate {
  pub(crate) fn bridge_retained(&self) -> SecCertificate {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn sec_certificate_copy_data(&self) -> Vec<u8> {
    match self.never {}
  }
}
