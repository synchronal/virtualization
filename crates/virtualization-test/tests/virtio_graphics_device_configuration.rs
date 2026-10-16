use virtualization::{VirtioGraphicsDeviceConfiguration, VirtioGraphicsScanoutConfiguration};
use virtualization_test::{guest_pci_devices, linux_configuration, ns_error};

#[test]
fn has_no_scanouts_when_new() {
  assert!(
    VirtioGraphicsDeviceConfiguration::new()
      .unwrap()
      .scanouts()
      .is_empty()
  );
}

#[test]
fn has_the_scanouts_it_was_given() {
  let device = VirtioGraphicsDeviceConfiguration::new().unwrap();
  let scanout = VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap();

  device.set_scanouts(&[&scanout]);
  let scanouts = device.scanouts();

  assert_eq!(scanouts.len(), 1);
  assert_eq!(format!("{:?}", scanouts[0]), format!("{scanout:?}"));
}

#[test]
fn is_a_graphics_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_graphics_device_of_the_configuration_it_was_given_to");
  let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

  device.set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);
  configuration.set_graphics_devices(&[&device]);

  assert!(matches!(
    configuration.graphics_devices::<VirtioGraphicsDeviceConfiguration>()[..],
    [Some(_)]
  ));
  configuration.validate().unwrap();
}

#[test]
fn fails_as_virtualization_does_to_validate_with_no_scanouts() {
  let configuration = linux_configuration("fails_as_virtualization_does_to_validate_with_no_scanouts");

  configuration.set_graphics_devices(&[&VirtioGraphicsDeviceConfiguration::new().unwrap()]);
  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

/// The suites' kernel has no virtio GPU driver, so the guest has the device
/// by its PCI vendor (Red Hat's virtio) and device (GPU), and no display.
#[test]
fn a_guest_has_a_virtio_gpu() {
  let devices = guest_pci_devices("a_guest_has_a_virtio_gpu", |configuration| {
    let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

    device.set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);
    configuration.set_graphics_devices(&[&device]);
  });

  assert!(devices.contains(&"0x1af4:0x1050:0x038000".to_string()), "{devices:?}");
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

  device
    .clone()
    .set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);

  assert_eq!(device.scanouts().len(), 1);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioGraphicsDeviceConfiguration::new().unwrap())
      .starts_with("<VZVirtioGraphicsDeviceConfiguration: 0x")
  );
}
