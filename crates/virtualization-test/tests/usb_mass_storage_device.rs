use virtualization::uuid::Uuid;
use virtualization::{
  USBController, USBControllerConfiguration, USBDevice, USBDeviceConfiguration, USBMassStorageDevice, XHCIController,
  XHCIControllerConfiguration,
};
use virtualization_test::{
  booting_machine, completed, initial_ramdisk, on_queue, start, usb_mass_storage_device_configuration,
  with_usb_mass_storage_device, xhci_controller,
};

#[test]
fn has_the_uuid_of_its_configuration_and_no_controller_when_new() {
  let configuration = usb_mass_storage_device_configuration("has_the_uuid_of_its_configuration");

  let device = USBMassStorageDevice::new(&configuration).unwrap();

  assert_eq!(device.uuid(), configuration.uuid());
  assert!(device.usb_controller::<XHCIController>().is_none());
}

#[test]
fn is_the_device_of_a_controller_whose_configuration_has_its_own() {
  let name = "is_the_device_of_a_controller_whose_configuration_has_its_own";
  let uuid: Uuid = "01234567-89ab-cdef-0123-456789abcdef".parse().unwrap();

  let (queue, machine, _console) = booting_machine(name, None, |machine| {
    let controller = XHCIControllerConfiguration::new().unwrap();
    let device = usb_mass_storage_device_configuration(name);

    device.set_uuid(&uuid);
    controller.set_usb_devices(&[&device]);
    machine.set_usb_controllers(&[&controller]);
  });

  on_queue(&queue, || {
    let controller = xhci_controller(&machine);
    let device = controller
      .usb_devices::<USBMassStorageDevice>()
      .remove(0)
      .unwrap();

    assert_eq!(device.uuid(), uuid);
    assert_eq!(
      format!("{:?}", device.usb_controller::<XHCIController>().unwrap()),
      format!("{controller:?}")
    );
  });
}

#[test]
fn has_the_controller_it_is_attached_to_until_detached() {
  let name = "has_the_controller_it_is_attached_to_until_detached";

  let (queue, machine, _console) = booting_machine(name, Some(&initial_ramdisk(name, "read line")), |machine| {
    machine.set_usb_controllers(&[&XHCIControllerConfiguration::new().unwrap()]);
  });
  let controller = || with_usb_mass_storage_device(|device| device.usb_controller::<XHCIController>().is_some());

  start(&queue, &machine);
  completed(&queue, |completion_handler| {
    with_usb_mass_storage_device(|device| xhci_controller(&machine).attach(device, completion_handler));
  })
  .unwrap();

  assert!(on_queue(&queue, controller));

  completed(&queue, |completion_handler| {
    with_usb_mass_storage_device(|device| xhci_controller(&machine).detach(device, completion_handler));
  })
  .unwrap();

  assert!(!on_queue(&queue, controller));
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = usb_mass_storage_device_configuration("usb_device_a_clone_is_the_same_object");
  let device = USBMassStorageDevice::new(&configuration).unwrap();
  let clone = device.clone();

  drop(device);

  assert_eq!(clone.uuid(), configuration.uuid());
}

#[test]
fn debugs_as_its_description() {
  let configuration = usb_mass_storage_device_configuration("usb_device_debugs_as_its_description");
  let device = USBMassStorageDevice::new(&configuration).unwrap();

  assert!(format!("{device:?}").starts_with("<VZUSBMassStorageDevice: 0x"));
}
