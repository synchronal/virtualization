use super::{GUARDED, Never};

pub(crate) type BridgedNil = Never;
pub(crate) type BridgedThrown = Never;

pub(crate) struct NSURL {
  never: Never,
}

pub(crate) fn ns_url_init_string(_string: &str) -> Result<NSURL, BridgedNil> {
  unreachable!("{GUARDED}")
}

impl NSURL {
  pub(crate) fn ns_absolute_string(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct NSError {
  never: Never,
}

pub(crate) fn bridge_take_thrown_error() -> NSError {
  unreachable!("{GUARDED}")
}

impl NSError {
  pub(crate) fn ns_domain(&self) -> String {
    match self.never {}
  }

  pub(crate) fn ns_code(&self) -> isize {
    match self.never {}
  }

  pub(crate) fn ns_localized_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn ns_localized_failure_reason(&self) -> Option<String> {
    match self.never {}
  }

  pub(crate) fn ns_localized_recovery_suggestion(&self) -> Option<String> {
    match self.never {}
  }

  pub(crate) fn bridge_user_info_count(&self) -> usize {
    match self.never {}
  }

  pub(crate) fn bridge_user_info_key_at(&self, _index: usize) -> String {
    match self.never {}
  }

  pub(crate) fn bridge_user_info_value_description_at(&self, _index: usize) -> String {
    match self.never {}
  }

  pub(crate) fn bridge_underlying_errors_count(&self) -> usize {
    match self.never {}
  }

  pub(crate) fn bridge_underlying_error_at(&self, _index: usize) -> NSError {
    match self.never {}
  }
}
