use virtualization::HostAudioOutputStreamSink;

#[test]
fn a_clone_is_the_same_object() {
  let object = HostAudioOutputStreamSink::new().unwrap();

  assert_eq!(format!("{:?}", object.clone()), format!("{object:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", HostAudioOutputStreamSink::new().unwrap()).starts_with("<VZHostAudioOutputStreamSink: 0x"));
}
