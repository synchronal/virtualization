use virtualization::{
  GraphicsDevice, VirtioGraphicsDevice, VirtioGraphicsDeviceConfiguration, VirtioGraphicsScanout,
  VirtioGraphicsScanoutConfiguration, VirtualMachine,
};
use virtualization_test::{linux_configuration, on_queue, serial_queue};

/// Runs `check` on the graphics device of a machine with one, of one
/// scanout, made on its queue and not started.
fn with_device(name: &str, check: impl FnOnce(&VirtioGraphicsDevice) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);
    let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

    device.set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);
    configuration.set_graphics_devices(&[&device]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.graphics_devices::<VirtioGraphicsDevice>();

    assert_eq!(devices.len(), 1);
    check(devices[0].as_ref().unwrap());
  });
}

#[test]
fn has_a_scanout_for_its_configurations_scanout() {
  with_device("has_a_scanout_for_its_configurations_scanout", |device| {
    assert!(matches!(device.displays::<VirtioGraphicsScanout>()[..], [Some(_)]));
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_device("graphics_device_a_clone_is_the_same_object", |device| {
    assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_device("graphics_device_debugs_as_its_description", |device| {
    assert!(format!("{device:?}").starts_with("<VZVirtioGraphicsDevice: 0x"));
  });
}
