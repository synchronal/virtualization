use virtualization::{EFISignature, EFISignatureDatabaseConfiguration, EFISignatureList, SecCertificate};
use virtualization_test::certificate;

/// A list of one hash, of 32 of `byte`.
fn hashes(byte: u8) -> EFISignatureList {
  EFISignatureList::with_signatures(&[EFISignature::Sha256(vec![byte; 32])]).unwrap()
}

/// The first hash of the only list of `lists`.
fn hash(lists: &[EFISignatureList]) -> Vec<u8> {
  assert_eq!(lists.len(), 1);

  match lists[0].signatures().first() {
    Some(EFISignature::Sha256(hash)) => hash.clone(),
    other => panic!("expected a hash, got {other:?}"),
  }
}

#[test]
fn has_the_signature_lists_it_was_made_with() {
  let data = certificate("has_the_signature_lists_it_was_made_with");
  let key = EFISignature::X509Certificate(SecCertificate::create_with_data(&data).unwrap().unwrap());
  let configuration = EFISignatureDatabaseConfiguration::new(
    &[EFISignatureList::with_signatures(&[key]).unwrap()],
    &[hashes(1)],
    &[hashes(2)],
  )
  .unwrap();

  let key_exchange_keys = configuration.key_exchange_keys();

  assert_eq!(key_exchange_keys.len(), 1);
  assert!(matches!(
    &key_exchange_keys[0].signatures()[0],
    EFISignature::X509Certificate(certificate) if certificate.copy_data() == data
  ));
  assert_eq!(hash(&configuration.db_signatures()), [1; 32]);
  assert_eq!(hash(&configuration.dbx_signatures()), [2; 32]);
}

#[test]
fn has_no_signature_lists_when_made_with_none() {
  let configuration = EFISignatureDatabaseConfiguration::new(&[], &[], &[]).unwrap();

  assert!(configuration.key_exchange_keys().is_empty());
  assert!(configuration.db_signatures().is_empty());
  assert!(configuration.dbx_signatures().is_empty());
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<EFISignatureDatabaseConfiguration>();
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = EFISignatureDatabaseConfiguration::new(&[], &[hashes(1)], &[]).unwrap();

  assert_eq!(format!("{:?}", configuration.clone()), format!("{configuration:?}"));
}

#[test]
fn debugs_as_its_description() {
  let configuration = EFISignatureDatabaseConfiguration::new(&[], &[], &[]).unwrap();

  assert!(format!("{configuration:?}").starts_with("<VZEFISignatureDatabaseConfiguration: 0x"));
}
