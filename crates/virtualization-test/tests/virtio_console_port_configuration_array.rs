use virtualization::{VirtioConsoleDeviceConfiguration, VirtioConsolePortConfiguration};

#[test]
fn allocates_one_port_but_has_none_at_first() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();

  assert_eq!(ports.maximum_port_count(), 1);
  assert!(ports.get(0).is_none());
}

#[test]
fn keeps_a_port_at_its_index() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();
  let port = VirtioConsolePortConfiguration::new().unwrap();

  ports.set(2, Some(&port));

  assert_eq!(format!("{:?}", ports.get(2).unwrap()), format!("{port:?}"));
  assert!(ports.get(0).is_none());
}

#[test]
fn counts_ports_up_to_the_highest_index() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();

  ports.set(2, Some(&VirtioConsolePortConfiguration::new().unwrap()));

  assert_eq!(ports.maximum_port_count(), 3);
}

#[test]
fn loses_a_port() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();

  ports.set(0, Some(&VirtioConsolePortConfiguration::new().unwrap()));
  ports.set(0, None);

  assert!(ports.get(0).is_none());
}

#[test]
fn keeps_its_maximum_port_count() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();

  ports.set_maximum_port_count(4);

  assert_eq!(ports.maximum_port_count(), 4);
}

#[test]
fn is_the_same_array_each_time() {
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();

  device
    .ports()
    .set(0, Some(&VirtioConsolePortConfiguration::new().unwrap()));

  assert!(device.ports().get(0).is_some());
}

#[test]
fn a_clone_is_the_same_object() {
  let ports = VirtioConsoleDeviceConfiguration::new().unwrap().ports();

  ports.clone().set_maximum_port_count(4);

  assert_eq!(ports.maximum_port_count(), 4);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioConsoleDeviceConfiguration::new().unwrap().ports())
      .starts_with("<VZVirtioConsolePortConfigurationArray: 0x")
  );
}
