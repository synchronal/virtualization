use virtualization::{
  EFIBootLoader, ErrorCode, LinuxBootLoader, VirtualMachine, VirtualMachineConfiguration, VirtualMachineState,
};
use virtualization_test::{ns_error, on_queue, serial_queue, start, stop, variable_store};

#[test]
fn has_no_variable_store_when_new() {
  assert!(EFIBootLoader::new().unwrap().variable_store().is_none());
}

#[test]
fn has_the_variable_store_it_was_given() {
  let loader = EFIBootLoader::new().unwrap();
  let store = variable_store("has_the_variable_store_it_was_given");

  loader.set_variable_store(Some(&store));

  assert_eq!(loader.variable_store().unwrap().url().unwrap(), store.url().unwrap());

  loader.set_variable_store(None);

  assert!(loader.variable_store().is_none());
}

/// A configuration that boots from EFI: 1 CPU and 512 MiB.
fn efi_configuration(loader: &EFIBootLoader) -> VirtualMachineConfiguration {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  configuration.set_boot_loader(Some(loader));
  configuration.set_cpu_count(1);
  configuration.set_memory_size(512 * 1024 * 1024);

  configuration
}

#[test]
fn is_the_boot_loader_a_configuration_was_given() {
  let loader = EFIBootLoader::new().unwrap();
  let configuration = efi_configuration(&loader);

  assert!(configuration.boot_loader::<EFIBootLoader>().is_some());
  assert!(configuration.boot_loader::<LinuxBootLoader>().is_none());
}

#[test]
fn validates_with_a_variable_store() {
  let loader = EFIBootLoader::new().unwrap();

  loader.set_variable_store(Some(&variable_store("validates_with_a_variable_store")));

  efi_configuration(&loader).validate().unwrap();
}

#[test]
fn fails_validation_as_virtualization_does_without_a_variable_store() {
  let error = ns_error(efi_configuration(&EFIBootLoader::new().unwrap()).validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
}

#[test]
fn starts_a_machine_with_nothing_to_boot() {
  let name = "efi_starts_a_machine_with_nothing_to_boot";
  let queue = serial_queue(name);
  let machine = on_queue(&queue, || {
    let loader = EFIBootLoader::new().unwrap();

    loader.set_variable_store(Some(&variable_store(name)));

    VirtualMachine::with_queue(&efi_configuration(&loader), &queue).unwrap()
  });

  start(&queue, &machine);

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);

  stop(&queue, &machine);
}

#[test]
fn a_clone_is_the_same_object() {
  let loader = EFIBootLoader::new().unwrap();
  let clone = loader.clone();

  clone.set_variable_store(Some(&variable_store("efi_boot_loader_a_clone_is_the_same_object")));
  drop(clone);

  assert!(loader.variable_store().is_some());
}

#[test]
fn debugs_as_its_description() {
  let loader = EFIBootLoader::new().unwrap();

  assert!(format!("{loader:?}").starts_with("<VZEFIBootLoader: 0x"));
}
