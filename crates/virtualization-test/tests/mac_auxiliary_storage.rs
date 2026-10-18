use virtualization::{MacAuxiliaryStorage, MacAuxiliaryStorageInitializationOptions};
use virtualization_test::{file_url, mac_hardware_model, new_file_url, ns_error};

#[test]
fn creates_auxiliary_storage_at_its_url() {
  let url = new_file_url("creates_auxiliary_storage_at_its_url.aux");

  let storage = MacAuxiliaryStorage::with_creating_storage_at_and_hardware_model_and_options(
    &url,
    &mac_hardware_model(),
    MacAuxiliaryStorageInitializationOptions::new(0),
  )
  .unwrap();

  assert_eq!(storage.url().unwrap(), url);
  assert!(url.to_file_path().unwrap().exists());
}

#[test]
fn fails_as_virtualization_does_to_create_it_where_a_file_exists() {
  let url = new_file_url("fails_as_virtualization_does_to_create_it_where_a_file_exists.aux");
  let hardware_model = mac_hardware_model();

  MacAuxiliaryStorage::with_creating_storage_at_and_hardware_model_and_options(
    &url,
    &hardware_model,
    MacAuxiliaryStorageInitializationOptions::new(0),
  )
  .unwrap();

  let error = ns_error(
    MacAuxiliaryStorage::with_creating_storage_at_and_hardware_model_and_options(
      &url,
      &hardware_model,
      MacAuxiliaryStorageInitializationOptions::new(0),
    ),
  );

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 17, "{error:#?}");
}

#[test]
fn overwrites_a_file_when_allowed() {
  let url = new_file_url("mac_auxiliary_storage_overwrites_a_file_when_allowed.aux");
  let hardware_model = mac_hardware_model();

  MacAuxiliaryStorage::with_creating_storage_at_and_hardware_model_and_options(
    &url,
    &hardware_model,
    MacAuxiliaryStorageInitializationOptions::new(0),
  )
  .unwrap();

  let storage = MacAuxiliaryStorage::with_creating_storage_at_and_hardware_model_and_options(
    &url,
    &hardware_model,
    MacAuxiliaryStorageInitializationOptions::allow_overwrite().unwrap(),
  )
  .unwrap();

  assert_eq!(storage.url().unwrap(), url);
}

#[test]
fn allows_overwriting_with_its_first_bit() {
  assert_eq!(
    MacAuxiliaryStorageInitializationOptions::allow_overwrite()
      .unwrap()
      .raw_value,
    1 << 0
  );
}

/// Virtualization reads the file only when a machine uses it.
#[test]
fn opens_any_url_it_is_given() {
  let url = file_url("/nonexistent/auxiliary-storage");

  assert_eq!(MacAuxiliaryStorage::new(&url).unwrap().url().unwrap(), url);
}

#[test]
fn a_clone_is_the_same_object() {
  let storage = MacAuxiliaryStorage::new(&file_url("/nonexistent/auxiliary-storage")).unwrap();

  assert_eq!(format!("{:?}", storage.clone()), format!("{storage:?}"));
}

#[test]
fn debugs_as_its_description() {
  let storage = MacAuxiliaryStorage::new(&file_url("/nonexistent/auxiliary-storage")).unwrap();

  assert!(format!("{storage:?}").starts_with("<VZMacAuxiliaryStorage: 0x"));
}
