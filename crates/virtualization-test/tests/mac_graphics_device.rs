use virtualization::{
  GraphicsDevice, MacGraphicsDevice, MacGraphicsDeviceConfiguration, MacGraphicsDisplay,
  MacGraphicsDisplayConfiguration,
};
use virtualization_test::{macos_machine, on_queue};

/// Runs `check` on the graphics device of a macOS machine with one, of one
/// display, on its queue and not started.
fn with_device(name: &str, check: impl FnOnce(&MacGraphicsDevice) + Send) {
  let (queue, machine) = macos_machine(name, |configuration| {
    let device = MacGraphicsDeviceConfiguration::new().unwrap();

    device.set_displays(&[&MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap()]);
    configuration.set_graphics_devices(&[&device]);
  });

  on_queue(&queue, || {
    let devices = machine.graphics_devices::<MacGraphicsDevice>();

    assert_eq!(devices.len(), 1);
    check(devices[0].as_ref().unwrap());
  });
}

#[test]
fn has_a_display_for_its_configurations_display() {
  with_device("has_a_display_for_its_configurations_display", |device| {
    assert!(matches!(device.displays::<MacGraphicsDisplay>()[..], [Some(_)]));
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_device("mac_graphics_device_a_clone_is_the_same_object", |device| {
    assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_device("mac_graphics_device_debugs_as_its_description", |device| {
    assert!(format!("{device:?}").starts_with("<VZMacGraphicsDevice: 0x"));
  });
}
