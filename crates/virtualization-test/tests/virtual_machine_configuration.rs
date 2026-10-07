use virtualization::{
  DiskImageStorageDeviceAttachment, Error, LinuxBootLoader, StorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
  VirtualMachineConfiguration,
};
use virtualization_test::{disk_image, file};

fn storage_device(name: &str) -> VirtioBlockDeviceConfiguration {
  VirtioBlockDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&disk_image(name), true).unwrap()).unwrap()
}

#[test]
fn is_empty_when_new() {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  assert!(configuration.boot_loader::<LinuxBootLoader>().is_none());
  assert_eq!(configuration.memory_size(), 4 * 1024 * 1024);
  assert_eq!(configuration.cpu_count(), 1);
  assert_eq!(configuration.label(), None);
  assert!(
    configuration
      .storage_devices::<VirtioBlockDeviceConfiguration>()
      .is_empty()
  );
}

#[test]
fn has_the_allowed_sizes() {
  let minimum_memory_size = VirtualMachineConfiguration::minimum_allowed_memory_size().unwrap();
  let maximum_memory_size = VirtualMachineConfiguration::maximum_allowed_memory_size().unwrap();
  let minimum_cpu_count = VirtualMachineConfiguration::minimum_allowed_cpu_count().unwrap();
  let maximum_cpu_count = VirtualMachineConfiguration::maximum_allowed_cpu_count().unwrap();

  assert_eq!(minimum_memory_size, 4 * 1024 * 1024);
  assert!(maximum_memory_size > minimum_memory_size);
  assert_eq!(minimum_cpu_count, 1);
  assert!(maximum_cpu_count > minimum_cpu_count);
}

#[test]
fn sets_each_value() {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  configuration.set_memory_size(512 * 1024 * 1024);
  configuration.set_cpu_count(2);
  configuration.set_label(Some("test machine"));

  assert_eq!(configuration.memory_size(), 512 * 1024 * 1024);
  assert_eq!(configuration.cpu_count(), 2);
  assert_eq!(configuration.label().as_deref(), Some("test machine"));

  configuration.set_label(None);

  assert_eq!(configuration.label(), None);
}

#[test]
fn has_the_boot_loader_it_was_given() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let loader = LinuxBootLoader::new(&file("has_the_boot_loader_it_was_given", b"kernel")).unwrap();

  configuration.set_boot_loader(Some(&loader));
  loader.set_command_line("console=hvc0");

  let given = configuration.boot_loader::<LinuxBootLoader>().unwrap();

  assert_eq!(given.command_line(), "console=hvc0");

  configuration.set_boot_loader(None);

  assert!(configuration.boot_loader::<LinuxBootLoader>().is_none());
}

#[test]
fn has_the_storage_devices_it_was_given() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let first = storage_device("has_the_storage_devices_it_was_given_first");
  let second = storage_device("has_the_storage_devices_it_was_given_second");

  configuration.set_storage_devices(&[&first, &second]);
  first.set_block_device_identifier("first");
  second.set_block_device_identifier("second");

  let given = configuration.storage_devices::<VirtioBlockDeviceConfiguration>();

  assert_eq!(given.len(), 2);
  assert_eq!(given[0].as_ref().unwrap().block_device_identifier(), "first");
  assert_eq!(given[1].as_ref().unwrap().block_device_identifier(), "second");
  assert!(
    given[0]
      .as_ref()
      .unwrap()
      .attachment::<DiskImageStorageDeviceAttachment>()
      .is_some()
  );

  configuration.set_storage_devices(&[]);

  assert!(
    configuration
      .storage_devices::<VirtioBlockDeviceConfiguration>()
      .is_empty()
  );
}

#[test]
fn validates_a_linux_machine() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let loader = LinuxBootLoader::new(&file("validates_a_linux_machine", b"kernel")).unwrap();

  configuration.set_boot_loader(Some(&loader));
  configuration.set_cpu_count(1);
  configuration.set_memory_size(512 * 1024 * 1024);
  configuration.set_storage_devices(&[&storage_device("validates_a_linux_machine")]);

  configuration.validate().unwrap();
}

#[test]
fn fails_as_virtualization_does_to_validate_an_empty_label() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let loader = LinuxBootLoader::new(&file("fails_to_validate_an_empty_label", b"kernel")).unwrap();

  configuration.set_boot_loader(Some(&loader));
  configuration.set_label(Some(""));

  let Err(Error::NSError(error)) = configuration.validate() else {
    panic!("validated an empty label");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
  assert!(error.localized_description.contains("label"), "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_validate_an_empty_configuration() {
  let Err(Error::NSError(error)) = VirtualMachineConfiguration::new().unwrap().validate() else {
    panic!("validated an empty configuration");
  };

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn checks_save_restore_support() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let loader = LinuxBootLoader::new(&file("checks_save_restore_support", b"kernel")).unwrap();

  configuration.set_boot_loader(Some(&loader));
  configuration.set_cpu_count(1);
  configuration.set_memory_size(512 * 1024 * 1024);

  configuration.validate_save_restore_support().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = VirtualMachineConfiguration::new().unwrap();
  let clone = configuration.clone();

  configuration.set_cpu_count(3);
  drop(configuration);

  assert_eq!(clone.cpu_count(), 3);
}

#[test]
fn debugs_as_its_description() {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  assert!(format!("{configuration:?}").starts_with("<VZVirtualMachineConfiguration: 0x"));
}
