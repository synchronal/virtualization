use virtualization::{ErrorCode, GuestProvisioningOptions, MacGuestProvisioningOptions};
use virtualization_test::ns_error;

/// Options that validate: a full name, a username, and a password.
fn options() -> MacGuestProvisioningOptions {
  let options = MacGuestProvisioningOptions::new().unwrap();

  options.set_full_name("Test User");
  options.set_username("test");
  options.set_password("password");

  options
}

#[test]
fn is_empty_when_new() {
  let options = MacGuestProvisioningOptions::new().unwrap();

  assert_eq!(options.full_name(), "");
  assert_eq!(options.username(), "");
  assert_eq!(options.password(), "");
  assert!(!options.logs_in_automatically());
  assert!(!options.enables_remote_login());
}

#[test]
fn has_the_values_it_was_given() {
  let options = options();

  options.set_logs_in_automatically(true);
  options.set_enables_remote_login(true);

  assert_eq!(options.full_name(), "Test User");
  assert_eq!(options.username(), "test");
  assert_eq!(options.password(), "password");
  assert!(options.logs_in_automatically());
  assert!(options.enables_remote_login());
}

#[test]
fn validates_with_a_full_name_username_and_password() {
  options().validate().unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_without_a_password() {
  let options = options();

  options.set_password("");
  let error = ns_error(options.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::GuestProvisioningInvalidPassword,
    "{error:#?}"
  );
}

#[test]
fn fails_validation_as_virtualization_does_without_a_full_name() {
  let options = options();

  options.set_full_name("");
  let error = ns_error(options.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::GuestProvisioningInvalidFullName,
    "{error:#?}"
  );
}

#[test]
fn fails_validation_as_virtualization_does_without_a_username() {
  let options = options();

  options.set_username("");
  let error = ns_error(options.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::GuestProvisioningInvalidUsername,
    "{error:#?}"
  );
}

#[test]
fn fails_validation_as_virtualization_does_on_a_username_macos_rejects() {
  let options = options();

  options.set_username("Not A Short Name!");
  let error = ns_error(options.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::GuestProvisioningInvalidUsername,
    "{error:#?}"
  );
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<MacGuestProvisioningOptions>();
}

#[test]
fn a_clone_is_the_same_object() {
  let options = MacGuestProvisioningOptions::new().unwrap();

  options.clone().set_username("test");

  assert_eq!(options.username(), "test");
}

#[test]
fn debugs_as_its_description() {
  let options = MacGuestProvisioningOptions::new().unwrap();

  assert!(format!("{options:?}").starts_with("<VZMacGuestProvisioningOptions: 0x"));
}
