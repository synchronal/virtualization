//! What a call into Virtualization failed with, and `VZError`'s codes.

use std::fmt;

pub use foundation_sys::NSError;
use url::Url;

use crate::platform::{self, ffi};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
  /// Virtualization exists only on macOS.
  Unsupported,
  /// What Virtualization threw: in [`error_domain`], with an [`ErrorCode`],
  /// or another domain's. Boxed: it is far larger than the others.
  NSError(Box<NSError>),
  /// A URL from Foundation that [`url::Url`] can't parse.
  UnparseableUrl(url::ParseError),
  /// A URL Foundation's `NSURL(string:)` rejects.
  UnrepresentableUrl(Url),
}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unsupported => write!(formatter, "Virtualization is only available on macOS"),
      Self::NSError(error) => error.fmt(formatter),
      Self::UnparseableUrl(error) => write!(formatter, "Foundation returned an unparseable URL: {error}"),
      Self::UnrepresentableUrl(url) => write!(formatter, "Foundation can't represent the URL {url}"),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::NSError(error) => error.source(),
      Self::UnparseableUrl(error) => Some(error),
      Self::Unsupported | Self::UnrepresentableUrl(_) => None,
    }
  }
}

/// [`VZErrorDomain`](https://developer.apple.com/documentation/virtualization/vzerrordomain)
pub fn error_domain() -> Result<String, Error> {
  platform::supported()?;

  Ok(ffi::vz_error_domain())
}

/// [`VZError.Code`](https://developer.apple.com/documentation/virtualization/vzerror/code):
/// an `NSError`'s `code` in [`error_domain`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
  /// [`internalError`](https://developer.apple.com/documentation/virtualization/vzerror/code/internalerror)
  InternalError,
  /// [`invalidVirtualMachineConfiguration`](https://developer.apple.com/documentation/virtualization/vzerror/code/invalidvirtualmachineconfiguration)
  InvalidVirtualMachineConfiguration,
  /// [`invalidVirtualMachineState`](https://developer.apple.com/documentation/virtualization/vzerror/code/invalidvirtualmachinestate)
  InvalidVirtualMachineState,
  /// [`invalidVirtualMachineStateTransition`](https://developer.apple.com/documentation/virtualization/vzerror/code/invalidvirtualmachinestatetransition)
  InvalidVirtualMachineStateTransition,
  /// [`invalidDiskImage`](https://developer.apple.com/documentation/virtualization/vzerror/code/invaliddiskimage)
  InvalidDiskImage,
  /// [`virtualMachineLimitExceeded`](https://developer.apple.com/documentation/virtualization/vzerror/code/virtualmachinelimitexceeded)
  VirtualMachineLimitExceeded,
  /// [`networkError`](https://developer.apple.com/documentation/virtualization/vzerror/code/networkerror)
  NetworkError,
  /// [`outOfDiskSpace`](https://developer.apple.com/documentation/virtualization/vzerror/code/outofdiskspace)
  OutOfDiskSpace,
  /// [`operationCancelled`](https://developer.apple.com/documentation/virtualization/vzerror/code/operationcancelled)
  OperationCancelled,
  /// [`notSupported`](https://developer.apple.com/documentation/virtualization/vzerror/code/notsupported)
  NotSupported,
  /// [`save`](https://developer.apple.com/documentation/virtualization/vzerror/code/save)
  Save,
  /// [`restore`](https://developer.apple.com/documentation/virtualization/vzerror/code/restore)
  Restore,
  /// [`restoreImageCatalogLoadFailed`](https://developer.apple.com/documentation/virtualization/vzerror/code/restoreimagecatalogloadfailed)
  RestoreImageCatalogLoadFailed,
  /// [`invalidRestoreImageCatalog`](https://developer.apple.com/documentation/virtualization/vzerror/code/invalidrestoreimagecatalog)
  InvalidRestoreImageCatalog,
  /// [`noSupportedRestoreImagesInCatalog`](https://developer.apple.com/documentation/virtualization/vzerror/code/nosupportedrestoreimagesincatalog)
  NoSupportedRestoreImagesInCatalog,
  /// [`restoreImageLoadFailed`](https://developer.apple.com/documentation/virtualization/vzerror/code/restoreimageloadfailed)
  RestoreImageLoadFailed,
  /// [`invalidRestoreImage`](https://developer.apple.com/documentation/virtualization/vzerror/code/invalidrestoreimage)
  InvalidRestoreImage,
  /// [`installationRequiresUpdate`](https://developer.apple.com/documentation/virtualization/vzerror/code/installationrequiresupdate)
  InstallationRequiresUpdate,
  /// [`installationFailed`](https://developer.apple.com/documentation/virtualization/vzerror/code/installationfailed)
  InstallationFailed,
  /// [`networkBlockDeviceNegotiationFailed`](https://developer.apple.com/documentation/virtualization/vzerror/code/networkblockdevicenegotiationfailed)
  NetworkBlockDeviceNegotiationFailed,
  /// [`networkBlockDeviceDisconnected`](https://developer.apple.com/documentation/virtualization/vzerror/code/networkblockdevicedisconnected)
  NetworkBlockDeviceDisconnected,
  /// [`usbControllerNotFound`](https://developer.apple.com/documentation/virtualization/vzerror/code/usbcontrollernotfound)
  USBControllerNotFound,
  /// [`deviceAlreadyAttached`](https://developer.apple.com/documentation/virtualization/vzerror/code/devicealreadyattached)
  DeviceAlreadyAttached,
  /// [`deviceInitializationFailure`](https://developer.apple.com/documentation/virtualization/vzerror/code/deviceinitializationfailure)
  DeviceInitializationFailure,
  /// [`deviceNotFound`](https://developer.apple.com/documentation/virtualization/vzerror/code/devicenotfound)
  DeviceNotFound,
  /// [`guestProvisioningInvalidFullName`](https://developer.apple.com/documentation/virtualization/vzerror/code/guestprovisioninginvalidfullname)
  GuestProvisioningInvalidFullName,
  /// [`guestProvisioningInvalidUsername`](https://developer.apple.com/documentation/virtualization/vzerror/code/guestprovisioninginvalidusername)
  GuestProvisioningInvalidUsername,
  /// [`guestProvisioningInvalidPassword`](https://developer.apple.com/documentation/virtualization/vzerror/code/guestprovisioninginvalidpassword)
  GuestProvisioningInvalidPassword,
  /// [`efiSecureBootEnrollmentFailed`](https://developer.apple.com/documentation/virtualization/vzerror/code/efisecurebootenrollmentfailed)
  EFISecureBootEnrollmentFailed,
  /// [`efiVariableInaccessible`](https://developer.apple.com/documentation/virtualization/vzerror/code/efivariableinaccessible)
  EFIVariableInaccessible,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl ErrorCode {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/virtualization/vzerror/code/init(rawvalue:)):
  /// `Unknown` for a raw value with no case.
  pub fn new(raw_value: isize) -> Result<Self, Error> {
    platform::supported()?;

    Ok(ffi::vz_error_code_init_raw_value(raw_value).into())
  }

  /// [`rawValue`](https://developer.apple.com/documentation/swift/rawrepresentable/rawvalue-swift.property)
  pub fn raw_value(self) -> Result<isize, Error> {
    platform::supported()?;

    Ok(ffi::vz_error_code_raw_value(self.into()))
  }
}

