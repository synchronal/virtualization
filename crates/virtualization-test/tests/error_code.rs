use virtualization::{ErrorCode, VirtualMachineConfiguration, error_domain};
use virtualization_test::ns_error;

#[test]
fn has_virtualizations_domain() {
  assert_eq!(error_domain().unwrap(), "VZErrorDomain");
}

#[test]
fn is_each_case_by_its_raw_value() {
  for (raw_value, code) in [
    (1, ErrorCode::InternalError),
    (2, ErrorCode::InvalidVirtualMachineConfiguration),
    (3, ErrorCode::InvalidVirtualMachineState),
    (4, ErrorCode::InvalidVirtualMachineStateTransition),
    (5, ErrorCode::InvalidDiskImage),
    (6, ErrorCode::VirtualMachineLimitExceeded),
    (7, ErrorCode::NetworkError),
    (8, ErrorCode::OutOfDiskSpace),
    (9, ErrorCode::OperationCancelled),
    (10, ErrorCode::NotSupported),
    (11, ErrorCode::Save),
    (12, ErrorCode::Restore),
    (10001, ErrorCode::RestoreImageCatalogLoadFailed),
    (10002, ErrorCode::InvalidRestoreImageCatalog),
    (10003, ErrorCode::NoSupportedRestoreImagesInCatalog),
    (10004, ErrorCode::RestoreImageLoadFailed),
    (10005, ErrorCode::InvalidRestoreImage),
    (10006, ErrorCode::InstallationRequiresUpdate),
    (10007, ErrorCode::InstallationFailed),
    (20001, ErrorCode::NetworkBlockDeviceNegotiationFailed),
    (20002, ErrorCode::NetworkBlockDeviceDisconnected),
    (30001, ErrorCode::USBControllerNotFound),
    (30002, ErrorCode::DeviceAlreadyAttached),
    (30003, ErrorCode::DeviceInitializationFailure),
    (30004, ErrorCode::DeviceNotFound),
    (40001, ErrorCode::GuestProvisioningInvalidFullName),
    (40002, ErrorCode::GuestProvisioningInvalidUsername),
    (40003, ErrorCode::GuestProvisioningInvalidPassword),
    (50001, ErrorCode::EFISecureBootEnrollmentFailed),
    (50002, ErrorCode::EFIVariableInaccessible),
  ] {
    assert_eq!(ErrorCode::new(raw_value).unwrap(), code, "{raw_value}");
    assert_eq!(code.raw_value().unwrap(), raw_value, "{code:?}");
  }
}

#[test]
fn is_unknown_for_a_raw_value_with_no_case() {
  assert_eq!(ErrorCode::new(0).unwrap(), ErrorCode::Unknown(0));
  assert_eq!(ErrorCode::new(13).unwrap(), ErrorCode::Unknown(13));
}

#[test]
fn has_an_unknown_codes_raw_value() {
  assert_eq!(ErrorCode::Unknown(42).raw_value().unwrap(), 42);
}

/// A configuration with no boot loader is invalid.
#[test]
fn is_the_code_of_what_virtualization_throws() {
  let error = ns_error(VirtualMachineConfiguration::new().unwrap().validate());

  assert_eq!(error.domain, error_domain().unwrap());
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidVirtualMachineConfiguration
  );
}
