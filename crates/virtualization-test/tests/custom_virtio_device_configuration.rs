use std::sync::Arc;

use virtualization::{
  CustomVirtioDeviceConfiguration, CustomVirtioDeviceConfigurationDelegate, CustomVirtioDeviceDelegateProvider,
  ErrorCode, VirtioDeviceSpecificConfiguration, VirtioSharedMemoryRegionConfiguration,
};
use virtualization_test::{custom_virtio_device_provider, linux_configuration, ns_error, serial_queue};

#[test]
fn starts_empty() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  assert_eq!(configuration.device_id(), 0);
  assert_eq!(configuration.pci_class_id(), 0);
  assert_eq!(configuration.pci_subclass_id(), 0);
  assert_eq!(configuration.virtio_queue_count(), 0);
  assert!(configuration.device_specific_configuration().is_none());
  assert!(
    configuration
      .provider::<CustomVirtioDeviceDelegateProvider>()
      .is_none()
  );
  assert!(configuration.shared_memory_regions().is_empty());
  assert!(!configuration.supports_save_restore());
}

/// `VIRTIO_F_VERSION_1` (bit 32) is mandatory.
#[test]
fn offers_virtio_version_1_as_mandatory() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .mandatory_features();

  assert_eq!((features.subset0(), features.subset1()), (0, 0x1));
}

/// Indirect descriptors and event indexes (bits 28 and 29), and bit 34, for
/// performance.
#[test]
fn offers_what_performs_better_as_optional() {
  let features = CustomVirtioDeviceConfiguration::new()
    .unwrap()
    .optional_features();

  assert_eq!((features.subset0(), features.subset1()), (0x3000_0000, 0x4));
}

#[test]
fn allows_one_shared_memory_region() {
  assert_eq!(
    CustomVirtioDeviceConfiguration::maximum_allowed_shared_memory_region_count().unwrap(),
    1
  );
}

#[test]
fn keeps_what_it_is_given() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration.set_device_id(4);
  configuration.set_pci_class_id(0xff);
  configuration.set_pci_subclass_id(0x80);
  configuration.set_virtio_queue_count(2);
  configuration.set_supports_save_restore(true);

  assert_eq!(configuration.device_id(), 4);
  assert_eq!(configuration.pci_class_id(), 0xff);
  assert_eq!(configuration.pci_subclass_id(), 0x80);
  assert_eq!(configuration.virtio_queue_count(), 2);
  assert!(configuration.supports_save_restore());
}

#[test]
fn keeps_the_device_specific_configuration_it_is_given() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration
    .set_device_specific_configuration(Some(&VirtioDeviceSpecificConfiguration::new(&[1, 2, 3, 4]).unwrap()));

  assert_eq!(
    configuration
      .device_specific_configuration()
      .unwrap()
      .configuration_data(),
    [1, 2, 3, 4]
  );

  configuration.set_device_specific_configuration(None);

  assert!(configuration.device_specific_configuration().is_none());
}

#[test]
fn keeps_the_shared_memory_regions_it_is_given() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration.set_shared_memory_regions(&[
    &VirtioSharedMemoryRegionConfiguration::new(1, 0x10_0000).unwrap(),
    &VirtioSharedMemoryRegionConfiguration::new(2, 0x20_0000).unwrap(),
  ]);

  let regions: Vec<_> = configuration
    .shared_memory_regions()
    .iter()
    .map(|region| (region.region_id(), region.size()))
    .collect();

  assert_eq!(regions, [(1, 0x10_0000), (2, 0x20_0000)]);
}

struct Nothing;

impl CustomVirtioDeviceConfigurationDelegate for Nothing {}

#[test]
fn keeps_the_provider_it_is_given() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider =
    CustomVirtioDeviceDelegateProvider::new(&serial_queue("keeps_the_provider_it_is_given"), &delegate).unwrap();

  configuration.set_provider(Some(&provider));

  assert_eq!(
    format!(
      "{:?}",
      configuration
        .provider::<CustomVirtioDeviceDelegateProvider>()
        .unwrap()
    ),
    format!("{provider:?}")
  );

  configuration.set_provider(None);

  assert!(
    configuration
      .provider::<CustomVirtioDeviceDelegateProvider>()
      .is_none()
  );
}

#[test]
fn is_a_custom_virtio_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_custom_virtio_device_of_the_configuration_it_was_given_to");
  let device = CustomVirtioDeviceConfiguration::new().unwrap();

  device.set_device_id(4);
  device.set_pci_class_id(0xff);
  device.set_virtio_queue_count(1);
  device.set_provider(Some(&custom_virtio_device_provider(
    "is_a_custom_virtio_device_of_the_configuration_it_was_given_to",
  )));
  configuration.set_custom_virtio_devices(&[&device]);

  let given = configuration.custom_virtio_devices();

  assert_eq!(given.len(), 1);
  assert_eq!(given[0].device_id(), 4);
  configuration.validate().unwrap();
}

#[test]
fn fails_validation_without_a_provider() {
  let configuration = linux_configuration("fails_validation_without_a_provider");
  let device = CustomVirtioDeviceConfiguration::new().unwrap();

  device.set_device_id(4);
  configuration.set_custom_virtio_devices(&[&device]);
  let error = ns_error(configuration.validate());

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
  assert_eq!(
    error.localized_description,
    "Invalid virtual machine configuration. The device was missing a provider."
  );
}

#[test]
fn fails_validation_without_a_device_id() {
  let configuration = linux_configuration("fails_validation_without_a_device_id");

  configuration.set_custom_virtio_devices(&[&CustomVirtioDeviceConfiguration::new().unwrap()]);
  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration,
    "{error:#?}"
  );
  assert_eq!(
    error.localized_description,
    "Invalid virtual machine configuration. The device was missing a non-standard Virtio device ID parameter."
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration.clone().set_device_id(4);

  assert_eq!(configuration.device_id(), 4);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", CustomVirtioDeviceConfiguration::new().unwrap())
      .starts_with("<VZCustomVirtioDeviceConfiguration: 0x")
  );
}
