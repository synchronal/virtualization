use virtualization::SharedDirectory;
use virtualization::url::Url;
use virtualization_test::directory;

#[test]
fn has_the_url_and_read_only_it_was_made_with() {
  let url = Url::from_directory_path(directory("has_the_url_and_read_only_it_was_made_with", &[])).unwrap();

  let writable = SharedDirectory::new(&url, false).unwrap();
  let read_only = SharedDirectory::new(&url, true).unwrap();

  assert_eq!(writable.url().unwrap(), url);
  assert!(!writable.is_read_only());
  assert!(read_only.is_read_only());
}

#[test]
fn a_clone_is_the_same_object() {
  let url = Url::from_directory_path(directory("shared_directory_a_clone_is_the_same_object", &[])).unwrap();
  let shared_directory = SharedDirectory::new(&url, true).unwrap();

  assert_eq!(
    format!("{:?}", shared_directory.clone()),
    format!("{shared_directory:?}")
  );
}

#[test]
fn debugs_as_its_description() {
  let url = Url::from_directory_path(directory("shared_directory_debugs_as_its_description", &[])).unwrap();

  assert!(format!("{:?}", SharedDirectory::new(&url, false).unwrap()).starts_with("<VZSharedDirectory: 0x"));
}
