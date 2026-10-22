use virtualization::{CustomVirtioDeviceConfiguration, ErrorCode, VirtioSharedMemoryRegionConfiguration};
use virtualization_test::{custom_virtio_device_provider, linux_configuration, ns_error};

#[test]
fn has_the_id_and_size_it_was_made_with() {
  let region = VirtioSharedMemoryRegionConfiguration::new(7, 0x20_0000).unwrap();

  assert_eq!((region.region_id(), region.size()), (7, 0x20_0000));
}

/// A device with `regions`, in a Linux machine's configuration, validated.
fn validated(name: &str, regions: &[&VirtioSharedMemoryRegionConfiguration]) -> Result<(), virtualization::Error> {
  let configuration = linux_configuration(name);
  let device = CustomVirtioDeviceConfiguration::new().unwrap();

  device.set_device_id(4);
  device.set_pci_class_id(0xff);
  device.set_virtio_queue_count(1);
  device.set_provider(Some(&custom_virtio_device_provider(name)));
  device.set_shared_memory_regions(regions);
  configuration.set_custom_virtio_devices(&[&device]);

  configuration.validate()
}

#[test]
fn validates_in_a_device() {
  validated(
    "virtio_shared_memory_region_configuration_validates_in_a_device",
    &[&VirtioSharedMemoryRegionConfiguration::new(0, 0x40_0000).unwrap()],
  )
  .unwrap();
}

#[test]
fn fails_validation_beyond_the_most_a_device_allows() {
  let error = ns_error(validated(
    "fails_validation_beyond_the_most_a_device_allows",
    &[
      &VirtioSharedMemoryRegionConfiguration::new(0, 0x40_0000).unwrap(),
      &VirtioSharedMemoryRegionConfiguration::new(1, 0x40_0000).unwrap(),
    ],
  ));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
  assert_eq!(
    error.localized_description,
    "Invalid virtual machine configuration. The number of Virtio shared memory regions is greater than the maximum \
     number supported."
  );
}

#[test]
fn validates_with_no_size() {
  validated(
    "virtio_shared_memory_region_configuration_validates_with_no_size",
    &[&VirtioSharedMemoryRegionConfiguration::new(0, 0).unwrap()],
  )
  .unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let region = VirtioSharedMemoryRegionConfiguration::new(1, 0x1000).unwrap();

  assert_eq!(format!("{:?}", region.clone()), format!("{region:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioSharedMemoryRegionConfiguration::new(1, 0x1000).unwrap())
      .starts_with("<VZVirtioSharedMemoryRegionConfiguration: 0x")
  );
}
