use std::sync::Arc;

use virtualization::{
  CustomVirtioDeviceConfiguration, CustomVirtioDeviceConfigurationDelegate, CustomVirtioDeviceDelegateProvider,
};
use virtualization_test::serial_queue;

struct Nothing;

impl CustomVirtioDeviceConfigurationDelegate for Nothing {}

#[test]
fn has_the_queue_it_was_made_with() {
  let queue = serial_queue("has_the_queue_it_was_made_with");
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider = CustomVirtioDeviceDelegateProvider::new(&queue, &delegate).unwrap();

  assert!(std::ptr::eq(&*provider.device_queue(), &*queue));
}

#[test]
fn has_the_delegate_it_was_made_with() {
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider =
    CustomVirtioDeviceDelegateProvider::new(&serial_queue("has_the_delegate_it_was_made_with"), &delegate).unwrap();

  assert!(Arc::ptr_eq(&provider.delegate().unwrap(), &delegate));
}

#[test]
fn holds_its_delegate_weakly() {
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider =
    CustomVirtioDeviceDelegateProvider::new(&serial_queue("holds_its_delegate_weakly"), &delegate).unwrap();

  drop(delegate);

  assert!(provider.delegate().is_none());
}

#[test]
fn is_the_provider_of_the_configuration_it_was_given_to() {
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider = CustomVirtioDeviceDelegateProvider::new(
    &serial_queue("is_the_provider_of_the_configuration_it_was_given_to"),
    &delegate,
  )
  .unwrap();
  let configuration = CustomVirtioDeviceConfiguration::new().unwrap();

  configuration.set_provider(Some(&provider));

  assert!(Arc::ptr_eq(
    &configuration
      .provider::<CustomVirtioDeviceDelegateProvider>()
      .unwrap()
      .delegate()
      .unwrap(),
    &delegate
  ));
}

#[test]
fn a_clone_is_the_same_object() {
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider =
    CustomVirtioDeviceDelegateProvider::new(&serial_queue("provider_a_clone_is_the_same_object"), &delegate).unwrap();

  assert_eq!(format!("{:?}", provider.clone()), format!("{provider:?}"));
}

#[test]
fn debugs_as_its_description() {
  let delegate: Arc<dyn CustomVirtioDeviceConfigurationDelegate> = Arc::new(Nothing);
  let provider =
    CustomVirtioDeviceDelegateProvider::new(&serial_queue("provider_debugs_as_its_description"), &delegate).unwrap();

  assert!(format!("{provider:?}").starts_with("<VZCustomVirtioDeviceDelegateProvider: 0x"));
}
