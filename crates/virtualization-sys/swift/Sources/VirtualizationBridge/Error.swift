//===----------------------------------------------------------------------===//
// `VZError`: its codes, and `VZErrorDomain`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_error_domain() -> RustString {
  VZErrorDomain.intoRustString()
}

/// `VZError.Code(rawValue:)`: `Unknown` for a raw value with no case.
func vz_error_code_init_raw_value(raw_value: Int) -> BridgedVZErrorCode {
  BridgedVZErrorCode(VZError.Code(rawValue: raw_value)!)
}

func vz_error_code_raw_value(code: BridgedVZErrorCode) -> Int {
  VZError.Code(code).rawValue
}

// An Objective-C enum's `init(rawValue:)` accepts any value: these unwraps
// can't fail.

extension VZError.Code {
  init(_ code: BridgedVZErrorCode) {
    switch code {
    case .InternalError: self = .internalError
    case .InvalidVirtualMachineConfiguration: self = .invalidVirtualMachineConfiguration
    case .InvalidVirtualMachineState: self = .invalidVirtualMachineState
    case .InvalidVirtualMachineStateTransition: self = .invalidVirtualMachineStateTransition
    case .InvalidDiskImage: self = .invalidDiskImage
    case .VirtualMachineLimitExceeded: self = .virtualMachineLimitExceeded
    case .NetworkError: self = .networkError
    case .OutOfDiskSpace: self = .outOfDiskSpace
    case .OperationCancelled: self = .operationCancelled
    case .NotSupported: self = .notSupported
    case .Save: self = .save
    case .Restore: self = .restore
    case .RestoreImageCatalogLoadFailed: self = .restoreImageCatalogLoadFailed
    case .InvalidRestoreImageCatalog: self = .invalidRestoreImageCatalog
    case .NoSupportedRestoreImagesInCatalog: self = .noSupportedRestoreImagesInCatalog
    case .RestoreImageLoadFailed: self = .restoreImageLoadFailed
    case .InvalidRestoreImage: self = .invalidRestoreImage
    case .InstallationRequiresUpdate: self = .installationRequiresUpdate
    case .InstallationFailed: self = .installationFailed
    case .NetworkBlockDeviceNegotiationFailed: self = .networkBlockDeviceNegotiationFailed
    case .NetworkBlockDeviceDisconnected: self = .networkBlockDeviceDisconnected
    case .USBControllerNotFound: self = .usbControllerNotFound
    case .DeviceAlreadyAttached: self = .deviceAlreadyAttached
    case .DeviceInitializationFailure: self = .deviceInitializationFailure
    case .DeviceNotFound: self = .deviceNotFound
    case .GuestProvisioningInvalidFullName: self = .guestProvisioningInvalidFullName
    case .GuestProvisioningInvalidUsername: self = .guestProvisioningInvalidUsername
    case .GuestProvisioningInvalidPassword: self = .guestProvisioningInvalidPassword
    case .EFISecureBootEnrollmentFailed: self = .efiSecureBootEnrollmentFailed
    case .EFIVariableInaccessible: self = .efiVariableInaccessible
    case .Unknown(let rawValue): self = VZError.Code(rawValue: rawValue)!
    }
  }
}

extension BridgedVZErrorCode {
  init(_ code: VZError.Code) {
    switch code {
    case .internalError: self = .InternalError
    case .invalidVirtualMachineConfiguration: self = .InvalidVirtualMachineConfiguration
    case .invalidVirtualMachineState: self = .InvalidVirtualMachineState
    case .invalidVirtualMachineStateTransition: self = .InvalidVirtualMachineStateTransition
    case .invalidDiskImage: self = .InvalidDiskImage
    case .virtualMachineLimitExceeded: self = .VirtualMachineLimitExceeded
    case .networkError: self = .NetworkError
    case .outOfDiskSpace: self = .OutOfDiskSpace
    case .operationCancelled: self = .OperationCancelled
    case .notSupported: self = .NotSupported
    case .save: self = .Save
    case .restore: self = .Restore
    case .restoreImageCatalogLoadFailed: self = .RestoreImageCatalogLoadFailed
    case .invalidRestoreImageCatalog: self = .InvalidRestoreImageCatalog
    case .noSupportedRestoreImagesInCatalog: self = .NoSupportedRestoreImagesInCatalog
    case .restoreImageLoadFailed: self = .RestoreImageLoadFailed
    case .invalidRestoreImage: self = .InvalidRestoreImage
    case .installationRequiresUpdate: self = .InstallationRequiresUpdate
    case .installationFailed: self = .InstallationFailed
    case .networkBlockDeviceNegotiationFailed: self = .NetworkBlockDeviceNegotiationFailed
    case .networkBlockDeviceDisconnected: self = .NetworkBlockDeviceDisconnected
    case .usbControllerNotFound: self = .USBControllerNotFound
    case .deviceAlreadyAttached: self = .DeviceAlreadyAttached
    case .deviceInitializationFailure: self = .DeviceInitializationFailure
    case .deviceNotFound: self = .DeviceNotFound
    case .guestProvisioningInvalidFullName: self = .GuestProvisioningInvalidFullName
    case .guestProvisioningInvalidUsername: self = .GuestProvisioningInvalidUsername
    case .guestProvisioningInvalidPassword: self = .GuestProvisioningInvalidPassword
    case .efiSecureBootEnrollmentFailed: self = .EFISecureBootEnrollmentFailed
    case .efiVariableInaccessible: self = .EFIVariableInaccessible
    @unknown default: self = .Unknown(code.rawValue)
    }
  }
}
