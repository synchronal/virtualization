use virtualization::{EFISignature, EFISignatureList, Error, SecCertificate};
use virtualization_test::{certificate, file, file_url};

#[test]
fn has_the_signatures_it_was_made_with() {
  let data = certificate("has_the_signatures_it_was_made_with");
  let list = EFISignatureList::with_signatures(&[
    EFISignature::Sha256(vec![7; 32]),
    EFISignature::X509Certificate(SecCertificate::create_with_data(&data).unwrap().unwrap()),
  ])
  .unwrap();

  let signatures = list.signatures();

  assert_eq!(signatures.len(), 2);
  assert!(matches!(&signatures[0], EFISignature::Sha256(hash) if hash == &[7; 32]));
  assert!(matches!(
    &signatures[1],
    EFISignature::X509Certificate(certificate) if certificate.copy_data() == data
  ));
}

#[test]
fn has_no_signatures_when_made_with_none() {
  assert!(
    EFISignatureList::with_signatures(&[])
      .unwrap()
      .signatures()
      .is_empty()
  );
}

#[test]
fn reads_a_certificate_from_a_file() {
  let data = certificate("reads_a_certificate_from_a_file");
  let list = EFISignatureList::new(&file("reads_a_certificate_from_a_file.cer", &data)).unwrap();

  let signatures = list.signatures();

  assert_eq!(signatures.len(), 1);
  assert!(matches!(
    &signatures[0],
    EFISignature::X509Certificate(certificate) if certificate.copy_data() == data
  ));
}

#[test]
fn fails_as_virtualization_does_to_read_a_missing_file() {
  let Err(Error::NSError(error)) = EFISignatureList::new(&file_url("/nowhere/signatures.esl")) else {
    panic!("read a missing file");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 1, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_read_a_file_of_no_format_it_knows() {
  let url = file("fails_to_read_a_file_of_no_format_it_knows.esl", b"not signatures");

  let Err(Error::NSError(error)) = EFISignatureList::new(&url) else {
    panic!("read signatures from a file with none");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 1, "{error:#?}");
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<EFISignatureList>();
  assert_send_sync::<EFISignature>();
}

#[test]
fn a_clone_is_the_same_object() {
  let list = EFISignatureList::with_signatures(&[EFISignature::Sha256(vec![7; 32])]).unwrap();

  assert_eq!(format!("{:?}", list.clone()), format!("{list:?}"));
}

#[test]
fn debugs_as_its_description() {
  let list = EFISignatureList::with_signatures(&[]).unwrap();

  assert!(format!("{list:?}").starts_with("<VZEFISignatureList: 0x"));
}
