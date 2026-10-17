use super::GUARDED;

pub(crate) enum VZErrorCode {
  InternalError,
  InvalidVirtualMachineConfiguration,
  InvalidVirtualMachineState,
  InvalidVirtualMachineStateTransition,
  InvalidDiskImage,
  VirtualMachineLimitExceeded,
  NetworkError,
  OutOfDiskSpace,
  OperationCancelled,
  NotSupported,
  Save,
  Restore,
  RestoreImageCatalogLoadFailed,
  InvalidRestoreImageCatalog,
  NoSupportedRestoreImagesInCatalog,
  RestoreImageLoadFailed,
  InvalidRestoreImage,
  InstallationRequiresUpdate,
  InstallationFailed,
  NetworkBlockDeviceNegotiationFailed,
  NetworkBlockDeviceDisconnected,
  USBControllerNotFound,
  DeviceAlreadyAttached,
  DeviceInitializationFailure,
  DeviceNotFound,
  GuestProvisioningInvalidFullName,
  GuestProvisioningInvalidUsername,
  GuestProvisioningInvalidPassword,
  EFISecureBootEnrollmentFailed,
  EFIVariableInaccessible,
  Unknown(isize),
}

pub(crate) fn vz_error_domain() -> String {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_error_code_init_raw_value(_raw_value: isize) -> VZErrorCode {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_error_code_raw_value(_code: VZErrorCode) -> isize {
  unreachable!("{GUARDED}")
}