impl From<ffi::VZErrorCode> for ErrorCode {
  fn from(code: ffi::VZErrorCode) -> Self {
    match code {
      ffi::VZErrorCode::InternalError => Self::InternalError,
      ffi::VZErrorCode::InvalidVirtualMachineConfiguration => Self::InvalidVirtualMachineConfiguration,
      ffi::VZErrorCode::InvalidVirtualMachineState => Self::InvalidVirtualMachineState,
      ffi::VZErrorCode::InvalidVirtualMachineStateTransition => Self::InvalidVirtualMachineStateTransition,
      ffi::VZErrorCode::InvalidDiskImage => Self::InvalidDiskImage,
      ffi::VZErrorCode::VirtualMachineLimitExceeded => Self::VirtualMachineLimitExceeded,
      ffi::VZErrorCode::NetworkError => Self::NetworkError,
      ffi::VZErrorCode::OutOfDiskSpace => Self::OutOfDiskSpace,
      ffi::VZErrorCode::OperationCancelled => Self::OperationCancelled,
      ffi::VZErrorCode::NotSupported => Self::NotSupported,
      ffi::VZErrorCode::Save => Self::Save,
      ffi::VZErrorCode::Restore => Self::Restore,
      ffi::VZErrorCode::RestoreImageCatalogLoadFailed => Self::RestoreImageCatalogLoadFailed,
      ffi::VZErrorCode::InvalidRestoreImageCatalog => Self::InvalidRestoreImageCatalog,
      ffi::VZErrorCode::NoSupportedRestoreImagesInCatalog => Self::NoSupportedRestoreImagesInCatalog,
      ffi::VZErrorCode::RestoreImageLoadFailed => Self::RestoreImageLoadFailed,
      ffi::VZErrorCode::InvalidRestoreImage => Self::InvalidRestoreImage,
      ffi::VZErrorCode::InstallationRequiresUpdate => Self::InstallationRequiresUpdate,
      ffi::VZErrorCode::InstallationFailed => Self::InstallationFailed,
      ffi::VZErrorCode::NetworkBlockDeviceNegotiationFailed => Self::NetworkBlockDeviceNegotiationFailed,
      ffi::VZErrorCode::NetworkBlockDeviceDisconnected => Self::NetworkBlockDeviceDisconnected,
      ffi::VZErrorCode::USBControllerNotFound => Self::USBControllerNotFound,
      ffi::VZErrorCode::DeviceAlreadyAttached => Self::DeviceAlreadyAttached,
      ffi::VZErrorCode::DeviceInitializationFailure => Self::DeviceInitializationFailure,
      ffi::VZErrorCode::DeviceNotFound => Self::DeviceNotFound,
      ffi::VZErrorCode::GuestProvisioningInvalidFullName => Self::GuestProvisioningInvalidFullName,
      ffi::VZErrorCode::GuestProvisioningInvalidUsername => Self::GuestProvisioningInvalidUsername,
      ffi::VZErrorCode::GuestProvisioningInvalidPassword => Self::GuestProvisioningInvalidPassword,
      ffi::VZErrorCode::EFISecureBootEnrollmentFailed => Self::EFISecureBootEnrollmentFailed,
      ffi::VZErrorCode::EFIVariableInaccessible => Self::EFIVariableInaccessible,
      ffi::VZErrorCode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ErrorCode> for ffi::VZErrorCode {
  fn from(code: ErrorCode) -> Self {
    match code {
      ErrorCode::InternalError => Self::InternalError,
      ErrorCode::InvalidVirtualMachineConfiguration => Self::InvalidVirtualMachineConfiguration,
      ErrorCode::InvalidVirtualMachineState => Self::InvalidVirtualMachineState,
      ErrorCode::InvalidVirtualMachineStateTransition => Self::InvalidVirtualMachineStateTransition,
      ErrorCode::InvalidDiskImage => Self::InvalidDiskImage,
      ErrorCode::VirtualMachineLimitExceeded => Self::VirtualMachineLimitExceeded,
      ErrorCode::NetworkError => Self::NetworkError,
      ErrorCode::OutOfDiskSpace => Self::OutOfDiskSpace,
      ErrorCode::OperationCancelled => Self::OperationCancelled,
      ErrorCode::NotSupported => Self::NotSupported,
      ErrorCode::Save => Self::Save,
      ErrorCode::Restore => Self::Restore,
      ErrorCode::RestoreImageCatalogLoadFailed => Self::RestoreImageCatalogLoadFailed,
      ErrorCode::InvalidRestoreImageCatalog => Self::InvalidRestoreImageCatalog,
      ErrorCode::NoSupportedRestoreImagesInCatalog => Self::NoSupportedRestoreImagesInCatalog,
      ErrorCode::RestoreImageLoadFailed => Self::RestoreImageLoadFailed,
      ErrorCode::InvalidRestoreImage => Self::InvalidRestoreImage,
      ErrorCode::InstallationRequiresUpdate => Self::InstallationRequiresUpdate,
      ErrorCode::InstallationFailed => Self::InstallationFailed,
      ErrorCode::NetworkBlockDeviceNegotiationFailed => Self::NetworkBlockDeviceNegotiationFailed,
      ErrorCode::NetworkBlockDeviceDisconnected => Self::NetworkBlockDeviceDisconnected,
      ErrorCode::USBControllerNotFound => Self::USBControllerNotFound,
      ErrorCode::DeviceAlreadyAttached => Self::DeviceAlreadyAttached,
      ErrorCode::DeviceInitializationFailure => Self::DeviceInitializationFailure,
      ErrorCode::DeviceNotFound => Self::DeviceNotFound,
      ErrorCode::GuestProvisioningInvalidFullName => Self::GuestProvisioningInvalidFullName,
      ErrorCode::GuestProvisioningInvalidUsername => Self::GuestProvisioningInvalidUsername,
      ErrorCode::GuestProvisioningInvalidPassword => Self::GuestProvisioningInvalidPassword,
      ErrorCode::EFISecureBootEnrollmentFailed => Self::EFISecureBootEnrollmentFailed,
      ErrorCode::EFIVariableInaccessible => Self::EFIVariableInaccessible,
      ErrorCode::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;
  use std::error::Error as _;

  use super::*;

  fn ns_error(underlying_errors: Vec<NSError>) -> NSError {
    NSError {
      domain: "VZErrorDomain".to_string(),
      code: 2,
      user_info: BTreeMap::new(),
      localized_description: "The storage device attachment is invalid.".to_string(),
      localized_failure_reason: None,
      localized_recovery_suggestion: None,
      underlying_errors,
    }
  }

  #[test]
  fn says_virtualization_is_macos_only() {
    assert_eq!(
      Error::Unsupported.to_string(),
      "Virtualization is only available on macOS"
    );
  }

  #[test]
  fn says_what_virtualization_threw() {
    assert_eq!(
      Error::NSError(Box::new(ns_error(vec![]))).to_string(),
      "The storage device attachment is invalid. (VZErrorDomain 2)"
    );
  }

  #[test]
  fn says_why_a_url_did_not_parse_and_has_the_parse_error_as_its_source() {
    let error = Error::UnparseableUrl(url::ParseError::RelativeUrlWithoutBase);

    assert_eq!(
      error.to_string(),
      "Foundation returned an unparseable URL: relative URL without a base"
    );
    assert_eq!(
      error.source().unwrap().to_string(),
      url::ParseError::RelativeUrlWithoutBase.to_string()
    );
  }

  #[test]
  fn names_a_url_foundation_rejects() {
    let url = Url::parse("file:///nowhere").unwrap();

    assert_eq!(
      Error::UnrepresentableUrl(url).to_string(),
      "Foundation can't represent the URL file:///nowhere"
    );
  }

  #[test]
  fn has_an_nserrors_first_underlying_error_as_its_source() {
    let underlying = ns_error(vec![]);
    let error = Error::NSError(Box::new(ns_error(vec![underlying.clone()])));

    assert_eq!(error.source().unwrap().to_string(), underlying.to_string());
    assert!(
      Error::NSError(Box::new(ns_error(vec![])))
        .source()
        .is_none()
    );
  }

  #[test]
  fn maps_error_codes_both_ways() {
    for code in [
      ErrorCode::InternalError,
      ErrorCode::InvalidVirtualMachineConfiguration,
      ErrorCode::InvalidVirtualMachineState,
      ErrorCode::InvalidVirtualMachineStateTransition,
      ErrorCode::InvalidDiskImage,
      ErrorCode::VirtualMachineLimitExceeded,
      ErrorCode::NetworkError,
      ErrorCode::OutOfDiskSpace,
      ErrorCode::OperationCancelled,
      ErrorCode::NotSupported,
      ErrorCode::Save,
      ErrorCode::Restore,
      ErrorCode::RestoreImageCatalogLoadFailed,
      ErrorCode::InvalidRestoreImageCatalog,
      ErrorCode::NoSupportedRestoreImagesInCatalog,
      ErrorCode::RestoreImageLoadFailed,
      ErrorCode::InvalidRestoreImage,
      ErrorCode::InstallationRequiresUpdate,
      ErrorCode::InstallationFailed,
      ErrorCode::NetworkBlockDeviceNegotiationFailed,
      ErrorCode::NetworkBlockDeviceDisconnected,
      ErrorCode::USBControllerNotFound,
      ErrorCode::DeviceAlreadyAttached,
      ErrorCode::DeviceInitializationFailure,
      ErrorCode::DeviceNotFound,
      ErrorCode::GuestProvisioningInvalidFullName,
      ErrorCode::GuestProvisioningInvalidUsername,
      ErrorCode::GuestProvisioningInvalidPassword,
      ErrorCode::EFISecureBootEnrollmentFailed,
      ErrorCode::EFIVariableInaccessible,
      ErrorCode::Unknown(99),
    ] {
      assert_eq!(ErrorCode::from(ffi::VZErrorCode::from(code)), code);
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(error_domain(), Err(Error::Unsupported)));
    assert!(matches!(ErrorCode::new(1), Err(Error::Unsupported)));
    assert!(matches!(ErrorCode::InternalError.raw_value(), Err(Error::Unsupported)));
  }

  #[test]
  fn has_no_source_when_nothing_underlies_it() {
    assert!(Error::Unsupported.source().is_none());
    assert!(
      Error::UnrepresentableUrl(Url::parse("file:///nowhere").unwrap())
        .source()
        .is_none()
    );
  }
}
