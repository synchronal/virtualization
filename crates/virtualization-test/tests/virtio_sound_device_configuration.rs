use virtualization::{
  VirtioSoundDeviceConfiguration, VirtioSoundDeviceInputStreamConfiguration, VirtioSoundDeviceOutputStreamConfiguration,
};
use virtualization_test::{guest_pci_devices, linux_configuration};

#[test]
fn has_no_streams_when_new() {
  assert!(
    VirtioSoundDeviceConfiguration::new()
      .unwrap()
      .streams::<VirtioSoundDeviceInputStreamConfiguration>()
      .is_empty()
  );
}

#[test]
fn has_the_streams_it_was_given() {
  let device = VirtioSoundDeviceConfiguration::new().unwrap();

  device.set_streams(&[
    &VirtioSoundDeviceInputStreamConfiguration::new().unwrap(),
    &VirtioSoundDeviceOutputStreamConfiguration::new().unwrap(),
  ]);

  assert!(matches!(
    device.streams::<VirtioSoundDeviceInputStreamConfiguration>()[..],
    [Some(_), None]
  ));
  assert!(matches!(
    device.streams::<VirtioSoundDeviceOutputStreamConfiguration>()[..],
    [None, Some(_)]
  ));
}

#[test]
fn is_an_audio_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_an_audio_device_of_the_configuration_it_was_given_to");
  let device = VirtioSoundDeviceConfiguration::new().unwrap();

  device.set_streams(&[&VirtioSoundDeviceOutputStreamConfiguration::new().unwrap()]);
  configuration.set_audio_devices(&[&device]);

  assert!(matches!(
    configuration.audio_devices::<VirtioSoundDeviceConfiguration>()[..],
    [Some(_)]
  ));
  configuration.validate().unwrap();
}

/// The suites' kernel has no virtio sound driver, so the guest has the device
/// by its PCI vendor (Red Hat's virtio) and device (sound), and no sound card.
#[test]
fn a_guest_has_a_virtio_sound_device() {
  let devices = guest_pci_devices("a_guest_has_a_virtio_sound_device", |configuration| {
    let device = VirtioSoundDeviceConfiguration::new().unwrap();

    device.set_streams(&[&VirtioSoundDeviceOutputStreamConfiguration::new().unwrap()]);
    configuration.set_audio_devices(&[&device]);
  });

  assert!(devices.contains(&"0x1af4:0x1059:0x040100".to_string()), "{devices:?}");
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioSoundDeviceConfiguration::new().unwrap();

  device
    .clone()
    .set_streams(&[&VirtioSoundDeviceOutputStreamConfiguration::new().unwrap()]);

  assert_eq!(
    device
      .streams::<VirtioSoundDeviceOutputStreamConfiguration>()
      .len(),
    1
  );
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioSoundDeviceConfiguration::new().unwrap())
      .starts_with("<VZVirtioSoundDeviceConfiguration: 0x")
  );
}
