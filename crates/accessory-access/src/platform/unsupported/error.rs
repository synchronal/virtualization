use super::GUARDED;

pub(crate) enum AAErrorCode {
  InternalError,
  AccessoryListenerAlreadyRegistered,
  AccessoryNotAccessible,
  InvalidAccessoryState,
  Unknown(isize),
}

pub(crate) fn aa_error_domain() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn aa_error_code_init_raw_value(_raw_value: isize) -> AAErrorCode {
  unreachable!("{GUARDED}")
}

pub(crate) fn aa_error_code_raw_value(_code: AAErrorCode) -> isize {
  unreachable!("{GUARDED}")
}
