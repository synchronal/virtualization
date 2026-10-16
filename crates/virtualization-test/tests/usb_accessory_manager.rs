//! The suites aren't signed with `com.apple.developer.accessory-access.usb`,
//! nor an app with a UI, which `AAUSBAccessoryManager` needs: it refuses
//! them.

use std::sync::Arc;
use std::sync::mpsc;

use virtualization::accessory_access::{
  Error, ErrorCode, USBAccessoryListener, USBAccessoryManager, USBAccessoryMatchingCriteria,
  USBAccessoryMatchingCriteriaDeviceMatchingCriteria,
};

struct Listener;

impl USBAccessoryListener for Listener {}

fn listener() -> Arc<dyn USBAccessoryListener> {
  Arc::new(Listener)
}

#[test]
fn is_shared() {
  let manager = USBAccessoryManager::shared().unwrap();

  assert!(format!("{manager:?}").starts_with("<AAUSBAccessoryManager: 0x"));
  assert_eq!(
    format!("{:?}", USBAccessoryManager::shared().unwrap()),
    format!("{manager:?}")
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let manager = USBAccessoryManager::shared().unwrap();

  assert_eq!(format!("{:?}", manager.clone()), format!("{manager:?}"));
}

#[test]
fn fails_to_register_a_listener_without_the_entitlement() {
  let manager = USBAccessoryManager::shared().unwrap();
  let criteria =
    USBAccessoryMatchingCriteria::with_device_matching_criteria(&USBAccessoryMatchingCriteriaDeviceMatchingCriteria {
      vendor_id: Some(0x05ac),
      ..Default::default()
    })
    .unwrap();
  let (sender, receiver) = mpsc::channel();

  manager.register_listener(&listener(), &[&criteria], move |result| {
    sender.send(result).unwrap();
  });

  let error = match receiver.recv().unwrap() {
    Err(Error::NSError(error)) => error,
    other => panic!("expected AccessoryAccess's error, got {other:?}"),
  };

  assert_eq!(error.domain, "AAErrorDomain");
  assert!(matches!(ErrorCode::new(error.code), Ok(ErrorCode::InternalError)));
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("Unable to communicate with service.")
  );
}

#[test]
fn unregisters_a_listener_it_never_registered() {
  let manager = USBAccessoryManager::shared().unwrap();
  let (sender, receiver) = mpsc::channel();

  manager.unregister_listener(&listener(), move || sender.send(()).unwrap());

  receiver.recv().unwrap();
}
