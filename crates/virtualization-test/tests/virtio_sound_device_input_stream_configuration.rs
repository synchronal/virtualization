use virtualization::{HostAudioInputStreamSource, VirtioSoundDeviceInputStreamConfiguration};

#[test]
fn has_no_source_when_new() {
  assert!(
    VirtioSoundDeviceInputStreamConfiguration::new()
      .unwrap()
      .source::<HostAudioInputStreamSource>()
      .is_none()
  );
}

#[test]
fn has_the_source_it_was_given() {
  let stream = VirtioSoundDeviceInputStreamConfiguration::new().unwrap();
  let source = HostAudioInputStreamSource::new().unwrap();

  stream.set_source(Some(&source));

  assert_eq!(
    format!("{:?}", stream.source::<HostAudioInputStreamSource>().unwrap()),
    format!("{source:?}")
  );

  stream.set_source(None);

  assert!(stream.source::<HostAudioInputStreamSource>().is_none());
}

#[test]
fn a_clone_is_the_same_object() {
  let stream = VirtioSoundDeviceInputStreamConfiguration::new().unwrap();

  stream
    .clone()
    .set_source(Some(&HostAudioInputStreamSource::new().unwrap()));

  assert!(stream.source::<HostAudioInputStreamSource>().is_some());
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioSoundDeviceInputStreamConfiguration::new().unwrap())
      .starts_with("<VZVirtioSoundDeviceInputStreamConfiguration: 0x")
  );
}
