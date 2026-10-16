use virtualization::HostAudioInputStreamSource;

#[test]
fn a_clone_is_the_same_object() {
  let object = HostAudioInputStreamSource::new().unwrap();

  assert_eq!(format!("{:?}", object.clone()), format!("{object:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", HostAudioInputStreamSource::new().unwrap()).starts_with("<VZHostAudioInputStreamSource: 0x"));
}
