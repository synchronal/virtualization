use virtualization::{
  EFISignature, EFISignatureDatabaseConfiguration, EFISignatureList, EFIVariableStore,
  EFIVariableStoreInitializationOptions, Error, SecCertificate,
};
use virtualization_test::{certificate, file_url, new_file_url, variable_store};

/// Swift's `[]`.
const NO_OPTIONS: EFIVariableStoreInitializationOptions = EFIVariableStoreInitializationOptions { raw_value: 0 };

fn ns_error<T: std::fmt::Debug>(result: Result<T, Error>) -> Box<virtualization::NSError> {
  match result {
    Err(Error::NSError(error)) => error,
    other => panic!("expected Virtualization's error, got {other:?}"),
  }
}

#[test]
fn creates_a_variable_store_at_its_url() {
  let url = new_file_url("creates_a_variable_store_at_its_url.efivars");

  let store = EFIVariableStore::with_creating_variable_store_at_and_options(&url, NO_OPTIONS).unwrap();

  assert_eq!(store.url().unwrap(), url);
  assert!(url.to_file_path().unwrap().exists());
}

#[test]
fn fails_as_virtualization_does_to_create_one_where_a_file_exists() {
  let url = new_file_url("fails_to_create_one_where_a_file_exists.efivars");

  EFIVariableStore::with_creating_variable_store_at_and_options(&url, NO_OPTIONS).unwrap();

  let error = ns_error(EFIVariableStore::with_creating_variable_store_at_and_options(
    &url, NO_OPTIONS,
  ));

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 17, "{error:#?}");
}

#[test]
fn overwrites_a_file_when_allowed() {
  let url = new_file_url("overwrites_a_file_when_allowed.efivars");

  EFIVariableStore::with_creating_variable_store_at_and_options(&url, NO_OPTIONS).unwrap();

  let store = EFIVariableStore::with_creating_variable_store_at_and_options(
    &url,
    EFIVariableStoreInitializationOptions::ALLOW_OVERWRITE,
  )
  .unwrap();

  assert_eq!(store.url().unwrap(), url);
}

#[test]
fn fails_as_virtualization_does_to_create_one_in_a_missing_directory() {
  let error = ns_error(EFIVariableStore::with_creating_variable_store_at_and_options(
    &file_url("/nowhere/efivars"),
    NO_OPTIONS,
  ));

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn opens_a_file_it_does_not_read_yet() {
  let store = EFIVariableStore::new(&file_url("/nowhere/efivars")).unwrap();

  assert_eq!(store.url().unwrap(), file_url("/nowhere/efivars"));

  let error = ns_error(store.is_secure_boot_enabled());

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn opens_a_variable_store_another_created() {
  let created = variable_store("opens_a_variable_store_another_created");

  created
    .enable_secure_boot_using_default_platform_key()
    .unwrap();

  let opened = EFIVariableStore::new(&created.url().unwrap()).unwrap();

  assert!(opened.is_secure_boot_enabled().unwrap());
}

#[test]
fn has_secure_boot_disabled_when_created() {
  let store = variable_store("has_secure_boot_disabled_when_created");

  assert!(!store.is_secure_boot_enabled().unwrap());
}

#[test]
fn enables_and_disables_secure_boot_with_the_default_platform_key() {
  let store = variable_store("enables_and_disables_secure_boot_with_the_default_platform_key");

  store
    .enable_secure_boot_using_default_platform_key()
    .unwrap();

  assert!(store.is_secure_boot_enabled().unwrap());

  store.disable_secure_boot().unwrap();

  assert!(!store.is_secure_boot_enabled().unwrap());
}

#[test]
fn enables_secure_boot_with_a_platform_key_and_resets_it() {
  let name = "enables_secure_boot_with_a_platform_key_and_resets_it";
  let store = variable_store(name);
  let platform_key = SecCertificate::create_with_data(&certificate(name))
    .unwrap()
    .unwrap();

  store.enable_secure_boot(&platform_key).unwrap();

  assert!(store.is_secure_boot_enabled().unwrap());

  store.reset_secure_boot().unwrap();

  assert!(!store.is_secure_boot_enabled().unwrap());
}

/// How many signatures each of a configuration's databases has: KEK, db, dbx.
fn counts(signatures: &EFISignatureDatabaseConfiguration) -> [usize; 3] {
  [
    signatures.key_exchange_keys(),
    signatures.db_signatures(),
    signatures.dbx_signatures(),
  ]
  .map(|lists| lists.iter().map(|list| list.signatures().len()).sum())
}

#[test]
fn has_no_signatures_enrolled_when_created() {
  let store = variable_store("has_no_signatures_enrolled_when_created");

  assert_eq!(counts(&store.enrolled_secure_boot_signatures().unwrap()), [0, 0, 0]);
}

#[test]
fn enrolls_the_default_signatures() {
  let store = variable_store("enrolls_the_default_signatures");

  store.enroll_default_secure_boot_signatures().unwrap();

  let [key_exchange_keys, db_signatures, dbx_signatures] = counts(&store.enrolled_secure_boot_signatures().unwrap());

  assert!(key_exchange_keys > 0);
  assert!(db_signatures > 0);
  assert!(dbx_signatures > 0);
}

#[test]
fn enrolls_the_signatures_it_is_given_once() {
  let name = "enrolls_the_signatures_it_is_given_once";
  let store = variable_store(name);
  let data = certificate(name);
  let key = EFISignature::X509Certificate(SecCertificate::create_with_data(&data).unwrap().unwrap());
  let keys = [EFISignatureList::with_signatures(&[key]).unwrap()];
  let signatures = EFISignatureDatabaseConfiguration::new(
    &keys,
    &keys,
    &[EFISignatureList::with_signatures(&[EFISignature::Sha256(vec![7; 32])]).unwrap()],
  )
  .unwrap();

  store.enroll_secure_boot_signatures(&signatures).unwrap();
  store.enroll_secure_boot_signatures(&signatures).unwrap();

  let enrolled = store.enrolled_secure_boot_signatures().unwrap();

  assert_eq!(counts(&enrolled), [1, 1, 1]);
  assert!(matches!(
    &enrolled.key_exchange_keys()[0].signatures()[0],
    EFISignature::X509Certificate(certificate) if certificate.copy_data() == data
  ));
  assert!(matches!(
    &enrolled.dbx_signatures()[0].signatures()[0],
    EFISignature::Sha256(hash) if hash == &[7; 32]
  ));
}

#[test]
fn a_clone_is_the_same_object() {
  let store = variable_store("efi_variable_store_a_clone_is_the_same_object");
  let clone = store.clone();

  drop(store);

  assert!(!clone.is_secure_boot_enabled().unwrap());
}

#[test]
fn debugs_as_its_description() {
  let store = variable_store("efi_variable_store_debugs_as_its_description");

  assert!(format!("{store:?}").starts_with("<VZEFIVariableStore: 0x"));
}
