use virtualization::{
  VirtioConsoleDevice, VirtioConsoleDeviceConfiguration, VirtioConsolePortArray, VirtioConsolePortConfiguration,
  VirtualMachine,
};
use virtualization_test::{linux_configuration, on_queue, serial_queue};

/// Runs `check` on the ports of a machine's console device, configured with
/// `ports` at their indexes and `maximum_port_count`, on its queue and not
/// started.
fn with_ports(
  name: &str,
  ports: &[isize],
  maximum_port_count: u32,
  check: impl FnOnce(&VirtioConsolePortArray) + Send,
) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);
    let device = VirtioConsoleDeviceConfiguration::new().unwrap();

    for index in ports {
      let port = VirtioConsolePortConfiguration::new().unwrap();

      port.set_name(Some(&format!("port {index}")));
      device.ports().set(*index, Some(&port));
    }
    device.ports().set_maximum_port_count(maximum_port_count);
    configuration.set_console_devices(&[&device]);

    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();

    check(
      &machine.console_devices::<VirtioConsoleDevice>()[0]
        .as_ref()
        .unwrap()
        .ports(),
    );
  });
}

#[test]
fn has_the_maximum_port_count_of_its_configuration() {
  with_ports("has_the_maximum_port_count_of_its_configuration", &[0], 4, |ports| {
    assert_eq!(ports.maximum_port_count(), 4);
  });
}

#[test]
fn has_each_port_at_its_index() {
  with_ports("has_each_port_at_its_index", &[0, 2], 4, |ports| {
    assert_eq!(ports.get(0).unwrap().name().as_deref(), Some("port 0"));
    assert!(ports.get(1).is_none());
    assert_eq!(ports.get(2).unwrap().name().as_deref(), Some("port 2"));
  });
}

#[test]
fn a_clone_is_the_same_object() {
  with_ports("console_port_array_a_clone_is_the_same_object", &[0], 1, |ports| {
    assert_eq!(format!("{:?}", ports.clone()), format!("{ports:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_ports("console_port_array_debugs_as_its_description", &[0], 1, |ports| {
    assert!(format!("{ports:?}").starts_with("<VZVirtioConsolePortArray: 0x"));
  });
}
