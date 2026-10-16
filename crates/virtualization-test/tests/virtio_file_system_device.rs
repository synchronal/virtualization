use virtualization::url::Url;
use virtualization::{
  MultipleDirectoryShare, SharedDirectory, SingleDirectoryShare, VirtioFileSystemDevice,
  VirtioFileSystemDeviceConfiguration, VirtualMachine,
};
use virtualization_test::{
  booting_machine, directory, initial_ramdisk, linux_configuration, on_queue, serial_queue, start,
};

#[test]
fn is_one_per_virtio_file_system_device_configuration_with_its_tag_and_share() {
  let name = "is_one_per_virtio_file_system_device_configuration_with_its_tag_and_share";
  let url = Url::from_directory_path(directory(name, &[])).unwrap();
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);
    let shared = VirtioFileSystemDeviceConfiguration::new("shared").unwrap();

    shared.set_share(Some(
      &SingleDirectoryShare::new(&SharedDirectory::new(&url, true).unwrap()).unwrap(),
    ));
    configuration
      .set_directory_sharing_devices(&[&shared, &VirtioFileSystemDeviceConfiguration::new("unshared").unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.directory_sharing_devices::<VirtioFileSystemDevice>();

    assert_eq!(devices.len(), 2);
    let shared = devices[0].as_ref().unwrap();
    let unshared = devices[1].as_ref().unwrap();
    assert_eq!(shared.tag(), "shared");
    assert_eq!(
      shared
        .share::<SingleDirectoryShare>()
        .unwrap()
        .directory()
        .url()
        .unwrap(),
      url
    );
    assert_eq!(unshared.tag(), "unshared");
    assert!(unshared.share::<SingleDirectoryShare>().is_none());
  });
}

/// A guest that mounts the device tagged `share` at `/mnt` and reports what
/// `/mnt/which` holds for each line it reads, once it says it's ready.
const WHICH: &str = "mkdir /mnt\nmount -t virtiofs share /mnt || echo mount:failed\necho ready\nwhile read line; do \
                     echo \"which:$(cat /mnt/which 2>&1)\"; done";

#[test]
fn changes_its_share_while_running() {
  let name = "changes_its_share_while_running";
  let first = Url::from_directory_path(directory(&format!("{name}_first"), &[("which", "first")])).unwrap();
  let second = Url::from_directory_path(directory(&format!("{name}_second"), &[("which", "second")])).unwrap();
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, WHICH)), |configuration| {
    let device = VirtioFileSystemDeviceConfiguration::new("share").unwrap();

    device.set_share(Some(
      &SingleDirectoryShare::new(&SharedDirectory::new(&first, true).unwrap()).unwrap(),
    ));
    configuration.set_directory_sharing_devices(&[&device]);
  });

  start(&queue, &machine);
  console.read_until("ready");
  console.ask_until("which:", |which| which == "first");

  on_queue(&queue, || {
    let devices = machine.directory_sharing_devices::<VirtioFileSystemDevice>();
    let device = devices[0].as_ref().unwrap();

    device.set_share(Some(
      &SingleDirectoryShare::new(&SharedDirectory::new(&second, true).unwrap()).unwrap(),
    ));

    assert_eq!(
      device
        .share::<SingleDirectoryShare>()
        .unwrap()
        .directory()
        .url()
        .unwrap(),
      second
    );
  });
  // The guest's mount sees the new share only once its cache of the old one
  // expires.
  console.ask_until("which:", |which| which == "second");
}

#[test]
fn a_clone_is_the_same_object() {
  let name = "virtio_file_system_device_a_clone_is_the_same_object";
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    configuration.set_directory_sharing_devices(&[&VirtioFileSystemDeviceConfiguration::new("share").unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.directory_sharing_devices::<VirtioFileSystemDevice>();
    let device = devices[0].as_ref().unwrap();

    device
      .clone()
      .set_share(Some(&MultipleDirectoryShare::new().unwrap()));

    assert!(device.share::<MultipleDirectoryShare>().is_some());
  });
}

#[test]
fn debugs_as_its_description() {
  let name = "virtio_file_system_device_debugs_as_its_description";
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    configuration.set_directory_sharing_devices(&[&VirtioFileSystemDeviceConfiguration::new("share").unwrap()]);
    let machine = VirtualMachine::with_queue(&configuration, &queue).unwrap();
    let devices = machine.directory_sharing_devices::<VirtioFileSystemDevice>();

    assert!(format!("{:?}", devices[0].as_ref().unwrap()).starts_with("<VZVirtioFileSystemDevice: 0x"));
  });
}
