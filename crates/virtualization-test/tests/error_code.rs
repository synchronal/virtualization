//! AccessoryAccess's `AAError.Code` and `AAErrorDomain`.

use virtualization::accessory_access::{ErrorCode, error_domain};

#[test]
fn has_accessory_accesss_domain() {
  assert_eq!(error_domain().unwrap(), "AAErrorDomain");
}

#[test]
fn is_each_case_by_its_raw_value() {
  assert_eq!(ErrorCode::new(1).unwrap(), ErrorCode::InternalError);
  assert_eq!(
    ErrorCode::new(2).unwrap(),
    ErrorCode::AccessoryListenerAlreadyRegistered
  );
  assert_eq!(ErrorCode::new(3).unwrap(), ErrorCode::AccessoryNotAccessible);
  assert_eq!(ErrorCode::new(4).unwrap(), ErrorCode::InvalidAccessoryState);
}

#[test]
fn is_unknown_for_a_raw_value_with_no_case() {
  assert_eq!(ErrorCode::new(0).unwrap(), ErrorCode::Unknown(0));
  assert_eq!(ErrorCode::new(5).unwrap(), ErrorCode::Unknown(5));
}

#[test]
fn has_its_raw_value() {
  for raw_value in 1..=4 {
    assert_eq!(ErrorCode::new(raw_value).unwrap().raw_value().unwrap(), raw_value);
  }

  assert_eq!(ErrorCode::Unknown(42).raw_value().unwrap(), 42);
}
