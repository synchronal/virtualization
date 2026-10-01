use virtualization::LinuxBootLoader;
use virtualization_test::file_url;

#[test]
fn has_the_kernel_it_was_made_with() {
  let loader = LinuxBootLoader::new(&file_url("/nowhere/vmlinuz")).unwrap();

  assert_eq!(loader.kernel_url().unwrap(), file_url("/nowhere/vmlinuz"));
  assert_eq!(loader.command_line(), "");
  assert_eq!(loader.initial_ramdisk_url().unwrap(), None);
}

#[test]
fn sets_each_property() {
  let loader = LinuxBootLoader::new(&file_url("/nowhere/vmlinuz")).unwrap();

  loader
    .set_kernel_url(&file_url("/elsewhere/vmlinuz"))
    .unwrap();
  loader.set_command_line("console=hvc0");
  loader
    .set_initial_ramdisk_url(Some(&file_url("/nowhere/initrd")))
    .unwrap();

  assert_eq!(loader.kernel_url().unwrap(), file_url("/elsewhere/vmlinuz"));
  assert_eq!(loader.command_line(), "console=hvc0");
  assert_eq!(loader.initial_ramdisk_url().unwrap(), Some(file_url("/nowhere/initrd")));

  loader.set_initial_ramdisk_url(None).unwrap();

  assert_eq!(loader.initial_ramdisk_url().unwrap(), None);
}

#[test]
fn keeps_a_url_foundation_must_escape() {
  let loader = LinuxBootLoader::new(&file_url("/no where/vmlinüz")).unwrap();

  assert_eq!(loader.kernel_url().unwrap(), file_url("/no where/vmlinüz"));
}

#[test]
fn a_clone_is_the_same_object() {
  let loader = LinuxBootLoader::new(&file_url("/nowhere/vmlinuz")).unwrap();
  let clone = loader.clone();

  clone.set_command_line("console=hvc0");
  drop(clone);

  assert_eq!(loader.command_line(), "console=hvc0");
}

#[test]
fn debugs_as_its_description() {
  let loader = LinuxBootLoader::new(&file_url("/nowhere/vmlinuz")).unwrap();

  assert!(format!("{loader:?}").starts_with("<VZLinuxBootLoader: 0x"));
}
