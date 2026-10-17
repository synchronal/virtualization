use virtualization::{ErrorCode, MacGraphicsDeviceConfiguration, MacGraphicsDisplayConfiguration};
use virtualization_test::{linux_configuration, ns_error};

#[test]
fn has_no_displays_when_new() {
  assert!(
    MacGraphicsDeviceConfiguration::new()
      .unwrap()
      .displays()
      .is_empty()
  );
}

#[test]
fn has_the_displays_it_was_given() {
  let device = MacGraphicsDeviceConfiguration::new().unwrap();
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  device.set_displays(&[&display]);
  let displays = device.displays();

  assert_eq!(displays.len(), 1);
  assert_eq!(format!("{:?}", displays[0]), format!("{display:?}"));
}

/// Only a machine on the macOS platform can have one.
#[test]
fn fails_as_virtualization_does_to_validate_on_another_platform() {
  let configuration = linux_configuration("fails_as_virtualization_does_to_validate_on_another_platform");
  let device = MacGraphicsDeviceConfiguration::new().unwrap();

  device.set_displays(&[&MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap()]);
  configuration.set_graphics_devices(&[&device]);

  assert!(matches!(
    configuration.graphics_devices::<MacGraphicsDeviceConfiguration>()[..],
    [Some(_)]
  ));
  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let device = MacGraphicsDeviceConfiguration::new().unwrap();

  device
    .clone()
    .set_displays(&[&MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap()]);

  assert_eq!(device.displays().len(), 1);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", MacGraphicsDeviceConfiguration::new().unwrap())
      .starts_with("<VZMacGraphicsDeviceConfiguration: 0x")
  );
}
