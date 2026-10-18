use virtualization::{LinuxBootLoader, MacOSBootLoader, VirtualMachineConfiguration};

#[test]
fn is_the_boot_loader_of_the_configuration_it_was_given_to() {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  configuration.set_boot_loader(Some(&MacOSBootLoader::new().unwrap()));

  assert!(configuration.boot_loader::<MacOSBootLoader>().is_some());
  assert!(configuration.boot_loader::<LinuxBootLoader>().is_none());
}

#[test]
fn a_clone_is_the_same_object() {
  let boot_loader = MacOSBootLoader::new().unwrap();

  assert_eq!(format!("{:?}", boot_loader.clone()), format!("{boot_loader:?}"));
}

#[test]
fn debugs_as_its_description() {
  let boot_loader = MacOSBootLoader::new().unwrap();

  assert!(format!("{boot_loader:?}").starts_with("<VZMacOSBootLoader: 0x"));
}
