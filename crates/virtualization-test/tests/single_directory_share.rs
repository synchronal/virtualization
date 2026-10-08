use virtualization::url::Url;
use virtualization::{SharedDirectory, SingleDirectoryShare};
use virtualization_test::directory;

#[test]
fn has_the_directory_it_was_made_with() {
  let url = Url::from_directory_path(directory("has_the_directory_it_was_made_with", &[])).unwrap();
  let share = SingleDirectoryShare::new(&SharedDirectory::new(&url, true).unwrap()).unwrap();

  assert_eq!(share.directory().url().unwrap(), url);
  assert!(share.directory().is_read_only());
}

#[test]
fn a_clone_is_the_same_object() {
  let url = Url::from_directory_path(directory("single_directory_share_a_clone_is_the_same_object", &[])).unwrap();
  let share = SingleDirectoryShare::new(&SharedDirectory::new(&url, false).unwrap()).unwrap();

  assert_eq!(format!("{:?}", share.clone()), format!("{share:?}"));
}

#[test]
fn debugs_as_its_description() {
  let url = Url::from_directory_path(directory("single_directory_share_debugs_as_its_description", &[])).unwrap();
  let share = SingleDirectoryShare::new(&SharedDirectory::new(&url, false).unwrap()).unwrap();

  assert!(format!("{share:?}").starts_with("<VZSingleDirectoryShare: 0x"));
}
