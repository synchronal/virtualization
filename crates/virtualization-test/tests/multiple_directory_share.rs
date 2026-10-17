use std::collections::BTreeMap;

use virtualization::url::Url;
use virtualization::{ErrorCode, MultipleDirectoryShare, SharedDirectory};
use virtualization_test::{directory, ns_error};

#[test]
fn has_no_directories_when_new() {
  assert!(
    MultipleDirectoryShare::new()
      .unwrap()
      .directories()
      .is_empty()
  );
}

#[test]
fn has_the_directories_it_was_made_with() {
  let first = Url::from_directory_path(directory("has_the_directories_it_was_made_with_first", &[])).unwrap();
  let second = Url::from_directory_path(directory("has_the_directories_it_was_made_with_second", &[])).unwrap();
  let share = MultipleDirectoryShare::with_directories(&BTreeMap::from([
    ("first".to_string(), SharedDirectory::new(&first, false).unwrap()),
    ("second".to_string(), SharedDirectory::new(&second, true).unwrap()),
  ]))
  .unwrap();

  let directories: Vec<_> = share
    .directories()
    .into_iter()
    .map(|(name, directory)| (name, directory.url().unwrap(), directory.is_read_only()))
    .collect();

  assert_eq!(
    directories,
    [
      ("first".to_string(), first, false),
      ("second".to_string(), second, true)
    ]
  );
}

#[test]
fn validates_a_plain_name() {
  assert!(MultipleDirectoryShare::validate_name("name").is_ok());
}

#[test]
fn fails_as_virtualization_does_on_an_empty_name() {
  let error = ns_error(MultipleDirectoryShare::validate_name(""));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn fails_as_virtualization_does_on_a_name_with_a_slash() {
  let error = ns_error(MultipleDirectoryShare::validate_name("a/b"));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

/// Virtualization canonicalizes a name by dropping what it can't have.
#[test]
fn canonicalizes_a_name() {
  assert_eq!(
    MultipleDirectoryShare::canonicalized_name("name").unwrap(),
    Some("name".to_string())
  );
  assert_eq!(
    MultipleDirectoryShare::canonicalized_name("a/b").unwrap(),
    Some("ab".to_string())
  );
  assert_eq!(MultipleDirectoryShare::canonicalized_name("").unwrap(), None);
}

#[test]
fn a_clone_is_the_same_object() {
  let share = MultipleDirectoryShare::new().unwrap();

  assert_eq!(format!("{:?}", share.clone()), format!("{share:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", MultipleDirectoryShare::new().unwrap()).starts_with("<VZMultipleDirectoryShare: 0x"));
}
