use virtualization::{
  CGSize, GraphicsDevice, GraphicsDisplay, MacGraphicsDevice, MacGraphicsDeviceConfiguration, MacGraphicsDisplay,
  MacGraphicsDisplayConfiguration,
};
use virtualization_test::{macos_machine, on_queue};

/// Runs `check` on the display of a macOS machine with one, 1920 by 1080 at
/// 80 pixels per inch, on its queue and not started.
fn with_display(name: &str, check: impl FnOnce(&MacGraphicsDisplay) + Send) {
  let (queue, machine) = macos_machine(name, |configuration| {
    let device = MacGraphicsDeviceConfiguration::new().unwrap();

    device.set_displays(&[&MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap()]);
    configuration.set_graphics_devices(&[&device]);
  });

  on_queue(&queue, || {
    let devices = machine.graphics_devices::<MacGraphicsDevice>();
    let displays = devices[0]
      .as_ref()
      .unwrap()
      .displays::<MacGraphicsDisplay>();

    check(displays[0].as_ref().unwrap());
  });
}

#[test]
fn has_the_pixels_per_inch_of_its_configuration() {
  with_display("has_the_pixels_per_inch_of_its_configuration", |display| {
    assert_eq!(display.pixels_per_inch(), 80);
  });
}

#[test]
fn has_the_size_of_its_configuration() {
  with_display("mac_graphics_display_has_the_size_of_its_configuration", |display| {
    assert_eq!(
      display.size_in_pixels(),
      CGSize {
        width: 1920.0,
        height: 1080.0
      }
    );
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_display("mac_graphics_display_a_clone_is_the_same_object", |display| {
    assert_eq!(format!("{:?}", display.clone()), format!("{display:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_display("mac_graphics_display_debugs_as_its_description", |display| {
    assert!(format!("{display:?}").starts_with("<VZMacGraphicsDisplay: 0x"));
  });
}
