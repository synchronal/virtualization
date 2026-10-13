use virtualization::SecCertificate;
use virtualization_test::certificate;

#[test]
fn has_the_data_it_was_created_with() {
  let data = certificate("has_the_data_it_was_created_with");

  let certificate = SecCertificate::create_with_data(&data).unwrap().unwrap();

  assert_eq!(certificate.copy_data(), data);
}

#[test]
fn is_none_from_data_that_is_no_certificate() {
  assert!(
    SecCertificate::create_with_data(b"not a certificate")
      .unwrap()
      .is_none()
  );
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<SecCertificate>();
}

#[test]
fn a_clone_is_the_same_object() {
  let certificate = SecCertificate::create_with_data(&certificate("sec_certificate_a_clone_is_the_same_object"))
    .unwrap()
    .unwrap();

  assert_eq!(format!("{:?}", certificate.clone()), format!("{certificate:?}"));
}

#[test]
fn debugs_as_its_description() {
  let certificate = SecCertificate::create_with_data(&certificate("sec_certificate_debugs_as_its_description"))
    .unwrap()
    .unwrap();

  assert!(format!("{certificate:?}").starts_with("<cert(0x"), "{certificate:?}");
}
