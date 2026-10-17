use virtualization::{ErrorCode, VirtioSocketDeviceConfiguration, VirtualMachineConfiguration};
use virtualization_test::{add_socket_device, booting_machine, initial_ramdisk, linux_configuration, ns_error, start};

#[test]
fn is_a_socket_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_socket_device_of_the_configuration_it_was_given_to");

  configuration.set_socket_devices(&[&VirtioSocketDeviceConfiguration::new().unwrap()]);

  let given = configuration.socket_devices::<VirtioSocketDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert!(given[0].is_some());
  configuration.validate().unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_with_two() {
  let configuration = linux_configuration("socket_device_fails_validation_with_two");

  configuration.set_socket_devices(&[
    &VirtioSocketDeviceConfiguration::new().unwrap(),
    &VirtioSocketDeviceConfiguration::new().unwrap(),
  ]);

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
  let device = VirtioSocketDeviceConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", device.clone()), format!("{device:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioSocketDeviceConfiguration::new().unwrap())
      .starts_with("<VZVirtioSocketDeviceConfiguration: 0x")
  );
}

/// Reports how many devices the guest's Virtio socket driver has.
const DEVICES: &str = "echo \"devices:$(ls /sys/bus/virtio/drivers/vmw_vsock_virtio_transport | grep -c virtio)\"";

/// What a guest reports with `DEVICES`, after `configure`.
fn devices(name: &str, configure: impl FnOnce(&VirtualMachineConfiguration) + Send) -> String {
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, DEVICES)), configure);

  start(&queue, &machine);

  console.read_until("devices:").trim_end().to_string()
}

#[test]
fn a_guest_has_a_socket_device() {
  assert_eq!(devices("a_guest_has_a_socket_device", add_socket_device), "devices:1");
}

#[test]
fn a_guest_without_one_has_no_socket_device() {
  assert_eq!(devices("a_guest_without_one_has_no_socket_device", |_| {}), "devices:0");
}
