use virtualization::{HostAudioOutputStreamSink, VirtioSoundDeviceOutputStreamConfiguration};

#[test]
fn has_no_sink_when_new() {
  assert!(
    VirtioSoundDeviceOutputStreamConfiguration::new()
      .unwrap()
      .sink::<HostAudioOutputStreamSink>()
      .is_none()
  );
}

#[test]
fn has_the_sink_it_was_given() {
  let stream = VirtioSoundDeviceOutputStreamConfiguration::new().unwrap();
  let sink = HostAudioOutputStreamSink::new().unwrap();

  stream.set_sink(Some(&sink));

  assert_eq!(
    format!("{:?}", stream.sink::<HostAudioOutputStreamSink>().unwrap()),
    format!("{sink:?}")
  );

  stream.set_sink(None);

  assert!(stream.sink::<HostAudioOutputStreamSink>().is_none());
}

#[test]
fn a_clone_is_the_same_object() {
  let stream = VirtioSoundDeviceOutputStreamConfiguration::new().unwrap();

  stream
    .clone()
    .set_sink(Some(&HostAudioOutputStreamSink::new().unwrap()));

  assert!(stream.sink::<HostAudioOutputStreamSink>().is_some());
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioSoundDeviceOutputStreamConfiguration::new().unwrap())
      .starts_with("<VZVirtioSoundDeviceOutputStreamConfiguration: 0x")
  );
}
