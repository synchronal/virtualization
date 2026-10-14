use virtualization::{
  DiskImageStorageDeviceAttachment, NVMExpressControllerDeviceConfiguration, StorageDeviceConfiguration,
  VirtioBlockDeviceConfiguration,
};
use virtualization_test::{booting_machine, disk_image, initial_ramdisk, linux_configuration, start};

fn configuration(name: &str) -> NVMExpressControllerDeviceConfiguration {
  NVMExpressControllerDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&disk_image(name), true).unwrap())
    .unwrap()
}

#[test]
fn has_the_attachment_it_was_made_with() {
  let url = disk_image("nvme_has_the_attachment_it_was_made_with");
  let configuration =
    NVMExpressControllerDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&url, true).unwrap()).unwrap();

  let attachment = configuration
    .attachment::<DiskImageStorageDeviceAttachment>()
    .unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.is_read_only());
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = configuration("nvme_a_clone_is_the_same_object");
  let clone = configuration.clone();

  assert_eq!(format!("{clone:?}"), format!("{configuration:?}"));
}

#[test]
fn is_a_storage_device_of_the_configuration_it_was_given_to() {
  let name = "nvme_is_a_storage_device_of_the_configuration_it_was_given_to";
  let machine = linux_configuration(name);

  machine.set_storage_devices(&[&configuration(name)]);

  assert!(matches!(
    machine.storage_devices::<NVMExpressControllerDeviceConfiguration>()[..],
    [Some(_)]
  ));
  assert!(matches!(
    machine.storage_devices::<VirtioBlockDeviceConfiguration>()[..],
    [None]
  ));
  machine.validate().unwrap();
}

/// The suites' kernel has no NVMe driver, so the guest has the controller, by
/// its PCI class (mass storage, non-volatile memory, NVM Express), and no
/// disk.
#[test]
fn a_guest_has_an_nvm_express_controller() {
  let name = "a_guest_has_an_nvm_express_controller";
  let script = "echo \"classes:$(cat /sys/bus/pci/devices/*/class | tr '\\n' ' ')\"";

  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |machine| {
    machine.set_storage_devices(&[&configuration(name)]);
  });
  start(&queue, &machine);

  assert!(console.read_until("classes:").contains("0x010802"));
}

#[test]
fn debugs_as_its_description() {
  let configuration = configuration("nvme_debugs_as_its_description");

  assert!(format!("{configuration:?}").starts_with("<VZNVMExpressControllerDeviceConfiguration: 0x"));
}
