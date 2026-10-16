//! FFI to Apple's [Virtualization] framework, through swift-bridge. Use the
//! `virtualization` crate instead; this one holds every `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them:
//!
//! - Classes keep Apple's names. Members are methods `vz_<member>` and
//!   `vz_set_<member>`; Foundation's are `ns_<member>`.
//! - Initializers and class members are free functions:
//!   `vz_<type>_init[_<later labels>]`, `vz_<type>_<member>`.
//! - Foundation's objects are objc2's, through `foundation-sys`: `URL` is
//!   [`Retained<NSURL>`], and a thrown error is [`Retained<NSError>`]. They
//!   cross the bridge as pointers, which only this crate handles: the bridge
//!   module is private, and each function that takes or returns one is
//!   wrapped here.
//! - `bridge_*` members cross what swift-bridge can't: `bridge_retained` is
//!   another reference to the same object (opaque Swift types pass only by
//!   value).
//! - An abstract class is [`NSObject`]: swift-bridge has no subclassing.
//!   `bridge_object` lends a class's object as one, and
//!   `vz_<type>_bridge_cast` is Swift's `as?` back to the class.
//! - An array of an abstract class crosses as an `NSArray`, and is a slice
//!   of [`NSObject`]s going in and a `Vec` coming out.
//! - swift-bridge can't pass `Option` of an opaque Swift type. One returned is
//!   `Result<T, BridgedNil>`.
//! - A throwing function returns `Result<T, BridgedThrown>`, and
//!   [`bridge_take_thrown_error`] then returns the `NSError` it threw.
//!   (swift-bridge 0.1.59 calls a function whose `Result` has an opaque type
//!   on both sides more than once, so the error can't be the `Err`.)
//! - Enums keep Apple's names and cases, plus `Unknown(rawValue)` for a case
//!   newer than this crate.
//!
//! One bridge module, because swift-bridge can't share an opaque Swift type
//! between modules. It is grouped as the Swift package is.
//!
//! macOS only: the build fails elsewhere.
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

use std::any::Any;
use std::ffi::c_void;
use std::os::fd::{BorrowedFd, OwnedFd};
use std::ptr::{self, NonNull};
use std::sync::Arc;

use dispatch2::{DispatchQueue, DispatchRetained};
use foundation_sys::objc::{
  AnyObject, NSArray, NSData, NSDictionary, NSNull, NSString, borrowed, borrowed_or_nil, ns_array, ns_array_objects,
  ns_data, ns_data_bytes, ns_description, ns_dictionary, ns_dictionary_entries, retained, retained_or_nil,
};
pub use foundation_sys::objc::{
  NSError, NSObject, NSURL, NSUUID, Retained, ns_url_absolute_string, ns_url_init_string, ns_uuid_init_uuid_string,
  ns_uuid_uuid_string,
};
use foundation_sys::objc::{
  NSFileHandle, ns_file_handle_file_descriptor, ns_file_handle_init_file_descriptor_close_on_dealloc,
};

pub use self::ffi::{
  BridgedNil, BridgedThrown, VZBridgedNetworkDeviceAttachment, VZBridgedNetworkInterface,
  VZDiskBlockDeviceStorageDeviceAttachment, VZDiskImageCachingMode, VZDiskImageStorageDeviceAttachment,
  VZDiskImageSynchronizationMode, VZDiskSynchronizationMode, VZEFIBootLoader, VZEFISignatureDatabaseConfiguration,
  VZEFISignatureList, VZEFIVariableStore, VZFileHandleNetworkDeviceAttachment, VZFileHandleSerialPortAttachment,
  VZFileSerialPortAttachment, VZGenericMachineIdentifier, VZGenericPlatformConfiguration, VZLinuxBootLoader,
  VZMACAddress, VZMultipleDirectoryShare, VZNATNetworkDeviceAttachment, VZNVMExpressControllerDeviceConfiguration,
  VZNetworkBlockDeviceStorageDeviceAttachment, VZNetworkDevice, VZSharedDirectory, VZSingleDirectoryShare,
  VZSpiceAgentPortAttachment, VZUSBMassStorageDevice, VZUSBMassStorageDeviceConfiguration, VZUSBPassthroughDevice,
  VZUSBPassthroughDeviceConfiguration, VZVirtioBlockDeviceConfiguration, VZVirtioConsoleDevice,
  VZVirtioConsoleDeviceConfiguration, VZVirtioConsoleDeviceSerialPortConfiguration, VZVirtioConsolePort,
  VZVirtioConsolePortArray, VZVirtioConsolePortConfiguration, VZVirtioConsolePortConfigurationArray,
  VZVirtioEntropyDeviceConfiguration, VZVirtioFileSystemDevice, VZVirtioFileSystemDeviceConfiguration,
  VZVirtioNetworkDeviceConfiguration, VZVirtioSocketConnection, VZVirtioSocketDevice,
  VZVirtioSocketDeviceConfiguration, VZVirtioSocketListener, VZVirtioTraditionalMemoryBalloonDevice,
  VZVirtioTraditionalMemoryBalloonDeviceConfiguration, VZVirtualMachine, VZVirtualMachineConfiguration,
  VZVirtualMachineState, VZVmnetNetworkDeviceAttachment, VZXHCIController, VZXHCIControllerConfiguration,
  vz_efi_boot_loader_init, vz_efi_variable_store_initialization_options_allow_overwrite,
  vz_generic_machine_identifier_init, vz_generic_platform_configuration_init,
  vz_generic_platform_configuration_is_nested_virtualization_supported, vz_mac_address_random_locally_administered,
  vz_multiple_directory_share_canonicalized_name, vz_multiple_directory_share_init,
  vz_multiple_directory_share_validate_name, vz_nat_network_device_attachment_init,
  vz_spice_agent_port_attachment_init, vz_spice_agent_port_attachment_spice_agent_port_name,
  vz_virtio_block_device_configuration_validate_block_device_identifier, vz_virtio_console_device_configuration_init,
  vz_virtio_console_device_serial_port_configuration_init, vz_virtio_console_port_configuration_init,
  vz_virtio_entropy_device_configuration_init, vz_virtio_file_system_device_configuration_init,
  vz_virtio_file_system_device_configuration_macos_guest_automount_tag,
  vz_virtio_file_system_device_configuration_validate_tag, vz_virtio_network_device_configuration_init,
  vz_virtio_socket_device_configuration_init, vz_virtio_socket_listener_init,
  vz_virtio_traditional_memory_balloon_device_configuration_init, vz_virtual_machine_configuration_init,
  vz_virtual_machine_configuration_maximum_allowed_cpu_count,
  vz_virtual_machine_configuration_maximum_allowed_memory_size,
  vz_virtual_machine_configuration_minimum_allowed_cpu_count,
  vz_virtual_machine_configuration_minimum_allowed_memory_size, vz_virtual_machine_is_supported,
  vz_xhci_controller_configuration_init,
};

// No doc comments inside the module: swift-bridge can't parse them. Shared
// enums are renamed on the Swift side, where Apple's names are taken.
//
// A function with a Foundation pointer takes the object it is a member of by
// value: swift-bridge 0.1.59 releases an object a free function borrows.
#[swift_bridge::bridge]
mod ffi {
  // Foundation.swift

  enum BridgedNil {
    Nil,
  }

  enum BridgedThrown {
    Thrown,
  }

  extern "Swift" {
    fn bridge_take_thrown_error() -> *mut c_void;
  }

  // Security.swift

  extern "Swift" {
    fn sec_certificate_create_with_data(data: *const c_void) -> usize;
    fn sec_certificate_copy_data(certificate: *const c_void) -> *mut c_void;
  }

  // VirtualMachine.swift

  #[swift_bridge(swift_name = "BridgedVZVirtualMachineState")]
  enum VZVirtualMachineState {
    Stopped,
    Running,
    Paused,
    Error,
    Starting,
    Pausing,
    Resuming,
    Stopping,
    Saving,
    Restoring,
    Unknown(isize),
  }

  extern "Swift" {
    type VZVirtualMachine;

    fn vz_virtual_machine_init(configuration: VZVirtualMachineConfiguration) -> VZVirtualMachine;
    fn vz_virtual_machine_init_queue(
      configuration: VZVirtualMachineConfiguration,
      queue: *const c_void,
    ) -> VZVirtualMachine;
    fn bridge_retained(self: &VZVirtualMachine) -> VZVirtualMachine;
    fn ns_description(self: &VZVirtualMachine) -> String;
    fn vz_virtual_machine_is_supported() -> bool;
    fn vz_virtual_machine_queue(this: VZVirtualMachine) -> *mut c_void;
    fn vz_state(self: &VZVirtualMachine) -> VZVirtualMachineState;
    fn vz_virtual_machine_delegate(this: VZVirtualMachine) -> usize;
    fn vz_virtual_machine_set_delegate(this: VZVirtualMachine, delegate: VZVirtualMachineDelegateBridge);
    fn bridge_set_delegate_nil(self: &VZVirtualMachine);
    fn vz_can_start(self: &VZVirtualMachine) -> bool;
    fn vz_can_stop(self: &VZVirtualMachine) -> bool;
    fn vz_can_pause(self: &VZVirtualMachine) -> bool;
    fn vz_can_resume(self: &VZVirtualMachine) -> bool;
    fn vz_can_request_stop(self: &VZVirtualMachine) -> bool;
    fn vz_virtual_machine_console_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_directory_sharing_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_graphics_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_memory_balloon_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_network_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_socket_devices(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_usb_controllers(this: VZVirtualMachine) -> *mut c_void;
    fn vz_virtual_machine_start(this: VZVirtualMachine, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_virtual_machine_start_options(
      this: VZVirtualMachine,
      options: *const c_void,
      completion_handler: Box<dyn FnOnce(usize)>,
    );
    fn vz_virtual_machine_stop(this: VZVirtualMachine, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_virtual_machine_pause(this: VZVirtualMachine, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_virtual_machine_resume(this: VZVirtualMachine, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_request_stop(self: &VZVirtualMachine) -> Result<(), BridgedThrown>;
    fn vz_virtual_machine_restore_machine_state_from(
      this: VZVirtualMachine,
      url: *const c_void,
      completion_handler: Box<dyn FnOnce(usize)>,
    );
    fn vz_virtual_machine_save_machine_state_to(
      this: VZVirtualMachine,
      url: *const c_void,
      completion_handler: Box<dyn FnOnce(usize)>,
    );
  }

  extern "Rust" {
    type VZVirtualMachineDelegateBridge;

    fn bridge_retained(self: &VZVirtualMachineDelegateBridge) -> usize;
    fn guest_did_stop(self: &VZVirtualMachineDelegateBridge, virtual_machine: VZVirtualMachine);
    fn virtual_machine_did_stop_with_error(
      self: &VZVirtualMachineDelegateBridge,
      virtual_machine: VZVirtualMachine,
      error: *mut c_void,
    );
    fn virtual_machine_network_device_attachment_was_disconnected_with_error(
      self: &VZVirtualMachineDelegateBridge,
      virtual_machine: VZVirtualMachine,
      network_device: VZNetworkDevice,
      error: *mut c_void,
    );
  }

  // VirtualMachineConfiguration.swift

  extern "Swift" {
    type VZVirtualMachineConfiguration;

    fn vz_virtual_machine_configuration_init() -> VZVirtualMachineConfiguration;
    fn bridge_retained(self: &VZVirtualMachineConfiguration) -> VZVirtualMachineConfiguration;
    fn ns_description(self: &VZVirtualMachineConfiguration) -> String;
    fn vz_virtual_machine_configuration_minimum_allowed_memory_size() -> u64;
    fn vz_virtual_machine_configuration_maximum_allowed_memory_size() -> u64;
    fn vz_virtual_machine_configuration_minimum_allowed_cpu_count() -> isize;
    fn vz_virtual_machine_configuration_maximum_allowed_cpu_count() -> isize;
    fn vz_virtual_machine_configuration_boot_loader(this: VZVirtualMachineConfiguration) -> usize;
    fn vz_virtual_machine_configuration_set_boot_loader(this: VZVirtualMachineConfiguration, boot_loader: usize);
    fn vz_virtual_machine_configuration_platform(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_platform(this: VZVirtualMachineConfiguration, platform: *const c_void);
    fn vz_virtual_machine_configuration_audio_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_audio_devices(
      this: VZVirtualMachineConfiguration,
      audio_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_console_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_console_devices(
      this: VZVirtualMachineConfiguration,
      console_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_directory_sharing_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_directory_sharing_devices(
      this: VZVirtualMachineConfiguration,
      directory_sharing_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_entropy_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_entropy_devices(
      this: VZVirtualMachineConfiguration,
      entropy_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_memory_balloon_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_memory_balloon_devices(
      this: VZVirtualMachineConfiguration,
      memory_balloon_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_network_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_network_devices(
      this: VZVirtualMachineConfiguration,
      network_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_serial_ports(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_serial_ports(
      this: VZVirtualMachineConfiguration,
      serial_ports: *const c_void,
    );
    fn vz_virtual_machine_configuration_socket_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_socket_devices(
      this: VZVirtualMachineConfiguration,
      socket_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_storage_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_storage_devices(
      this: VZVirtualMachineConfiguration,
      storage_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_keyboards(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_keyboards(this: VZVirtualMachineConfiguration, keyboards: *const c_void);
    fn vz_virtual_machine_configuration_pointing_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_pointing_devices(
      this: VZVirtualMachineConfiguration,
      pointing_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_graphics_devices(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_graphics_devices(
      this: VZVirtualMachineConfiguration,
      graphics_devices: *const c_void,
    );
    fn vz_virtual_machine_configuration_usb_controllers(this: VZVirtualMachineConfiguration) -> *mut c_void;
    fn vz_virtual_machine_configuration_set_usb_controllers(
      this: VZVirtualMachineConfiguration,
      usb_controllers: *const c_void,
    );
    fn vz_memory_size(self: &VZVirtualMachineConfiguration) -> u64;
    fn vz_set_memory_size(self: &VZVirtualMachineConfiguration, memory_size: u64);
    fn vz_cpu_count(self: &VZVirtualMachineConfiguration) -> isize;
    fn vz_set_cpu_count(self: &VZVirtualMachineConfiguration, cpu_count: isize);
    fn vz_label(self: &VZVirtualMachineConfiguration) -> Option<String>;
    fn vz_set_label(self: &VZVirtualMachineConfiguration, label: Option<&str>);
    fn vz_validate(self: &VZVirtualMachineConfiguration) -> Result<(), BridgedThrown>;
    fn vz_validate_save_restore_support(self: &VZVirtualMachineConfiguration) -> Result<(), BridgedThrown>;
  }

  // BootLoader/LinuxBootLoader.swift

  extern "Swift" {
    type VZLinuxBootLoader;

    fn vz_linux_boot_loader_init(kernel_url: *const c_void) -> VZLinuxBootLoader;
    fn bridge_retained(self: &VZLinuxBootLoader) -> VZLinuxBootLoader;
    fn bridge_borrowed(self: &VZLinuxBootLoader) -> *const c_void;
    fn vz_linux_boot_loader_bridge_cast(object: *const c_void) -> Result<VZLinuxBootLoader, BridgedNil>;
    fn ns_description(self: &VZLinuxBootLoader) -> String;
    fn vz_linux_boot_loader_kernel_url(this: VZLinuxBootLoader) -> *mut c_void;
    fn vz_linux_boot_loader_set_kernel_url(this: VZLinuxBootLoader, kernel_url: *const c_void);
    fn vz_command_line(self: &VZLinuxBootLoader) -> String;
    fn vz_set_command_line(self: &VZLinuxBootLoader, command_line: &str);
    fn vz_linux_boot_loader_initial_ramdisk_url(this: VZLinuxBootLoader) -> usize;
    fn vz_linux_boot_loader_set_initial_ramdisk_url(this: VZLinuxBootLoader, initial_ramdisk_url: usize);
  }

  // BootLoader/EFIBootLoader.swift

  extern "Swift" {
    type VZEFIBootLoader;

    fn vz_efi_boot_loader_init() -> VZEFIBootLoader;
    fn bridge_retained(self: &VZEFIBootLoader) -> VZEFIBootLoader;
    fn bridge_borrowed(self: &VZEFIBootLoader) -> *const c_void;
    fn vz_efi_boot_loader_bridge_cast(object: *const c_void) -> Result<VZEFIBootLoader, BridgedNil>;
    fn ns_description(self: &VZEFIBootLoader) -> String;
    fn vz_variable_store(self: &VZEFIBootLoader) -> Result<VZEFIVariableStore, BridgedNil>;
    fn vz_set_variable_store(self: &VZEFIBootLoader, variable_store: VZEFIVariableStore);
    fn bridge_set_variable_store_nil(self: &VZEFIBootLoader);
  }

  // BootLoader/EFIVariableStore.swift

  extern "Swift" {
    type VZEFIVariableStore;

    fn vz_efi_variable_store_init(url: *const c_void) -> VZEFIVariableStore;
    fn vz_efi_variable_store_init_creating_variable_store_at_options(
      url: *const c_void,
      options: usize,
    ) -> Result<VZEFIVariableStore, BridgedThrown>;
    fn vz_efi_variable_store_initialization_options_allow_overwrite() -> usize;
    fn bridge_retained(self: &VZEFIVariableStore) -> VZEFIVariableStore;
    fn ns_description(self: &VZEFIVariableStore) -> String;
    fn vz_efi_variable_store_url(this: VZEFIVariableStore) -> *mut c_void;
    fn vz_enable_secure_boot_using_default_platform_key(self: &VZEFIVariableStore) -> Result<(), BridgedThrown>;
    fn vz_efi_variable_store_enable_secure_boot(
      this: VZEFIVariableStore,
      platform_key: *const c_void,
    ) -> Result<(), BridgedThrown>;
    fn vz_disable_secure_boot(self: &VZEFIVariableStore) -> Result<(), BridgedThrown>;
    fn vz_reset_secure_boot(self: &VZEFIVariableStore) -> Result<(), BridgedThrown>;
    fn vz_is_secure_boot_enabled(self: &VZEFIVariableStore) -> Result<bool, BridgedThrown>;
    fn vz_enroll_default_secure_boot_signatures(self: &VZEFIVariableStore) -> Result<(), BridgedThrown>;
    fn vz_enroll_secure_boot_signatures(
      self: &VZEFIVariableStore,
      signatures: VZEFISignatureDatabaseConfiguration,
    ) -> Result<(), BridgedThrown>;
    fn vz_enrolled_secure_boot_signatures(
      self: &VZEFIVariableStore,
    ) -> Result<VZEFISignatureDatabaseConfiguration, BridgedThrown>;
  }

  // BootLoader/EFISignatureList.swift

  extern "Swift" {
    type VZEFISignatureList;

    fn vz_efi_signature_list_init_contents_of(url: *const c_void) -> Result<VZEFISignatureList, BridgedThrown>;
    fn vz_efi_signature_list_init_signatures(signatures: *const c_void) -> VZEFISignatureList;
    fn vz_efi_signature_list_bridge_from(object: *const c_void) -> VZEFISignatureList;
    fn bridge_retained(self: &VZEFISignatureList) -> VZEFISignatureList;
    fn bridge_borrowed(self: &VZEFISignatureList) -> *const c_void;
    fn ns_description(self: &VZEFISignatureList) -> String;
    fn vz_efi_signature_list_signatures(this: VZEFISignatureList) -> *mut c_void;
  }

  // BootLoader/EFISignatureDatabaseConfiguration.swift

  extern "Swift" {
    type VZEFISignatureDatabaseConfiguration;

    fn vz_efi_signature_database_configuration_init(
      key_exchange_keys: *const c_void,
      db_signatures: *const c_void,
      dbx_signatures: *const c_void,
    ) -> VZEFISignatureDatabaseConfiguration;
    fn bridge_retained(self: &VZEFISignatureDatabaseConfiguration) -> VZEFISignatureDatabaseConfiguration;
    fn ns_description(self: &VZEFISignatureDatabaseConfiguration) -> String;
    fn vz_efi_signature_database_configuration_key_exchange_keys(
      this: VZEFISignatureDatabaseConfiguration,
    ) -> *mut c_void;
    fn vz_efi_signature_database_configuration_db_signatures(this: VZEFISignatureDatabaseConfiguration) -> *mut c_void;
    fn vz_efi_signature_database_configuration_dbx_signatures(this: VZEFISignatureDatabaseConfiguration)
    -> *mut c_void;
  }

  // BootLoader/GenericPlatformConfiguration.swift

  extern "Swift" {
    type VZGenericPlatformConfiguration;

    fn vz_generic_platform_configuration_init() -> VZGenericPlatformConfiguration;
    fn bridge_retained(self: &VZGenericPlatformConfiguration) -> VZGenericPlatformConfiguration;
    fn bridge_borrowed(self: &VZGenericPlatformConfiguration) -> *const c_void;
    fn vz_generic_platform_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZGenericPlatformConfiguration, BridgedNil>;
    fn ns_description(self: &VZGenericPlatformConfiguration) -> String;
    fn vz_generic_platform_configuration_is_nested_virtualization_supported() -> bool;
    fn vz_machine_identifier(self: &VZGenericPlatformConfiguration) -> VZGenericMachineIdentifier;
    fn vz_set_machine_identifier(self: &VZGenericPlatformConfiguration, machine_identifier: VZGenericMachineIdentifier);
    fn vz_is_nested_virtualization_enabled(self: &VZGenericPlatformConfiguration) -> bool;
    fn vz_set_nested_virtualization_enabled(self: &VZGenericPlatformConfiguration, nested_virtualization_enabled: bool);
  }

  // BootLoader/GenericMachineIdentifier.swift

  extern "Swift" {
    type VZGenericMachineIdentifier;

    fn vz_generic_machine_identifier_init() -> VZGenericMachineIdentifier;
    fn vz_generic_machine_identifier_init_data_representation(
      data_representation: *const c_void,
    ) -> Result<VZGenericMachineIdentifier, BridgedNil>;
    fn bridge_retained(self: &VZGenericMachineIdentifier) -> VZGenericMachineIdentifier;
    fn ns_description(self: &VZGenericMachineIdentifier) -> String;
    fn ns_is_equal(self: &VZGenericMachineIdentifier, object: VZGenericMachineIdentifier) -> bool;
    fn vz_generic_machine_identifier_data_representation(this: VZGenericMachineIdentifier) -> *mut c_void;
  }

  // DirectorySharing/MultipleDirectoryShare.swift

  extern "Swift" {
    type VZMultipleDirectoryShare;

    fn vz_multiple_directory_share_init() -> VZMultipleDirectoryShare;
    fn vz_multiple_directory_share_init_directories(directories: *const c_void) -> VZMultipleDirectoryShare;
    fn bridge_retained(self: &VZMultipleDirectoryShare) -> VZMultipleDirectoryShare;
    fn bridge_borrowed(self: &VZMultipleDirectoryShare) -> *const c_void;
    fn vz_multiple_directory_share_bridge_cast(object: *const c_void) -> Result<VZMultipleDirectoryShare, BridgedNil>;
    fn ns_description(self: &VZMultipleDirectoryShare) -> String;
    fn vz_multiple_directory_share_validate_name(name: &str) -> Result<(), BridgedThrown>;
    fn vz_multiple_directory_share_canonicalized_name(name: &str) -> Option<String>;
    fn vz_multiple_directory_share_directories(this: VZMultipleDirectoryShare) -> *mut c_void;
  }

  // DirectorySharing/SharedDirectory.swift

  extern "Swift" {
    type VZSharedDirectory;

    fn vz_shared_directory_init(url: *const c_void, read_only: bool) -> VZSharedDirectory;
    fn vz_shared_directory_bridge_from(object: *const c_void) -> VZSharedDirectory;
    fn bridge_retained(self: &VZSharedDirectory) -> VZSharedDirectory;
    fn bridge_borrowed(self: &VZSharedDirectory) -> *const c_void;
    fn ns_description(self: &VZSharedDirectory) -> String;
    fn vz_shared_directory_url(this: VZSharedDirectory) -> *mut c_void;
    fn vz_is_read_only(self: &VZSharedDirectory) -> bool;
  }

  // DirectorySharing/SingleDirectoryShare.swift

  extern "Swift" {
    type VZSingleDirectoryShare;

    fn vz_single_directory_share_init(directory: VZSharedDirectory) -> VZSingleDirectoryShare;
    fn bridge_retained(self: &VZSingleDirectoryShare) -> VZSingleDirectoryShare;
    fn bridge_borrowed(self: &VZSingleDirectoryShare) -> *const c_void;
    fn vz_single_directory_share_bridge_cast(object: *const c_void) -> Result<VZSingleDirectoryShare, BridgedNil>;
    fn ns_description(self: &VZSingleDirectoryShare) -> String;
    fn vz_directory(self: &VZSingleDirectoryShare) -> VZSharedDirectory;
  }

  // DirectorySharing/VirtioFileSystemDevice.swift

  extern "Swift" {
    type VZVirtioFileSystemDevice;

    fn bridge_retained(self: &VZVirtioFileSystemDevice) -> VZVirtioFileSystemDevice;
    fn bridge_borrowed(self: &VZVirtioFileSystemDevice) -> *const c_void;
    fn vz_virtio_file_system_device_bridge_cast(object: *const c_void) -> Result<VZVirtioFileSystemDevice, BridgedNil>;
    fn ns_description(self: &VZVirtioFileSystemDevice) -> String;
    fn vz_tag(self: &VZVirtioFileSystemDevice) -> String;
    fn vz_virtio_file_system_device_share(this: VZVirtioFileSystemDevice) -> usize;
    fn vz_virtio_file_system_device_set_share(this: VZVirtioFileSystemDevice, share: usize);
  }

  // DirectorySharing/VirtioFileSystemDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioFileSystemDeviceConfiguration;

    fn vz_virtio_file_system_device_configuration_init(tag: &str) -> VZVirtioFileSystemDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioFileSystemDeviceConfiguration) -> VZVirtioFileSystemDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioFileSystemDeviceConfiguration) -> *const c_void;
    fn vz_virtio_file_system_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioFileSystemDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioFileSystemDeviceConfiguration) -> String;
    fn vz_virtio_file_system_device_configuration_validate_tag(tag: &str) -> Result<(), BridgedThrown>;
    fn vz_virtio_file_system_device_configuration_macos_guest_automount_tag() -> String;
    fn vz_tag(self: &VZVirtioFileSystemDeviceConfiguration) -> String;
    fn vz_set_tag(self: &VZVirtioFileSystemDeviceConfiguration, tag: &str);
    fn vz_virtio_file_system_device_configuration_share(this: VZVirtioFileSystemDeviceConfiguration) -> usize;
    fn vz_virtio_file_system_device_configuration_set_share(this: VZVirtioFileSystemDeviceConfiguration, share: usize);
  }

  // Network/BridgedNetworkDeviceAttachment.swift

  extern "Swift" {
    type VZBridgedNetworkDeviceAttachment;

    fn vz_bridged_network_device_attachment_init(
      interface: VZBridgedNetworkInterface,
    ) -> VZBridgedNetworkDeviceAttachment;
    fn bridge_retained(self: &VZBridgedNetworkDeviceAttachment) -> VZBridgedNetworkDeviceAttachment;
    fn bridge_borrowed(self: &VZBridgedNetworkDeviceAttachment) -> *const c_void;
    fn vz_bridged_network_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZBridgedNetworkDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZBridgedNetworkDeviceAttachment) -> String;
    fn vz_interface(self: &VZBridgedNetworkDeviceAttachment) -> VZBridgedNetworkInterface;
  }

  // Network/BridgedNetworkInterface.swift

  extern "Swift" {
    type VZBridgedNetworkInterface;

    fn vz_bridged_network_interface_bridge_from(object: *const c_void) -> VZBridgedNetworkInterface;
    fn vz_bridged_network_interface_network_interfaces() -> *mut c_void;
    fn bridge_retained(self: &VZBridgedNetworkInterface) -> VZBridgedNetworkInterface;
    fn ns_description(self: &VZBridgedNetworkInterface) -> String;
    fn vz_identifier(self: &VZBridgedNetworkInterface) -> String;
    fn vz_localized_display_name(self: &VZBridgedNetworkInterface) -> Option<String>;
  }

  // Network/FileHandleNetworkDeviceAttachment.swift

  extern "Swift" {
    type VZFileHandleNetworkDeviceAttachment;

    fn vz_file_handle_network_device_attachment_init(file_handle: *const c_void)
    -> VZFileHandleNetworkDeviceAttachment;
    fn bridge_retained(self: &VZFileHandleNetworkDeviceAttachment) -> VZFileHandleNetworkDeviceAttachment;
    fn bridge_borrowed(self: &VZFileHandleNetworkDeviceAttachment) -> *const c_void;
    fn vz_file_handle_network_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZFileHandleNetworkDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZFileHandleNetworkDeviceAttachment) -> String;
    fn vz_file_handle_network_device_attachment_file_handle(this: VZFileHandleNetworkDeviceAttachment) -> *mut c_void;
    fn vz_maximum_transmission_unit(self: &VZFileHandleNetworkDeviceAttachment) -> isize;
    fn vz_set_maximum_transmission_unit(self: &VZFileHandleNetworkDeviceAttachment, maximum_transmission_unit: isize);
  }

  // Network/MACAddress.swift

  extern "Swift" {
    type VZMACAddress;

    fn vz_mac_address_init_ethernet_address(ethernet_address: u64) -> VZMACAddress;
    fn vz_mac_address_init_string(string: &str) -> Result<VZMACAddress, BridgedNil>;
    fn vz_mac_address_random_locally_administered() -> VZMACAddress;
    fn bridge_retained(self: &VZMACAddress) -> VZMACAddress;
    fn ns_description(self: &VZMACAddress) -> String;
    fn bridge_ethernet_address(self: &VZMACAddress) -> u64;
    fn vz_string(self: &VZMACAddress) -> String;
    fn vz_is_broadcast_address(self: &VZMACAddress) -> bool;
    fn vz_is_multicast_address(self: &VZMACAddress) -> bool;
    fn vz_is_unicast_address(self: &VZMACAddress) -> bool;
    fn vz_is_locally_administered_address(self: &VZMACAddress) -> bool;
    fn vz_is_universally_administered_address(self: &VZMACAddress) -> bool;
  }

  // Network/NATNetworkDeviceAttachment.swift

  extern "Swift" {
    type VZNATNetworkDeviceAttachment;

    fn vz_nat_network_device_attachment_init() -> VZNATNetworkDeviceAttachment;
    fn bridge_retained(self: &VZNATNetworkDeviceAttachment) -> VZNATNetworkDeviceAttachment;
    fn bridge_borrowed(self: &VZNATNetworkDeviceAttachment) -> *const c_void;
    fn vz_nat_network_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZNATNetworkDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZNATNetworkDeviceAttachment) -> String;
  }

  // Network/NetworkDevice.swift

  extern "Swift" {
    type VZNetworkDevice;

    fn vz_network_device_bridge_from(object: *const c_void) -> VZNetworkDevice;
    fn bridge_retained(self: &VZNetworkDevice) -> VZNetworkDevice;
    fn ns_description(self: &VZNetworkDevice) -> String;
    fn vz_network_device_attachment(this: VZNetworkDevice) -> usize;
    fn vz_network_device_set_attachment(this: VZNetworkDevice, attachment: usize);
  }

  // Network/NetworkDeviceConfiguration.swift

  extern "Swift" {
    fn vz_network_device_configuration_mac_address(this: *const c_void) -> VZMACAddress;
    fn vz_network_device_configuration_set_mac_address(this: *const c_void, mac_address: VZMACAddress);
    fn vz_network_device_configuration_attachment(this: *const c_void) -> usize;
    fn vz_network_device_configuration_set_attachment(this: *const c_void, attachment: usize);
  }

  // Network/VirtioNetworkDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioNetworkDeviceConfiguration;

    fn vz_virtio_network_device_configuration_init() -> VZVirtioNetworkDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioNetworkDeviceConfiguration) -> VZVirtioNetworkDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioNetworkDeviceConfiguration) -> *const c_void;
    fn vz_virtio_network_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioNetworkDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioNetworkDeviceConfiguration) -> String;
  }

  // Network/VmnetNetworkDeviceAttachment.swift

  extern "Swift" {
    type VZVmnetNetworkDeviceAttachment;

    fn vz_vmnet_network_device_attachment_init(network: *const c_void) -> VZVmnetNetworkDeviceAttachment;
    fn bridge_retained(self: &VZVmnetNetworkDeviceAttachment) -> VZVmnetNetworkDeviceAttachment;
    fn bridge_borrowed(self: &VZVmnetNetworkDeviceAttachment) -> *const c_void;
    fn vz_vmnet_network_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVmnetNetworkDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZVmnetNetworkDeviceAttachment) -> String;
    fn vz_vmnet_network_device_attachment_network(this: VZVmnetNetworkDeviceAttachment) -> *const c_void;
  }

  // OtherDevices/USBController.swift

  extern "Swift" {
    fn vz_usb_controller_attach(this: *const c_void, device: *const c_void, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_usb_controller_detach(this: *const c_void, device: *const c_void, completion_handler: Box<dyn FnOnce(usize)>);
    fn vz_usb_controller_usb_devices(this: *const c_void) -> *mut c_void;
    fn vz_usb_controller_delegate(this: *const c_void) -> usize;
    fn vz_usb_controller_set_delegate(this: *const c_void, delegate: VZUSBControllerDelegateBridge);
    fn vz_usb_controller_set_delegate_nil(this: *const c_void);
  }

  extern "Rust" {
    type VZUSBControllerDelegateBridge;

    fn bridge_retained(self: &VZUSBControllerDelegateBridge) -> usize;
    fn usb_controller_usb_passthrough_device_did_disconnect(
      self: &VZUSBControllerDelegateBridge,
      usb_controller: usize,
      device: VZUSBPassthroughDevice,
    );
  }

  // OtherDevices/USBControllerConfiguration.swift

  extern "Swift" {
    fn vz_usb_controller_configuration_usb_devices(this: *const c_void) -> *mut c_void;
    fn vz_usb_controller_configuration_set_usb_devices(this: *const c_void, usb_devices: *const c_void);
  }

  // OtherDevices/USBDevice.swift

  extern "Swift" {
    fn vz_usb_device_usb_controller(this: *const c_void) -> usize;
    fn vz_usb_device_uuid(this: *const c_void) -> *mut c_void;
  }

  // OtherDevices/USBDeviceConfiguration.swift

  extern "Swift" {
    fn vz_usb_device_configuration_uuid(this: *const c_void) -> *mut c_void;
    fn vz_usb_device_configuration_set_uuid(this: *const c_void, uuid: *const c_void);
  }

  // OtherDevices/USBPassthroughDevice.swift

  extern "Swift" {
    type VZUSBPassthroughDevice;

    fn vz_usb_passthrough_device_init_configuration(
      configuration: VZUSBPassthroughDeviceConfiguration,
    ) -> Result<VZUSBPassthroughDevice, BridgedThrown>;
    fn bridge_retained(self: &VZUSBPassthroughDevice) -> VZUSBPassthroughDevice;
    fn bridge_borrowed(self: &VZUSBPassthroughDevice) -> *const c_void;
    fn vz_usb_passthrough_device_bridge_cast(object: *const c_void) -> Result<VZUSBPassthroughDevice, BridgedNil>;
    fn ns_description(self: &VZUSBPassthroughDevice) -> String;
  }

  // OtherDevices/USBPassthroughDeviceConfiguration.swift

  extern "Swift" {
    type VZUSBPassthroughDeviceConfiguration;

    fn vz_usb_passthrough_device_configuration_init(device: *const c_void) -> VZUSBPassthroughDeviceConfiguration;
    fn bridge_retained(self: &VZUSBPassthroughDeviceConfiguration) -> VZUSBPassthroughDeviceConfiguration;
    fn bridge_borrowed(self: &VZUSBPassthroughDeviceConfiguration) -> *const c_void;
    fn vz_usb_passthrough_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZUSBPassthroughDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZUSBPassthroughDeviceConfiguration) -> String;
  }

  // OtherDevices/VirtioEntropyDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioEntropyDeviceConfiguration;

    fn vz_virtio_entropy_device_configuration_init() -> VZVirtioEntropyDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioEntropyDeviceConfiguration) -> VZVirtioEntropyDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioEntropyDeviceConfiguration) -> *const c_void;
    fn vz_virtio_entropy_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioEntropyDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioEntropyDeviceConfiguration) -> String;
  }

  // OtherDevices/VirtioTraditionalMemoryBalloonDevice.swift

  extern "Swift" {
    type VZVirtioTraditionalMemoryBalloonDevice;

    fn bridge_retained(self: &VZVirtioTraditionalMemoryBalloonDevice) -> VZVirtioTraditionalMemoryBalloonDevice;
    fn bridge_borrowed(self: &VZVirtioTraditionalMemoryBalloonDevice) -> *const c_void;
    fn vz_virtio_traditional_memory_balloon_device_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioTraditionalMemoryBalloonDevice, BridgedNil>;
    fn ns_description(self: &VZVirtioTraditionalMemoryBalloonDevice) -> String;
    fn vz_target_virtual_machine_memory_size(self: &VZVirtioTraditionalMemoryBalloonDevice) -> u64;
    fn vz_set_target_virtual_machine_memory_size(
      self: &VZVirtioTraditionalMemoryBalloonDevice,
      target_virtual_machine_memory_size: u64,
    );
  }

  // OtherDevices/VirtioTraditionalMemoryBalloonDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioTraditionalMemoryBalloonDeviceConfiguration;

    fn vz_virtio_traditional_memory_balloon_device_configuration_init()
    -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration;
    fn bridge_retained(
      self: &VZVirtioTraditionalMemoryBalloonDeviceConfiguration,
    ) -> VZVirtioTraditionalMemoryBalloonDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioTraditionalMemoryBalloonDeviceConfiguration) -> *const c_void;
    fn vz_virtio_traditional_memory_balloon_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioTraditionalMemoryBalloonDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioTraditionalMemoryBalloonDeviceConfiguration) -> String;
  }

  // OtherDevices/XHCIController.swift

  extern "Swift" {
    type VZXHCIController;

    fn bridge_retained(self: &VZXHCIController) -> VZXHCIController;
    fn bridge_borrowed(self: &VZXHCIController) -> *const c_void;
    fn vz_xhci_controller_bridge_cast(object: *const c_void) -> Result<VZXHCIController, BridgedNil>;
    fn ns_description(self: &VZXHCIController) -> String;
  }

  // OtherDevices/XHCIControllerConfiguration.swift

  extern "Swift" {
    type VZXHCIControllerConfiguration;

    fn vz_xhci_controller_configuration_init() -> VZXHCIControllerConfiguration;
    fn bridge_retained(self: &VZXHCIControllerConfiguration) -> VZXHCIControllerConfiguration;
    fn bridge_borrowed(self: &VZXHCIControllerConfiguration) -> *const c_void;
    fn vz_xhci_controller_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZXHCIControllerConfiguration, BridgedNil>;
    fn ns_description(self: &VZXHCIControllerConfiguration) -> String;
  }

  // SerialPort/ConsolePortConfiguration.swift

  extern "Swift" {
    fn vz_console_port_configuration_attachment(this: *const c_void) -> usize;
    fn vz_console_port_configuration_set_attachment(this: *const c_void, attachment: usize);
  }

  // SerialPort/FileHandleSerialPortAttachment.swift

  extern "Swift" {
    type VZFileHandleSerialPortAttachment;

    fn vz_file_handle_serial_port_attachment_init(
      file_handle_for_reading: usize,
      file_handle_for_writing: usize,
    ) -> VZFileHandleSerialPortAttachment;
    fn bridge_retained(self: &VZFileHandleSerialPortAttachment) -> VZFileHandleSerialPortAttachment;
    fn bridge_borrowed(self: &VZFileHandleSerialPortAttachment) -> *const c_void;
    fn vz_file_handle_serial_port_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZFileHandleSerialPortAttachment, BridgedNil>;
    fn ns_description(self: &VZFileHandleSerialPortAttachment) -> String;
    fn vz_file_handle_serial_port_attachment_file_handle_for_reading(this: VZFileHandleSerialPortAttachment) -> usize;
    fn vz_file_handle_serial_port_attachment_file_handle_for_writing(this: VZFileHandleSerialPortAttachment) -> usize;
  }

  // SerialPort/FileSerialPortAttachment.swift

  extern "Swift" {
    type VZFileSerialPortAttachment;

    fn vz_file_serial_port_attachment_init(
      url: *const c_void,
      append: bool,
    ) -> Result<VZFileSerialPortAttachment, BridgedThrown>;
    fn bridge_retained(self: &VZFileSerialPortAttachment) -> VZFileSerialPortAttachment;
    fn bridge_borrowed(self: &VZFileSerialPortAttachment) -> *const c_void;
    fn vz_file_serial_port_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZFileSerialPortAttachment, BridgedNil>;
    fn ns_description(self: &VZFileSerialPortAttachment) -> String;
    fn vz_file_serial_port_attachment_url(this: VZFileSerialPortAttachment) -> *mut c_void;
    fn vz_append(self: &VZFileSerialPortAttachment) -> bool;
  }

  // SerialPort/SerialPortConfiguration.swift

  extern "Swift" {
    fn vz_serial_port_configuration_attachment(this: *const c_void) -> usize;
    fn vz_serial_port_configuration_set_attachment(this: *const c_void, attachment: usize);
  }

  // SerialPort/SpiceAgentPortAttachment.swift

  extern "Swift" {
    type VZSpiceAgentPortAttachment;

    fn vz_spice_agent_port_attachment_init() -> VZSpiceAgentPortAttachment;
    fn bridge_retained(self: &VZSpiceAgentPortAttachment) -> VZSpiceAgentPortAttachment;
    fn bridge_borrowed(self: &VZSpiceAgentPortAttachment) -> *const c_void;
    fn vz_spice_agent_port_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZSpiceAgentPortAttachment, BridgedNil>;
    fn ns_description(self: &VZSpiceAgentPortAttachment) -> String;
    fn vz_spice_agent_port_attachment_spice_agent_port_name() -> String;
    fn vz_shares_clipboard(self: &VZSpiceAgentPortAttachment) -> bool;
    fn vz_set_shares_clipboard(self: &VZSpiceAgentPortAttachment, shares_clipboard: bool);
  }

  // SerialPort/VirtioConsoleDevice.swift

  extern "Swift" {
    type VZVirtioConsoleDevice;

    fn bridge_retained(self: &VZVirtioConsoleDevice) -> VZVirtioConsoleDevice;
    fn bridge_borrowed(self: &VZVirtioConsoleDevice) -> *const c_void;
    fn vz_virtio_console_device_bridge_cast(object: *const c_void) -> Result<VZVirtioConsoleDevice, BridgedNil>;
    fn ns_description(self: &VZVirtioConsoleDevice) -> String;
    fn vz_virtio_console_device_delegate(this: VZVirtioConsoleDevice) -> usize;
    fn vz_virtio_console_device_set_delegate(
      this: VZVirtioConsoleDevice,
      delegate: VZVirtioConsoleDeviceDelegateBridge,
    );
    fn bridge_set_delegate_nil(self: &VZVirtioConsoleDevice);
    fn vz_ports(self: &VZVirtioConsoleDevice) -> VZVirtioConsolePortArray;
  }

  extern "Rust" {
    type VZVirtioConsoleDeviceDelegateBridge;

    fn bridge_retained(self: &VZVirtioConsoleDeviceDelegateBridge) -> usize;
    fn console_device_did_open(
      self: &VZVirtioConsoleDeviceDelegateBridge,
      console_device: VZVirtioConsoleDevice,
      console_port: VZVirtioConsolePort,
    );
    fn console_device_did_close(
      self: &VZVirtioConsoleDeviceDelegateBridge,
      console_device: VZVirtioConsoleDevice,
      console_port: VZVirtioConsolePort,
    );
  }

  // SerialPort/VirtioConsoleDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioConsoleDeviceConfiguration;

    fn vz_virtio_console_device_configuration_init() -> VZVirtioConsoleDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioConsoleDeviceConfiguration) -> VZVirtioConsoleDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioConsoleDeviceConfiguration) -> *const c_void;
    fn vz_virtio_console_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioConsoleDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioConsoleDeviceConfiguration) -> String;
    fn vz_ports(self: &VZVirtioConsoleDeviceConfiguration) -> VZVirtioConsolePortConfigurationArray;
  }

  // SerialPort/VirtioConsoleDeviceSerialPortConfiguration.swift

  extern "Swift" {
    type VZVirtioConsoleDeviceSerialPortConfiguration;

    fn vz_virtio_console_device_serial_port_configuration_init() -> VZVirtioConsoleDeviceSerialPortConfiguration;
    fn bridge_retained(
      self: &VZVirtioConsoleDeviceSerialPortConfiguration,
    ) -> VZVirtioConsoleDeviceSerialPortConfiguration;
    fn bridge_borrowed(self: &VZVirtioConsoleDeviceSerialPortConfiguration) -> *const c_void;
    fn vz_virtio_console_device_serial_port_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioConsoleDeviceSerialPortConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioConsoleDeviceSerialPortConfiguration) -> String;
  }

  // SerialPort/VirtioConsolePort.swift

  extern "Swift" {
    type VZVirtioConsolePort;

    fn bridge_retained(self: &VZVirtioConsolePort) -> VZVirtioConsolePort;
    fn ns_description(self: &VZVirtioConsolePort) -> String;
    fn vz_name(self: &VZVirtioConsolePort) -> Option<String>;
    fn vz_virtio_console_port_attachment(this: VZVirtioConsolePort) -> usize;
    fn vz_virtio_console_port_set_attachment(this: VZVirtioConsolePort, attachment: usize);
  }

  // SerialPort/VirtioConsolePortArray.swift

  extern "Swift" {
    type VZVirtioConsolePortArray;

    fn bridge_retained(self: &VZVirtioConsolePortArray) -> VZVirtioConsolePortArray;
    fn ns_description(self: &VZVirtioConsolePortArray) -> String;
    fn vz_maximum_port_count(self: &VZVirtioConsolePortArray) -> u32;
    fn vz_subscript(self: &VZVirtioConsolePortArray, port_index: isize) -> Result<VZVirtioConsolePort, BridgedNil>;
  }

  // SerialPort/VirtioConsolePortConfiguration.swift

  extern "Swift" {
    type VZVirtioConsolePortConfiguration;

    fn vz_virtio_console_port_configuration_init() -> VZVirtioConsolePortConfiguration;
    fn bridge_retained(self: &VZVirtioConsolePortConfiguration) -> VZVirtioConsolePortConfiguration;
    fn bridge_borrowed(self: &VZVirtioConsolePortConfiguration) -> *const c_void;
    fn vz_virtio_console_port_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioConsolePortConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioConsolePortConfiguration) -> String;
    fn vz_name(self: &VZVirtioConsolePortConfiguration) -> Option<String>;
    fn vz_set_name(self: &VZVirtioConsolePortConfiguration, name: Option<&str>);
    fn vz_is_console(self: &VZVirtioConsolePortConfiguration) -> bool;
    fn vz_set_is_console(self: &VZVirtioConsolePortConfiguration, is_console: bool);
  }

  // SerialPort/VirtioConsolePortConfigurationArray.swift

  extern "Swift" {
    type VZVirtioConsolePortConfigurationArray;

    fn bridge_retained(self: &VZVirtioConsolePortConfigurationArray) -> VZVirtioConsolePortConfigurationArray;
    fn ns_description(self: &VZVirtioConsolePortConfigurationArray) -> String;
    fn vz_maximum_port_count(self: &VZVirtioConsolePortConfigurationArray) -> u32;
    fn vz_set_maximum_port_count(self: &VZVirtioConsolePortConfigurationArray, maximum_port_count: u32);
    fn vz_subscript(
      self: &VZVirtioConsolePortConfigurationArray,
      port_index: isize,
    ) -> Result<VZVirtioConsolePortConfiguration, BridgedNil>;
    fn vz_set_subscript(
      self: &VZVirtioConsolePortConfigurationArray,
      port_index: isize,
      value: VZVirtioConsolePortConfiguration,
    );
    fn bridge_set_subscript_nil(self: &VZVirtioConsolePortConfigurationArray, port_index: isize);
  }

  // SerialPort/VirtioSocketConnection.swift

  extern "Swift" {
    type VZVirtioSocketConnection;

    fn vz_virtio_socket_connection_bridge_from(object: *const c_void) -> VZVirtioSocketConnection;
    fn bridge_retained(self: &VZVirtioSocketConnection) -> VZVirtioSocketConnection;
    fn ns_description(self: &VZVirtioSocketConnection) -> String;
    fn vz_destination_port(self: &VZVirtioSocketConnection) -> u32;
    fn vz_source_port(self: &VZVirtioSocketConnection) -> u32;
    fn vz_file_descriptor(self: &VZVirtioSocketConnection) -> i32;
    fn vz_close(self: &VZVirtioSocketConnection);
  }

  // SerialPort/VirtioSocketDevice.swift

  extern "Swift" {
    type VZVirtioSocketDevice;

    fn bridge_retained(self: &VZVirtioSocketDevice) -> VZVirtioSocketDevice;
    fn bridge_borrowed(self: &VZVirtioSocketDevice) -> *const c_void;
    fn vz_virtio_socket_device_bridge_cast(object: *const c_void) -> Result<VZVirtioSocketDevice, BridgedNil>;
    fn ns_description(self: &VZVirtioSocketDevice) -> String;
    fn vz_set_socket_listener(self: &VZVirtioSocketDevice, listener: VZVirtioSocketListener, port: u32);
    fn vz_remove_socket_listener(self: &VZVirtioSocketDevice, port: u32);
    fn vz_virtio_socket_device_connect(
      this: VZVirtioSocketDevice,
      port: u32,
      completion_handler: Box<dyn FnOnce(usize, usize)>,
    );
  }

  // SerialPort/VirtioSocketDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioSocketDeviceConfiguration;

    fn vz_virtio_socket_device_configuration_init() -> VZVirtioSocketDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioSocketDeviceConfiguration) -> VZVirtioSocketDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioSocketDeviceConfiguration) -> *const c_void;
    fn vz_virtio_socket_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioSocketDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioSocketDeviceConfiguration) -> String;
  }

  // SerialPort/VirtioSocketListener.swift

  extern "Swift" {
    type VZVirtioSocketListener;

    fn vz_virtio_socket_listener_init() -> VZVirtioSocketListener;
    fn bridge_retained(self: &VZVirtioSocketListener) -> VZVirtioSocketListener;
    fn ns_description(self: &VZVirtioSocketListener) -> String;
    fn vz_virtio_socket_listener_delegate(this: VZVirtioSocketListener) -> usize;
    fn vz_virtio_socket_listener_set_delegate(
      this: VZVirtioSocketListener,
      delegate: VZVirtioSocketListenerDelegateBridge,
    );
    fn bridge_set_delegate_nil(self: &VZVirtioSocketListener);
  }

  extern "Rust" {
    type VZVirtioSocketListenerDelegateBridge;

    fn bridge_retained(self: &VZVirtioSocketListenerDelegateBridge) -> usize;
    fn listener_should_accept_new_connection_from(
      self: &VZVirtioSocketListenerDelegateBridge,
      listener: VZVirtioSocketListener,
      connection: VZVirtioSocketConnection,
      socket_device: VZVirtioSocketDevice,
    ) -> bool;
  }

  // Storage/DiskBlockDeviceStorageDeviceAttachment.swift

  #[swift_bridge(swift_name = "BridgedVZDiskSynchronizationMode")]
  enum VZDiskSynchronizationMode {
    Full,
    None,
    Unknown(isize),
  }

  extern "Swift" {
    type VZDiskBlockDeviceStorageDeviceAttachment;

    fn vz_disk_block_device_storage_device_attachment_init(
      file_handle: *const c_void,
      read_only: bool,
      synchronization_mode: VZDiskSynchronizationMode,
    ) -> Result<VZDiskBlockDeviceStorageDeviceAttachment, BridgedThrown>;
    fn bridge_retained(self: &VZDiskBlockDeviceStorageDeviceAttachment) -> VZDiskBlockDeviceStorageDeviceAttachment;
    fn bridge_borrowed(self: &VZDiskBlockDeviceStorageDeviceAttachment) -> *const c_void;
    fn vz_disk_block_device_storage_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZDiskBlockDeviceStorageDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZDiskBlockDeviceStorageDeviceAttachment) -> String;
    fn vz_disk_block_device_storage_device_attachment_file_handle(
      this: VZDiskBlockDeviceStorageDeviceAttachment,
    ) -> *mut c_void;
    fn vz_is_read_only(self: &VZDiskBlockDeviceStorageDeviceAttachment) -> bool;
    fn vz_synchronization_mode(self: &VZDiskBlockDeviceStorageDeviceAttachment) -> VZDiskSynchronizationMode;
  }

  // Storage/DiskImageStorageDeviceAttachment.swift

  #[swift_bridge(swift_name = "BridgedVZDiskImageCachingMode")]
  enum VZDiskImageCachingMode {
    Automatic,
    Uncached,
    Cached,
    Unknown(isize),
  }

  #[swift_bridge(swift_name = "BridgedVZDiskImageSynchronizationMode")]
  enum VZDiskImageSynchronizationMode {
    Full,
    Fsync,
    None,
    Unknown(isize),
  }

  extern "Swift" {
    type VZDiskImageStorageDeviceAttachment;

    fn vz_disk_image_storage_device_attachment_init(
      url: *const c_void,
      read_only: bool,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown>;
    fn vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
      url: *const c_void,
      read_only: bool,
      caching_mode: VZDiskImageCachingMode,
      synchronization_mode: VZDiskImageSynchronizationMode,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown>;
    fn vz_disk_image_storage_device_attachment_init_disk_image_caching_mode_synchronization_mode(
      disk_image: *const c_void,
      caching_mode: VZDiskImageCachingMode,
      synchronization_mode: VZDiskImageSynchronizationMode,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown>;
    fn bridge_retained(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageStorageDeviceAttachment;
    fn bridge_borrowed(self: &VZDiskImageStorageDeviceAttachment) -> *const c_void;
    fn vz_disk_image_storage_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZDiskImageStorageDeviceAttachment) -> String;
    fn vz_disk_image_storage_device_attachment_url(this: VZDiskImageStorageDeviceAttachment) -> *mut c_void;
    fn vz_is_read_only(self: &VZDiskImageStorageDeviceAttachment) -> bool;
    fn vz_caching_mode(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageCachingMode;
    fn vz_synchronization_mode(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageSynchronizationMode;
  }

  // Storage/NVMExpressControllerDeviceConfiguration.swift

  extern "Swift" {
    type VZNVMExpressControllerDeviceConfiguration;

    fn vz_nvm_express_controller_device_configuration_init(
      attachment: *const c_void,
    ) -> VZNVMExpressControllerDeviceConfiguration;
    fn bridge_retained(self: &VZNVMExpressControllerDeviceConfiguration) -> VZNVMExpressControllerDeviceConfiguration;
    fn bridge_borrowed(self: &VZNVMExpressControllerDeviceConfiguration) -> *const c_void;
    fn vz_nvm_express_controller_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZNVMExpressControllerDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZNVMExpressControllerDeviceConfiguration) -> String;
  }

  // Storage/NetworkBlockDeviceStorageDeviceAttachment.swift

  extern "Swift" {
    type VZNetworkBlockDeviceStorageDeviceAttachment;

    fn vz_network_block_device_storage_device_attachment_init(
      url: *const c_void,
    ) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown>;
    fn vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
      url: *const c_void,
      timeout: f64,
      is_forced_read_only: bool,
      synchronization_mode: VZDiskSynchronizationMode,
    ) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown>;
    fn bridge_retained(
      self: &VZNetworkBlockDeviceStorageDeviceAttachment,
    ) -> VZNetworkBlockDeviceStorageDeviceAttachment;
    fn bridge_borrowed(self: &VZNetworkBlockDeviceStorageDeviceAttachment) -> *const c_void;
    fn vz_network_block_device_storage_device_attachment_bridge_cast(
      object: *const c_void,
    ) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedNil>;
    fn ns_description(self: &VZNetworkBlockDeviceStorageDeviceAttachment) -> String;
    fn vz_network_block_device_storage_device_attachment_validate(url: *const c_void) -> Result<(), BridgedThrown>;
    fn vz_network_block_device_storage_device_attachment_url(
      this: VZNetworkBlockDeviceStorageDeviceAttachment,
    ) -> *mut c_void;
    fn vz_timeout(self: &VZNetworkBlockDeviceStorageDeviceAttachment) -> f64;
    fn vz_is_forced_read_only(self: &VZNetworkBlockDeviceStorageDeviceAttachment) -> bool;
    fn vz_synchronization_mode(self: &VZNetworkBlockDeviceStorageDeviceAttachment) -> VZDiskSynchronizationMode;
    fn vz_network_block_device_storage_device_attachment_delegate(
      this: VZNetworkBlockDeviceStorageDeviceAttachment,
    ) -> usize;
    fn vz_network_block_device_storage_device_attachment_set_delegate(
      this: VZNetworkBlockDeviceStorageDeviceAttachment,
      delegate: VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge,
    );
    fn bridge_set_delegate_nil(self: &VZNetworkBlockDeviceStorageDeviceAttachment);
  }

  extern "Rust" {
    type VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge;

    fn bridge_retained(self: &VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge) -> usize;
    fn attachment_was_connected(
      self: &VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge,
      attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
    );
    fn attachment_did_encounter_error(
      self: &VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge,
      attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
      error: *mut c_void,
    );
  }

  // Storage/StorageDeviceConfiguration.swift

  extern "Swift" {
    fn vz_storage_device_configuration_attachment(this: *const c_void) -> *mut c_void;
  }

  // Storage/USBMassStorageDevice.swift

  extern "Swift" {
    type VZUSBMassStorageDevice;

    fn vz_usb_mass_storage_device_init(configuration: VZUSBMassStorageDeviceConfiguration) -> VZUSBMassStorageDevice;
    fn bridge_retained(self: &VZUSBMassStorageDevice) -> VZUSBMassStorageDevice;
    fn bridge_borrowed(self: &VZUSBMassStorageDevice) -> *const c_void;
    fn vz_usb_mass_storage_device_bridge_cast(object: *const c_void) -> Result<VZUSBMassStorageDevice, BridgedNil>;
    fn ns_description(self: &VZUSBMassStorageDevice) -> String;
  }

  // Storage/USBMassStorageDeviceConfiguration.swift

  extern "Swift" {
    type VZUSBMassStorageDeviceConfiguration;

    fn vz_usb_mass_storage_device_configuration_init(attachment: *const c_void) -> VZUSBMassStorageDeviceConfiguration;
    fn bridge_retained(self: &VZUSBMassStorageDeviceConfiguration) -> VZUSBMassStorageDeviceConfiguration;
    fn bridge_borrowed(self: &VZUSBMassStorageDeviceConfiguration) -> *const c_void;
    fn vz_usb_mass_storage_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZUSBMassStorageDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZUSBMassStorageDeviceConfiguration) -> String;
  }

  // Storage/VirtioBlockDeviceConfiguration.swift

  extern "Swift" {
    type VZVirtioBlockDeviceConfiguration;

    fn vz_virtio_block_device_configuration_init(attachment: *const c_void) -> VZVirtioBlockDeviceConfiguration;
    fn bridge_retained(self: &VZVirtioBlockDeviceConfiguration) -> VZVirtioBlockDeviceConfiguration;
    fn bridge_borrowed(self: &VZVirtioBlockDeviceConfiguration) -> *const c_void;
    fn vz_virtio_block_device_configuration_bridge_cast(
      object: *const c_void,
    ) -> Result<VZVirtioBlockDeviceConfiguration, BridgedNil>;
    fn ns_description(self: &VZVirtioBlockDeviceConfiguration) -> String;
    fn vz_virtio_block_device_configuration_validate_block_device_identifier(
      block_device_identifier: &str,
    ) -> Result<(), BridgedThrown>;
    fn vz_block_device_identifier(self: &VZVirtioBlockDeviceConfiguration) -> String;
    fn vz_set_block_device_identifier(self: &VZVirtioBlockDeviceConfiguration, block_device_identifier: &str);
  }
}

/// An object as its abstract class, for as long as `owner`, which holds it.
///
/// # Safety
///
/// `pointer` is what `bridge_borrowed()` returned for an object `owner` holds.
unsafe fn bridge_object<O>(_owner: &O, pointer: *const c_void) -> &NSObject {
  // SAFETY: the caller's: `owner` keeps the object alive while it is
  // borrowed, and every Virtualization class is an `NSObject`.
  unsafe { &*pointer.cast::<NSObject>() }
}

/// The objects of an array of an abstract class, which Swift handed over.
///
/// # Safety
///
/// `pointer` is what `bridging(retained:)` returned for an `NSArray` of
/// Virtualization objects, taken once.
unsafe fn bridge_objects(pointer: *mut c_void) -> Vec<Retained<NSObject>> {
  // SAFETY: the caller's.
  ns_array_objects(&*unsafe { retained::<NSArray<NSObject>>(pointer) })
}

// Foundation.swift

/// What the last throwing call on this thread threw. Call it on that thread,
/// only after the call returned [`BridgedThrown`].
pub fn bridge_take_thrown_error() -> Retained<NSError> {
  // SAFETY: `bridge_take_thrown_error` returns a retained `NSError`.
  unsafe { retained(ffi::bridge_take_thrown_error()) }
}

// Security.swift

/// [`SecCertificate`](https://developer.apple.com/documentation/security/seccertificate),
/// which Virtualization's Secure Boot members take and return. An objc2
/// object, not an opaque Swift type: an array holds one beside an `NSData`.
pub struct SecCertificate {
  object: Retained<AnyObject>,
}

// SAFETY: a certificate never changes, and Apple's `VZEFISignature`, which
// carries one, is `Sendable`.
unsafe impl Send for SecCertificate {}

// SAFETY: as for `Send`.
unsafe impl Sync for SecCertificate {}

/// [`SecCertificateCreateWithData(_:_:)`](https://developer.apple.com/documentation/security/seccertificatecreatewithdata(_:_:)),
/// with the default allocator. `Data` is its bytes.
pub fn sec_certificate_create_with_data(data: &[u8]) -> Option<SecCertificate> {
  // SAFETY: a retained `SecCertificate`, or `nil`.
  unsafe { retained_or_nil(ffi::sec_certificate_create_with_data(borrowed(&*ns_data(data)))) }
    .map(|object| SecCertificate { object })
}

impl SecCertificate {
  pub fn bridge_retained(&self) -> Self {
    Self {
      object: self.object.clone(),
    }
  }

  pub fn ns_description(&self) -> String {
    ns_description(&self.object)
  }

  /// [`SecCertificateCopyData(_:)`](https://developer.apple.com/documentation/security/seccertificatecopydata(_:)).
  /// `Data` is its bytes.
  pub fn sec_certificate_copy_data(&self) -> Vec<u8> {
    // SAFETY: a retained `NSData`.
    let data: Retained<NSData> = unsafe { retained(ffi::sec_certificate_copy_data(borrowed(&*self.object))) };

    ns_data_bytes(&data)
  }
}

// VirtualMachine.swift

pub fn vz_virtual_machine_init(configuration: &VZVirtualMachineConfiguration) -> VZVirtualMachine {
  ffi::vz_virtual_machine_init(configuration.bridge_retained())
}

pub fn vz_virtual_machine_init_queue(
  configuration: &VZVirtualMachineConfiguration,
  queue: &DispatchQueue,
) -> VZVirtualMachine {
  ffi::vz_virtual_machine_init_queue(configuration.bridge_retained(), ptr::from_ref(queue).cast())
}

// SAFETY: Virtualization takes every call on a machine on its queue, and
// dispatch runs that queue on any thread, so a machine must move between them.
// Calling it off that queue is the caller's to avoid, as in Swift.
unsafe impl Send for VZVirtualMachine {}

// SAFETY: as for `Send`: the machine's queue, not Rust, serializes its calls.
unsafe impl Sync for VZVirtualMachine {}

/// A completion handler as Swift calls it: with what failed by address, `0`
/// for success.
fn bridge_completion_handler(
  completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
) -> Box<dyn FnOnce(usize)> {
  Box::new(move |error| {
    // SAFETY: `bridging(retainedError:)`, a retained `NSError` or `nil`.
    completion_handler(unsafe { retained_or_nil(error) })
  })
}

impl VZVirtualMachine {
  pub fn vz_queue(&self) -> DispatchRetained<DispatchQueue> {
    let queue = ffi::vz_virtual_machine_queue(self.bridge_retained());

    // SAFETY: `queue`, a retained `dispatch_queue_t`, never `nil`.
    unsafe { DispatchRetained::from_raw(NonNull::new_unchecked(queue.cast())) }
  }

  /// The delegate set with [`Self::vz_set_delegate`].
  pub fn vz_delegate(&self) -> Option<Arc<dyn VZVirtualMachineDelegate>> {
    let address = ffi::vz_virtual_machine_delegate(self.bridge_retained());

    // SAFETY: `VZVirtualMachineDelegateBridge::bridge_retained`'s address,
    // taken once, or `0` for none.
    (address != 0).then(|| *unsafe { Box::from_raw(address as *mut Arc<dyn VZVirtualMachineDelegate>) })
  }

  pub fn vz_set_delegate(&self, delegate: Option<Arc<dyn VZVirtualMachineDelegate>>) {
    match delegate {
      Some(delegate) => {
        ffi::vz_virtual_machine_set_delegate(self.bridge_retained(), VZVirtualMachineDelegateBridge { delegate });
      }
      None => self.bridge_set_delegate_nil(),
    }
  }

  /// Each any `VZConsoleDevice`.
  pub fn vz_console_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `consoleDevices`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_console_devices(self.bridge_retained())) }
  }

  /// Each any `VZDirectorySharingDevice`.
  pub fn vz_directory_sharing_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `directorySharingDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_directory_sharing_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZGraphicsDevice`.
  pub fn vz_graphics_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `graphicsDevices`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_graphics_devices(self.bridge_retained())) }
  }

  /// Each any `VZMemoryBalloonDevice`.
  pub fn vz_memory_balloon_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `memoryBalloonDevices`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_memory_balloon_devices(self.bridge_retained())) }
  }

  pub fn vz_network_devices(&self) -> Vec<VZNetworkDevice> {
    // SAFETY: `networkDevices`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_network_devices(self.bridge_retained())) }
      .iter()
      .map(|device| ffi::vz_network_device_bridge_from(borrowed(&**device)))
      .collect()
  }

  /// Each any `VZSocketDevice`.
  pub fn vz_socket_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `socketDevices`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_socket_devices(self.bridge_retained())) }
  }

  /// Each any `VZUSBController`.
  pub fn vz_usb_controllers(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `usbControllers`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_usb_controllers(self.bridge_retained())) }
  }

  pub fn vz_start(&self, completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    ffi::vz_virtual_machine_start(self.bridge_retained(), bridge_completion_handler(completion_handler));
  }

  /// `options` is any `VZVirtualMachineStartOptions`.
  pub fn vz_start_options(
    &self,
    options: &NSObject,
    completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    ffi::vz_virtual_machine_start_options(
      self.bridge_retained(),
      borrowed(options),
      bridge_completion_handler(completion_handler),
    );
  }

  pub fn vz_stop(&self, completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    ffi::vz_virtual_machine_stop(self.bridge_retained(), bridge_completion_handler(completion_handler));
  }

  pub fn vz_pause(&self, completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    ffi::vz_virtual_machine_pause(self.bridge_retained(), bridge_completion_handler(completion_handler));
  }

  pub fn vz_resume(&self, completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static) {
    ffi::vz_virtual_machine_resume(self.bridge_retained(), bridge_completion_handler(completion_handler));
  }

  pub fn vz_restore_machine_state_from(
    &self,
    url: &NSURL,
    completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    ffi::vz_virtual_machine_restore_machine_state_from(
      self.bridge_retained(),
      borrowed(url),
      bridge_completion_handler(completion_handler),
    );
  }

  pub fn vz_save_machine_state_to(
    &self,
    url: &NSURL,
    completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
  ) {
    ffi::vz_virtual_machine_save_machine_state_to(
      self.bridge_retained(),
      borrowed(url),
      bridge_completion_handler(completion_handler),
    );
  }
}

/// `VZVirtualMachineDelegate`, which Rust implements. Virtualization calls it
/// on the machine's queue.
pub trait VZVirtualMachineDelegate: Any + Send + Sync {
  fn guest_did_stop(&self, virtual_machine: VZVirtualMachine);

  fn virtual_machine_did_stop_with_error(&self, virtual_machine: VZVirtualMachine, error: Retained<NSError>);

  fn virtual_machine_network_device_attachment_was_disconnected_with_error(
    &self,
    virtual_machine: VZVirtualMachine,
    network_device: VZNetworkDevice,
    error: Retained<NSError>,
  );
}

/// A [`VZVirtualMachineDelegate`] as Swift holds it.
pub struct VZVirtualMachineDelegateBridge {
  delegate: Arc<dyn VZVirtualMachineDelegate>,
}

impl VZVirtualMachineDelegateBridge {
  /// Another reference to the delegate, for [`VZVirtualMachine::vz_delegate`]
  /// to take, by address. (swift-bridge's code for returning this type by
  /// value fails clippy.)
  fn bridge_retained(&self) -> usize {
    Box::into_raw(Box::new(Arc::clone(&self.delegate))) as usize
  }

  fn guest_did_stop(&self, virtual_machine: VZVirtualMachine) {
    self.delegate.guest_did_stop(virtual_machine);
  }

  fn virtual_machine_did_stop_with_error(&self, virtual_machine: VZVirtualMachine, error: *mut c_void) {
    // SAFETY: `error`, a retained `NSError`.
    let error = unsafe { retained(error) };

    self
      .delegate
      .virtual_machine_did_stop_with_error(virtual_machine, error);
  }

  fn virtual_machine_network_device_attachment_was_disconnected_with_error(
    &self,
    virtual_machine: VZVirtualMachine,
    network_device: VZNetworkDevice,
    error: *mut c_void,
  ) {
    // SAFETY: `error`, a retained `NSError`.
    let error = unsafe { retained(error) };

    self
      .delegate
      .virtual_machine_network_device_attachment_was_disconnected_with_error(virtual_machine, network_device, error);
  }
}

// VirtualMachineConfiguration.swift

impl VZVirtualMachineConfiguration {
  /// Any `VZBootLoader`.
  pub fn vz_boot_loader(&self) -> Option<Retained<NSObject>> {
    // SAFETY: `bootLoader`, a retained `VZBootLoader` or `nil`.
    unsafe {
      retained_or_nil(ffi::vz_virtual_machine_configuration_boot_loader(
        self.bridge_retained(),
      ))
    }
  }

  /// `boot_loader` is any `VZBootLoader`.
  pub fn vz_set_boot_loader(&self, boot_loader: Option<&NSObject>) {
    ffi::vz_virtual_machine_configuration_set_boot_loader(self.bridge_retained(), borrowed_or_nil(boot_loader));
  }

  /// Any `VZPlatformConfiguration`.
  pub fn vz_platform(&self) -> Retained<NSObject> {
    // SAFETY: `platform`, a retained `VZPlatformConfiguration`.
    unsafe { retained(ffi::vz_virtual_machine_configuration_platform(self.bridge_retained())) }
  }

  /// `platform` is any `VZPlatformConfiguration`.
  pub fn vz_set_platform(&self, platform: &NSObject) {
    ffi::vz_virtual_machine_configuration_set_platform(self.bridge_retained(), borrowed(platform));
  }

  /// Each any `VZAudioDeviceConfiguration`.
  pub fn vz_audio_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `audioDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_audio_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZAudioDeviceConfiguration`.
  pub fn vz_set_audio_devices(&self, audio_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_audio_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(audio_devices)),
    );
  }

  /// Each any `VZConsoleDeviceConfiguration`.
  pub fn vz_console_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `consoleDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_console_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZConsoleDeviceConfiguration`.
  pub fn vz_set_console_devices(&self, console_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_console_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(console_devices)),
    );
  }

  /// Each any `VZDirectorySharingDeviceConfiguration`.
  pub fn vz_directory_sharing_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `directorySharingDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_directory_sharing_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZDirectorySharingDeviceConfiguration`.
  pub fn vz_set_directory_sharing_devices(&self, directory_sharing_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_directory_sharing_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(directory_sharing_devices)),
    );
  }

  /// Each any `VZEntropyDeviceConfiguration`.
  pub fn vz_entropy_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `entropyDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_entropy_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZEntropyDeviceConfiguration`.
  pub fn vz_set_entropy_devices(&self, entropy_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_entropy_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(entropy_devices)),
    );
  }

  /// Each any `VZMemoryBalloonDeviceConfiguration`.
  pub fn vz_memory_balloon_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `memoryBalloonDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_memory_balloon_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZMemoryBalloonDeviceConfiguration`.
  pub fn vz_set_memory_balloon_devices(&self, memory_balloon_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_memory_balloon_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(memory_balloon_devices)),
    );
  }

  /// Each any `VZNetworkDeviceConfiguration`.
  pub fn vz_network_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `networkDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_network_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZNetworkDeviceConfiguration`.
  pub fn vz_set_network_devices(&self, network_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_network_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(network_devices)),
    );
  }

  /// Each any `VZSerialPortConfiguration`.
  pub fn vz_serial_ports(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `serialPorts`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_serial_ports(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZSerialPortConfiguration`.
  pub fn vz_set_serial_ports(&self, serial_ports: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_serial_ports(self.bridge_retained(), borrowed(&*ns_array(serial_ports)));
  }

  /// Each any `VZSocketDeviceConfiguration`.
  pub fn vz_socket_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `socketDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_socket_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZSocketDeviceConfiguration`.
  pub fn vz_set_socket_devices(&self, socket_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_socket_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(socket_devices)),
    );
  }

  /// Each any `VZStorageDeviceConfiguration`.
  pub fn vz_storage_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `storageDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_storage_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZStorageDeviceConfiguration`.
  pub fn vz_set_storage_devices(&self, storage_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_storage_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(storage_devices)),
    );
  }

  /// Each any `VZKeyboardConfiguration`.
  pub fn vz_keyboards(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `keyboards`, a retained `NSArray`.
    unsafe { bridge_objects(ffi::vz_virtual_machine_configuration_keyboards(self.bridge_retained())) }
  }

  /// Each any `VZKeyboardConfiguration`.
  pub fn vz_set_keyboards(&self, keyboards: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_keyboards(self.bridge_retained(), borrowed(&*ns_array(keyboards)));
  }

  /// Each any `VZPointingDeviceConfiguration`.
  pub fn vz_pointing_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `pointingDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_pointing_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZPointingDeviceConfiguration`.
  pub fn vz_set_pointing_devices(&self, pointing_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_pointing_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(pointing_devices)),
    );
  }

  /// Each any `VZGraphicsDeviceConfiguration`.
  pub fn vz_graphics_devices(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `graphicsDevices`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_graphics_devices(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZGraphicsDeviceConfiguration`.
  pub fn vz_set_graphics_devices(&self, graphics_devices: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_graphics_devices(
      self.bridge_retained(),
      borrowed(&*ns_array(graphics_devices)),
    );
  }

  /// Each any `VZUSBControllerConfiguration`.
  pub fn vz_usb_controllers(&self) -> Vec<Retained<NSObject>> {
    // SAFETY: `usbControllers`, a retained `NSArray`.
    unsafe {
      bridge_objects(ffi::vz_virtual_machine_configuration_usb_controllers(
        self.bridge_retained(),
      ))
    }
  }

  /// Each any `VZUSBControllerConfiguration`.
  pub fn vz_set_usb_controllers(&self, usb_controllers: &[&NSObject]) {
    ffi::vz_virtual_machine_configuration_set_usb_controllers(
      self.bridge_retained(),
      borrowed(&*ns_array(usb_controllers)),
    );
  }
}

// BootLoader/LinuxBootLoader.swift

pub fn vz_linux_boot_loader_init(kernel_url: &NSURL) -> VZLinuxBootLoader {
  ffi::vz_linux_boot_loader_init(borrowed(kernel_url))
}

pub fn vz_linux_boot_loader_bridge_cast(object: &NSObject) -> Option<VZLinuxBootLoader> {
  ffi::vz_linux_boot_loader_bridge_cast(borrowed(object)).ok()
}

impl VZLinuxBootLoader {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_kernel_url(&self) -> Retained<NSURL> {
    // SAFETY: `kernelURL`, a retained `NSURL`.
    unsafe { retained(ffi::vz_linux_boot_loader_kernel_url(self.bridge_retained())) }
  }

  pub fn vz_set_kernel_url(&self, kernel_url: &NSURL) {
    ffi::vz_linux_boot_loader_set_kernel_url(self.bridge_retained(), borrowed(kernel_url));
  }

  pub fn vz_initial_ramdisk_url(&self) -> Option<Retained<NSURL>> {
    // SAFETY: `initialRamdiskURL`, a retained `NSURL` or `nil`.
    unsafe { retained_or_nil(ffi::vz_linux_boot_loader_initial_ramdisk_url(self.bridge_retained())) }
  }

  pub fn vz_set_initial_ramdisk_url(&self, initial_ramdisk_url: Option<&NSURL>) {
    ffi::vz_linux_boot_loader_set_initial_ramdisk_url(self.bridge_retained(), borrowed_or_nil(initial_ramdisk_url));
  }
}

// BootLoader/EFIBootLoader.swift

pub fn vz_efi_boot_loader_bridge_cast(object: &NSObject) -> Option<VZEFIBootLoader> {
  ffi::vz_efi_boot_loader_bridge_cast(borrowed(object)).ok()
}

impl VZEFIBootLoader {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// BootLoader/EFIVariableStore.swift

pub fn vz_efi_variable_store_init(url: &NSURL) -> VZEFIVariableStore {
  ffi::vz_efi_variable_store_init(borrowed(url))
}

/// `options` is a `VZEFIVariableStore.InitializationOptions`' raw value.
pub fn vz_efi_variable_store_init_creating_variable_store_at_options(
  url: &NSURL,
  options: usize,
) -> Result<VZEFIVariableStore, BridgedThrown> {
  ffi::vz_efi_variable_store_init_creating_variable_store_at_options(borrowed(url), options)
}

impl VZEFIVariableStore {
  pub fn vz_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe { retained(ffi::vz_efi_variable_store_url(self.bridge_retained())) }
  }

  pub fn vz_enable_secure_boot(&self, platform_key: &SecCertificate) -> Result<(), BridgedThrown> {
    ffi::vz_efi_variable_store_enable_secure_boot(self.bridge_retained(), borrowed(&*platform_key.object))
  }
}

// BootLoader/EFISignatureList.swift

/// [`VZEFISignature`](https://developer.apple.com/documentation/virtualization/vzefisignature).
/// `Data` is its bytes. `Unknown` is a case newer than this crate; passing it
/// to Virtualization stops the process.
pub enum VZEFISignature {
  Sha256(Vec<u8>),
  X509Certificate(SecCertificate),
  Unknown,
}

impl VZEFISignature {
  /// What crosses for it: its hash as `NSData`, its certificate, or `NSNull`
  /// for `Unknown`.
  fn bridge_object(&self) -> Retained<AnyObject> {
    match self {
      Self::Sha256(data) => Retained::into_super(Retained::into_super(ns_data(data))),
      Self::X509Certificate(certificate) => certificate.object.clone(),
      Self::Unknown => Retained::into_super(Retained::into_super(NSNull::null())),
    }
  }

  fn bridge_from(object: Retained<AnyObject>) -> Self {
    let object = match object.downcast::<NSData>() {
      Ok(data) => return Self::Sha256(ns_data_bytes(&data)),
      Err(object) => object,
    };

    match object.downcast::<NSNull>() {
      Ok(_) => Self::Unknown,
      Err(object) => Self::X509Certificate(SecCertificate { object }),
    }
  }
}

// SAFETY: Apple declares `VZEFISignatureList` `Sendable`.
unsafe impl Send for VZEFISignatureList {}

// SAFETY: as for `Send`.
unsafe impl Sync for VZEFISignatureList {}

pub fn vz_efi_signature_list_init_contents_of(url: &NSURL) -> Result<VZEFISignatureList, BridgedThrown> {
  ffi::vz_efi_signature_list_init_contents_of(borrowed(url))
}

pub fn vz_efi_signature_list_init_signatures(signatures: &[VZEFISignature]) -> VZEFISignatureList {
  let objects: Vec<_> = signatures
    .iter()
    .map(VZEFISignature::bridge_object)
    .collect();
  let objects: Vec<_> = objects.iter().map(|object| &**object).collect();

  ffi::vz_efi_signature_list_init_signatures(borrowed(&*ns_array(&objects)))
}

impl VZEFISignatureList {
  fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_signatures(&self) -> Vec<VZEFISignature> {
    // SAFETY: `signatures`, a retained `NSArray` of what each crosses as.
    let signatures: Retained<NSArray<AnyObject>> =
      unsafe { retained(ffi::vz_efi_signature_list_signatures(self.bridge_retained())) };

    ns_array_objects(&signatures)
      .into_iter()
      .map(VZEFISignature::bridge_from)
      .collect()
  }
}

// BootLoader/EFISignatureDatabaseConfiguration.swift

/// An `NSArray` of `signature_lists`.
fn signature_lists(signature_lists: &[&VZEFISignatureList]) -> Retained<NSArray<NSObject>> {
  let objects: Vec<_> = signature_lists
    .iter()
    .map(|signature_list| signature_list.bridge_object())
    .collect();

  ns_array(&objects)
}

/// The signature lists of an array Swift handed over.
///
/// # Safety
///
/// `pointer` is what `bridging(retained:)` returned for an `NSArray` of
/// `VZEFISignatureList`s, taken once.
unsafe fn bridge_signature_lists(pointer: *mut c_void) -> Vec<VZEFISignatureList> {
  // SAFETY: the caller's.
  unsafe { bridge_objects(pointer) }
    .iter()
    .map(|signature_list| ffi::vz_efi_signature_list_bridge_from(borrowed(&**signature_list)))
    .collect()
}

// SAFETY: Apple declares `VZEFISignatureDatabaseConfiguration` `Sendable`.
unsafe impl Send for VZEFISignatureDatabaseConfiguration {}

// SAFETY: as for `Send`.
unsafe impl Sync for VZEFISignatureDatabaseConfiguration {}

pub fn vz_efi_signature_database_configuration_init(
  key_exchange_keys: &[&VZEFISignatureList],
  db_signatures: &[&VZEFISignatureList],
  dbx_signatures: &[&VZEFISignatureList],
) -> VZEFISignatureDatabaseConfiguration {
  ffi::vz_efi_signature_database_configuration_init(
    borrowed(&*signature_lists(key_exchange_keys)),
    borrowed(&*signature_lists(db_signatures)),
    borrowed(&*signature_lists(dbx_signatures)),
  )
}

impl VZEFISignatureDatabaseConfiguration {
  pub fn vz_key_exchange_keys(&self) -> Vec<VZEFISignatureList> {
    // SAFETY: `keyExchangeKeys`, a retained `NSArray` of `VZEFISignatureList`s.
    unsafe {
      bridge_signature_lists(ffi::vz_efi_signature_database_configuration_key_exchange_keys(
        self.bridge_retained(),
      ))
    }
  }

  pub fn vz_db_signatures(&self) -> Vec<VZEFISignatureList> {
    // SAFETY: `dbSignatures`, a retained `NSArray` of `VZEFISignatureList`s.
    unsafe {
      bridge_signature_lists(ffi::vz_efi_signature_database_configuration_db_signatures(
        self.bridge_retained(),
      ))
    }
  }

  pub fn vz_dbx_signatures(&self) -> Vec<VZEFISignatureList> {
    // SAFETY: `dbxSignatures`, a retained `NSArray` of `VZEFISignatureList`s.
    unsafe {
      bridge_signature_lists(ffi::vz_efi_signature_database_configuration_dbx_signatures(
        self.bridge_retained(),
      ))
    }
  }
}

// BootLoader/GenericPlatformConfiguration.swift

pub fn vz_generic_platform_configuration_bridge_cast(object: &NSObject) -> Option<VZGenericPlatformConfiguration> {
  ffi::vz_generic_platform_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZGenericPlatformConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// BootLoader/GenericMachineIdentifier.swift

/// `Data` is its bytes.
pub fn vz_generic_machine_identifier_init_data_representation(
  data_representation: &[u8],
) -> Option<VZGenericMachineIdentifier> {
  ffi::vz_generic_machine_identifier_init_data_representation(borrowed(&*ns_data(data_representation))).ok()
}

impl VZGenericMachineIdentifier {
  /// `Data` is its bytes.
  pub fn vz_data_representation(&self) -> Vec<u8> {
    // SAFETY: `dataRepresentation`, a retained `NSData`.
    let data: Retained<NSData> = unsafe {
      retained(ffi::vz_generic_machine_identifier_data_representation(
        self.bridge_retained(),
      ))
    };

    ns_data_bytes(&data)
  }
}

// DirectorySharing/MultipleDirectoryShare.swift

/// `[String: VZSharedDirectory]` is each name with its directory.
pub fn vz_multiple_directory_share_init_directories(
  directories: &[(&str, &VZSharedDirectory)],
) -> VZMultipleDirectoryShare {
  let entries: Vec<_> = directories
    .iter()
    .map(|(name, directory)| (*name, directory.bridge_object()))
    .collect();

  ffi::vz_multiple_directory_share_init_directories(borrowed(&*ns_dictionary(&entries)))
}

pub fn vz_multiple_directory_share_bridge_cast(object: &NSObject) -> Option<VZMultipleDirectoryShare> {
  ffi::vz_multiple_directory_share_bridge_cast(borrowed(object)).ok()
}

impl VZMultipleDirectoryShare {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// `[String: VZSharedDirectory]` is each name with its directory, in no
  /// particular order.
  pub fn vz_directories(&self) -> Vec<(String, VZSharedDirectory)> {
    // SAFETY: `directories`, a retained `NSDictionary` of `NSString`s to
    // `VZSharedDirectory`s.
    let directories: Retained<NSDictionary<NSString, NSObject>> =
      unsafe { retained(ffi::vz_multiple_directory_share_directories(self.bridge_retained())) };

    ns_dictionary_entries(&directories)
      .into_iter()
      .map(|(name, directory)| (name, ffi::vz_shared_directory_bridge_from(borrowed(&*directory))))
      .collect()
  }
}

// DirectorySharing/SharedDirectory.swift

pub fn vz_shared_directory_init(url: &NSURL, read_only: bool) -> VZSharedDirectory {
  ffi::vz_shared_directory_init(borrowed(url), read_only)
}

impl VZSharedDirectory {
  fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe { retained(ffi::vz_shared_directory_url(self.bridge_retained())) }
  }
}

// DirectorySharing/SingleDirectoryShare.swift

pub fn vz_single_directory_share_init(directory: &VZSharedDirectory) -> VZSingleDirectoryShare {
  ffi::vz_single_directory_share_init(directory.bridge_retained())
}

pub fn vz_single_directory_share_bridge_cast(object: &NSObject) -> Option<VZSingleDirectoryShare> {
  ffi::vz_single_directory_share_bridge_cast(borrowed(object)).ok()
}

impl VZSingleDirectoryShare {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// DirectorySharing/VirtioFileSystemDevice.swift

pub fn vz_virtio_file_system_device_bridge_cast(object: &NSObject) -> Option<VZVirtioFileSystemDevice> {
  ffi::vz_virtio_file_system_device_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioFileSystemDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// Any `VZDirectoryShare`.
  pub fn vz_share(&self) -> Option<Retained<NSObject>> {
    // SAFETY: `share`, a retained `VZDirectoryShare` or `nil`.
    unsafe { retained_or_nil(ffi::vz_virtio_file_system_device_share(self.bridge_retained())) }
  }

  /// `share` is any `VZDirectoryShare`.
  pub fn vz_set_share(&self, share: Option<&NSObject>) {
    ffi::vz_virtio_file_system_device_set_share(self.bridge_retained(), borrowed_or_nil(share));
  }
}

// DirectorySharing/VirtioFileSystemDeviceConfiguration.swift

pub fn vz_virtio_file_system_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioFileSystemDeviceConfiguration> {
  ffi::vz_virtio_file_system_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioFileSystemDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// Any `VZDirectoryShare`.
  pub fn vz_share(&self) -> Option<Retained<NSObject>> {
    // SAFETY: `share`, a retained `VZDirectoryShare` or `nil`.
    unsafe {
      retained_or_nil(ffi::vz_virtio_file_system_device_configuration_share(
        self.bridge_retained(),
      ))
    }
  }

  /// `share` is any `VZDirectoryShare`.
  pub fn vz_set_share(&self, share: Option<&NSObject>) {
    ffi::vz_virtio_file_system_device_configuration_set_share(self.bridge_retained(), borrowed_or_nil(share));
  }
}

// Network/BridgedNetworkDeviceAttachment.swift

pub fn vz_bridged_network_device_attachment_init(
  interface: &VZBridgedNetworkInterface,
) -> VZBridgedNetworkDeviceAttachment {
  ffi::vz_bridged_network_device_attachment_init(interface.bridge_retained())
}

pub fn vz_bridged_network_device_attachment_bridge_cast(object: &NSObject) -> Option<VZBridgedNetworkDeviceAttachment> {
  ffi::vz_bridged_network_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZBridgedNetworkDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Network/BridgedNetworkInterface.swift

pub fn vz_bridged_network_interface_network_interfaces() -> Vec<VZBridgedNetworkInterface> {
  // SAFETY: `networkInterfaces`, a retained `NSArray`.
  unsafe { bridge_objects(ffi::vz_bridged_network_interface_network_interfaces()) }
    .iter()
    .map(|interface| ffi::vz_bridged_network_interface_bridge_from(borrowed(&**interface)))
    .collect()
}

// Network/FileHandleNetworkDeviceAttachment.swift

/// The descriptor becomes a `FileHandle` that owns it.
pub fn vz_file_handle_network_device_attachment_init(file_handle: OwnedFd) -> VZFileHandleNetworkDeviceAttachment {
  ffi::vz_file_handle_network_device_attachment_init(borrowed(&*ns_file_handle_init_file_descriptor_close_on_dealloc(
    file_handle,
  )))
}

pub fn vz_file_handle_network_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZFileHandleNetworkDeviceAttachment> {
  ffi::vz_file_handle_network_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZFileHandleNetworkDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// `fileHandle`'s descriptor, open as long as the attachment, which keeps
  /// the handle.
  pub fn vz_file_handle(&self) -> BorrowedFd<'_> {
    // SAFETY: `fileHandle`, a retained `FileHandle`.
    let handle: Retained<NSFileHandle> = unsafe {
      retained(ffi::vz_file_handle_network_device_attachment_file_handle(
        self.bridge_retained(),
      ))
    };

    // SAFETY: the attachment's handle is fixed, and a handle's descriptor
    // stays open while it lives.
    unsafe { BorrowedFd::borrow_raw(ns_file_handle_file_descriptor(&handle)) }
  }
}

// Network/MACAddress.swift

/// `ether_addr_t` is its six octets.
pub fn vz_mac_address_init_ethernet_address(ethernet_address: [u8; 6]) -> VZMACAddress {
  let mut packed = [0; 8];

  packed[..6].copy_from_slice(&ethernet_address);
  ffi::vz_mac_address_init_ethernet_address(u64::from_le_bytes(packed))
}

pub fn vz_mac_address_init_string(string: &str) -> Option<VZMACAddress> {
  ffi::vz_mac_address_init_string(string).ok()
}

impl VZMACAddress {
  /// `ether_addr_t` is its six octets.
  pub fn vz_ethernet_address(&self) -> [u8; 6] {
    let packed = self.bridge_ethernet_address().to_le_bytes();

    [packed[0], packed[1], packed[2], packed[3], packed[4], packed[5]]
  }
}

// Network/NATNetworkDeviceAttachment.swift

pub fn vz_nat_network_device_attachment_bridge_cast(object: &NSObject) -> Option<VZNATNetworkDeviceAttachment> {
  ffi::vz_nat_network_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZNATNetworkDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Network/NetworkDevice.swift

impl VZNetworkDevice {
  /// Any `VZNetworkDeviceAttachment`.
  pub fn vz_attachment(&self) -> Option<Retained<NSObject>> {
    // SAFETY: `attachment`, a retained `VZNetworkDeviceAttachment` or `nil`.
    unsafe { retained_or_nil(ffi::vz_network_device_attachment(self.bridge_retained())) }
  }

  /// `attachment` is any `VZNetworkDeviceAttachment`.
  pub fn vz_set_attachment(&self, attachment: Option<&NSObject>) {
    ffi::vz_network_device_set_attachment(self.bridge_retained(), borrowed_or_nil(attachment));
  }
}

// Network/NetworkDeviceConfiguration.swift

/// `this` is any `VZNetworkDeviceConfiguration`.
pub fn vz_network_device_configuration_mac_address(this: &NSObject) -> VZMACAddress {
  ffi::vz_network_device_configuration_mac_address(borrowed(this))
}

/// `this` is any `VZNetworkDeviceConfiguration`.
pub fn vz_network_device_configuration_set_mac_address(this: &NSObject, mac_address: &VZMACAddress) {
  ffi::vz_network_device_configuration_set_mac_address(borrowed(this), mac_address.bridge_retained());
}

/// `this` is any `VZNetworkDeviceConfiguration`; the attachment, any
/// `VZNetworkDeviceAttachment`.
pub fn vz_network_device_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  // SAFETY: `attachment`, a retained `VZNetworkDeviceAttachment` or `nil`.
  unsafe { retained_or_nil(ffi::vz_network_device_configuration_attachment(borrowed(this))) }
}

/// `this` is any `VZNetworkDeviceConfiguration`; `attachment`, any
/// `VZNetworkDeviceAttachment`.
pub fn vz_network_device_configuration_set_attachment(this: &NSObject, attachment: Option<&NSObject>) {
  ffi::vz_network_device_configuration_set_attachment(borrowed(this), borrowed_or_nil(attachment));
}

// Network/VirtioNetworkDeviceConfiguration.swift

pub fn vz_virtio_network_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioNetworkDeviceConfiguration> {
  ffi::vz_virtio_network_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioNetworkDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Network/VmnetNetworkDeviceAttachment.swift

/// `network` crosses from vmnet-sys's bridge, lent for the call.
pub fn vz_vmnet_network_device_attachment_init(network: &vmnet_sys::VmnetNetwork) -> VZVmnetNetworkDeviceAttachment {
  ffi::vz_vmnet_network_device_attachment_init(network.bridge_borrowed())
}

pub fn vz_vmnet_network_device_attachment_bridge_cast(object: &NSObject) -> Option<VZVmnetNetworkDeviceAttachment> {
  ffi::vz_vmnet_network_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZVmnetNetworkDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// `network`, crossing back to vmnet-sys's bridge.
  pub fn vz_network(&self) -> vmnet_sys::VmnetNetwork {
    // SAFETY: the attachment's `network`, which `self` holds through the call.
    unsafe {
      vmnet_sys::vmnet_network_bridge_from(ffi::vz_vmnet_network_device_attachment_network(self.bridge_retained()))
    }
  }
}

// OtherDevices/USBController.swift

/// `this` is any `VZUSBController`; `device`, any `VZUSBDevice`.
pub fn vz_usb_controller_attach(
  this: &NSObject,
  device: &NSObject,
  completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
) {
  ffi::vz_usb_controller_attach(
    borrowed(this),
    borrowed(device),
    bridge_completion_handler(completion_handler),
  );
}

/// `this` is any `VZUSBController`; `device`, any `VZUSBDevice`.
pub fn vz_usb_controller_detach(
  this: &NSObject,
  device: &NSObject,
  completion_handler: impl FnOnce(Option<Retained<NSError>>) + Send + 'static,
) {
  ffi::vz_usb_controller_detach(
    borrowed(this),
    borrowed(device),
    bridge_completion_handler(completion_handler),
  );
}

/// `this` is any `VZUSBController`; each device, any `VZUSBDevice`.
pub fn vz_usb_controller_usb_devices(this: &NSObject) -> Vec<Retained<NSObject>> {
  // SAFETY: `usbDevices`, a retained `NSArray`.
  unsafe { bridge_objects(ffi::vz_usb_controller_usb_devices(borrowed(this))) }
}

/// `this` is any `VZUSBController`. The delegate set with
/// [`vz_usb_controller_set_delegate`].
pub fn vz_usb_controller_delegate(this: &NSObject) -> Option<Arc<dyn VZUSBControllerDelegate>> {
  let address = ffi::vz_usb_controller_delegate(borrowed(this));

  // SAFETY: `VZUSBControllerDelegateBridge::bridge_retained`'s address, taken
  // once, or `0` for none.
  (address != 0).then(|| *unsafe { Box::from_raw(address as *mut Arc<dyn VZUSBControllerDelegate>) })
}

/// `this` is any `VZUSBController`.
pub fn vz_usb_controller_set_delegate(this: &NSObject, delegate: Option<Arc<dyn VZUSBControllerDelegate>>) {
  match delegate {
    Some(delegate) => ffi::vz_usb_controller_set_delegate(borrowed(this), VZUSBControllerDelegateBridge { delegate }),
    None => ffi::vz_usb_controller_set_delegate_nil(borrowed(this)),
  }
}

/// `VZUSBController.Delegate`, which Rust implements. `usb_controller` is
/// any `VZUSBController`.
pub trait VZUSBControllerDelegate: Any + Send + Sync {
  fn usb_controller_usb_passthrough_device_did_disconnect(
    &self,
    usb_controller: Retained<NSObject>,
    device: VZUSBPassthroughDevice,
  );
}

/// A [`VZUSBControllerDelegate`] as Swift holds it.
pub struct VZUSBControllerDelegateBridge {
  delegate: Arc<dyn VZUSBControllerDelegate>,
}

impl VZUSBControllerDelegateBridge {
  /// Another reference to the delegate, for [`vz_usb_controller_delegate`]
  /// to take, by address. (swift-bridge's code for returning this type by
  /// value fails clippy.)
  fn bridge_retained(&self) -> usize {
    Box::into_raw(Box::new(Arc::clone(&self.delegate))) as usize
  }

  fn usb_controller_usb_passthrough_device_did_disconnect(
    &self,
    usb_controller: usize,
    device: VZUSBPassthroughDevice,
  ) {
    // SAFETY: `bridging(retained:)`'s address, a `VZUSBController`, taken
    // once.
    let usb_controller = unsafe { retained_or_nil(usb_controller) }.expect("Swift passes the controller");

    self
      .delegate
      .usb_controller_usb_passthrough_device_did_disconnect(usb_controller, device);
  }
}

// OtherDevices/USBControllerConfiguration.swift

/// `this` is any `VZUSBControllerConfiguration`; each device, any
/// `VZUSBDeviceConfiguration`.
pub fn vz_usb_controller_configuration_usb_devices(this: &NSObject) -> Vec<Retained<NSObject>> {
  // SAFETY: `usbDevices`, a retained `NSArray`.
  unsafe { bridge_objects(ffi::vz_usb_controller_configuration_usb_devices(borrowed(this))) }
}

/// `this` is any `VZUSBControllerConfiguration`; each device, any
/// `VZUSBDeviceConfiguration`.
pub fn vz_usb_controller_configuration_set_usb_devices(this: &NSObject, usb_devices: &[&NSObject]) {
  ffi::vz_usb_controller_configuration_set_usb_devices(borrowed(this), borrowed(&*ns_array(usb_devices)));
}

// OtherDevices/USBDevice.swift

/// `this` is any `VZUSBDevice`; the controller, any `VZUSBController`.
pub fn vz_usb_device_usb_controller(this: &NSObject) -> Option<Retained<NSObject>> {
  // SAFETY: `usbController`, a retained `VZUSBController` or `nil`.
  unsafe { retained_or_nil(ffi::vz_usb_device_usb_controller(borrowed(this))) }
}

/// `this` is any `VZUSBDevice`.
pub fn vz_usb_device_uuid(this: &NSObject) -> Retained<NSUUID> {
  // SAFETY: `uuid`, a retained `NSUUID`.
  unsafe { retained(ffi::vz_usb_device_uuid(borrowed(this))) }
}

// OtherDevices/USBDeviceConfiguration.swift

/// `this` is any `VZUSBDeviceConfiguration`.
pub fn vz_usb_device_configuration_uuid(this: &NSObject) -> Retained<NSUUID> {
  // SAFETY: `uuid`, a retained `NSUUID`.
  unsafe { retained(ffi::vz_usb_device_configuration_uuid(borrowed(this))) }
}

/// `this` is any `VZUSBDeviceConfiguration`.
pub fn vz_usb_device_configuration_set_uuid(this: &NSObject, uuid: &NSUUID) {
  ffi::vz_usb_device_configuration_set_uuid(borrowed(this), borrowed(uuid));
}

// OtherDevices/USBPassthroughDevice.swift

pub fn vz_usb_passthrough_device_init_configuration(
  configuration: &VZUSBPassthroughDeviceConfiguration,
) -> Result<VZUSBPassthroughDevice, BridgedThrown> {
  ffi::vz_usb_passthrough_device_init_configuration(configuration.bridge_retained())
}

pub fn vz_usb_passthrough_device_bridge_cast(object: &NSObject) -> Option<VZUSBPassthroughDevice> {
  ffi::vz_usb_passthrough_device_bridge_cast(borrowed(object)).ok()
}

impl VZUSBPassthroughDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/USBPassthroughDeviceConfiguration.swift

/// `device` crosses from accessory-access-sys's bridge, lent for the call.
pub fn vz_usb_passthrough_device_configuration_init(
  device: &accessory_access_sys::AAUSBAccessory,
) -> VZUSBPassthroughDeviceConfiguration {
  ffi::vz_usb_passthrough_device_configuration_init(device.bridge_borrowed())
}

pub fn vz_usb_passthrough_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZUSBPassthroughDeviceConfiguration> {
  ffi::vz_usb_passthrough_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZUSBPassthroughDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/VirtioEntropyDeviceConfiguration.swift

pub fn vz_virtio_entropy_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioEntropyDeviceConfiguration> {
  ffi::vz_virtio_entropy_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioEntropyDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/VirtioTraditionalMemoryBalloonDevice.swift

pub fn vz_virtio_traditional_memory_balloon_device_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioTraditionalMemoryBalloonDevice> {
  ffi::vz_virtio_traditional_memory_balloon_device_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioTraditionalMemoryBalloonDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/VirtioTraditionalMemoryBalloonDeviceConfiguration.swift

pub fn vz_virtio_traditional_memory_balloon_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioTraditionalMemoryBalloonDeviceConfiguration> {
  ffi::vz_virtio_traditional_memory_balloon_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/XHCIController.swift

pub fn vz_xhci_controller_bridge_cast(object: &NSObject) -> Option<VZXHCIController> {
  ffi::vz_xhci_controller_bridge_cast(borrowed(object)).ok()
}

impl VZXHCIController {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// OtherDevices/XHCIControllerConfiguration.swift

pub fn vz_xhci_controller_configuration_bridge_cast(object: &NSObject) -> Option<VZXHCIControllerConfiguration> {
  ffi::vz_xhci_controller_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZXHCIControllerConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/ConsolePortConfiguration.swift

/// `this` is any `VZConsolePortConfiguration`; the attachment, any
/// `VZSerialPortAttachment`.
pub fn vz_console_port_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  // SAFETY: `attachment`, a retained `VZSerialPortAttachment` or `nil`.
  unsafe { retained_or_nil(ffi::vz_console_port_configuration_attachment(borrowed(this))) }
}

/// `this` is any `VZConsolePortConfiguration`; `attachment`, any
/// `VZSerialPortAttachment`.
pub fn vz_console_port_configuration_set_attachment(this: &NSObject, attachment: Option<&NSObject>) {
  ffi::vz_console_port_configuration_set_attachment(borrowed(this), borrowed_or_nil(attachment));
}

// SerialPort/FileHandleSerialPortAttachment.swift

/// Each descriptor becomes a `FileHandle` that owns it.
pub fn vz_file_handle_serial_port_attachment_init(
  file_handle_for_reading: Option<OwnedFd>,
  file_handle_for_writing: Option<OwnedFd>,
) -> VZFileHandleSerialPortAttachment {
  let file_handle_for_reading = file_handle_for_reading.map(ns_file_handle_init_file_descriptor_close_on_dealloc);
  let file_handle_for_writing = file_handle_for_writing.map(ns_file_handle_init_file_descriptor_close_on_dealloc);

  ffi::vz_file_handle_serial_port_attachment_init(
    borrowed_or_nil(file_handle_for_reading.as_deref()),
    borrowed_or_nil(file_handle_for_writing.as_deref()),
  )
}

pub fn vz_file_handle_serial_port_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZFileHandleSerialPortAttachment> {
  ffi::vz_file_handle_serial_port_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZFileHandleSerialPortAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// `fileHandleForReading`'s descriptor, open as long as the attachment,
  /// which keeps the handle.
  pub fn vz_file_handle_for_reading(&self) -> Option<BorrowedFd<'_>> {
    // SAFETY: `fileHandleForReading`, a retained `FileHandle` or `nil`.
    let handle: Option<Retained<NSFileHandle>> = unsafe {
      retained_or_nil(ffi::vz_file_handle_serial_port_attachment_file_handle_for_reading(
        self.bridge_retained(),
      ))
    };

    // SAFETY: the attachment's handles are fixed, and a handle's descriptor
    // stays open while it lives.
    handle.map(|handle| unsafe { BorrowedFd::borrow_raw(ns_file_handle_file_descriptor(&handle)) })
  }

  /// `fileHandleForWriting`'s descriptor, open as long as the attachment,
  /// which keeps the handle.
  pub fn vz_file_handle_for_writing(&self) -> Option<BorrowedFd<'_>> {
    // SAFETY: `fileHandleForWriting`, a retained `FileHandle` or `nil`.
    let handle: Option<Retained<NSFileHandle>> = unsafe {
      retained_or_nil(ffi::vz_file_handle_serial_port_attachment_file_handle_for_writing(
        self.bridge_retained(),
      ))
    };

    // SAFETY: as for `vz_file_handle_for_reading`.
    handle.map(|handle| unsafe { BorrowedFd::borrow_raw(ns_file_handle_file_descriptor(&handle)) })
  }
}

// SerialPort/FileSerialPortAttachment.swift

pub fn vz_file_serial_port_attachment_init(
  url: &NSURL,
  append: bool,
) -> Result<VZFileSerialPortAttachment, BridgedThrown> {
  ffi::vz_file_serial_port_attachment_init(borrowed(url), append)
}

pub fn vz_file_serial_port_attachment_bridge_cast(object: &NSObject) -> Option<VZFileSerialPortAttachment> {
  ffi::vz_file_serial_port_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZFileSerialPortAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe { retained(ffi::vz_file_serial_port_attachment_url(self.bridge_retained())) }
  }
}

// SerialPort/SerialPortConfiguration.swift

/// `this` is any `VZSerialPortConfiguration`; the attachment, any
/// `VZSerialPortAttachment`.
pub fn vz_serial_port_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  // SAFETY: `attachment`, a retained `VZSerialPortAttachment` or `nil`.
  unsafe { retained_or_nil(ffi::vz_serial_port_configuration_attachment(borrowed(this))) }
}

/// `this` is any `VZSerialPortConfiguration`; `attachment`, any
/// `VZSerialPortAttachment`.
pub fn vz_serial_port_configuration_set_attachment(this: &NSObject, attachment: Option<&NSObject>) {
  ffi::vz_serial_port_configuration_set_attachment(borrowed(this), borrowed_or_nil(attachment));
}

// SerialPort/SpiceAgentPortAttachment.swift

pub fn vz_spice_agent_port_attachment_bridge_cast(object: &NSObject) -> Option<VZSpiceAgentPortAttachment> {
  ffi::vz_spice_agent_port_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZSpiceAgentPortAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/VirtioConsoleDevice.swift

pub fn vz_virtio_console_device_bridge_cast(object: &NSObject) -> Option<VZVirtioConsoleDevice> {
  ffi::vz_virtio_console_device_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioConsoleDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// The delegate set with [`Self::vz_set_delegate`].
  pub fn vz_delegate(&self) -> Option<Arc<dyn VZVirtioConsoleDeviceDelegate>> {
    let address = ffi::vz_virtio_console_device_delegate(self.bridge_retained());

    // SAFETY: `VZVirtioConsoleDeviceDelegateBridge::bridge_retained`'s
    // address, taken once, or `0` for none.
    (address != 0).then(|| *unsafe { Box::from_raw(address as *mut Arc<dyn VZVirtioConsoleDeviceDelegate>) })
  }

  pub fn vz_set_delegate(&self, delegate: Option<Arc<dyn VZVirtioConsoleDeviceDelegate>>) {
    match delegate {
      Some(delegate) => {
        ffi::vz_virtio_console_device_set_delegate(
          self.bridge_retained(),
          VZVirtioConsoleDeviceDelegateBridge { delegate },
        );
      }
      None => self.bridge_set_delegate_nil(),
    }
  }
}

/// `VZVirtioConsoleDeviceDelegate`, which Rust implements. Virtualization
/// calls it on the machine's queue.
pub trait VZVirtioConsoleDeviceDelegate: Any + Send + Sync {
  fn console_device_did_open(&self, console_device: VZVirtioConsoleDevice, console_port: VZVirtioConsolePort);

  fn console_device_did_close(&self, console_device: VZVirtioConsoleDevice, console_port: VZVirtioConsolePort);
}

/// A [`VZVirtioConsoleDeviceDelegate`] as Swift holds it.
pub struct VZVirtioConsoleDeviceDelegateBridge {
  delegate: Arc<dyn VZVirtioConsoleDeviceDelegate>,
}

impl VZVirtioConsoleDeviceDelegateBridge {
  /// Another reference to the delegate, for
  /// [`VZVirtioConsoleDevice::vz_delegate`] to take, by address.
  /// (swift-bridge's code for returning this type by value fails clippy.)
  fn bridge_retained(&self) -> usize {
    Box::into_raw(Box::new(Arc::clone(&self.delegate))) as usize
  }

  fn console_device_did_open(&self, console_device: VZVirtioConsoleDevice, console_port: VZVirtioConsolePort) {
    self
      .delegate
      .console_device_did_open(console_device, console_port);
  }

  fn console_device_did_close(&self, console_device: VZVirtioConsoleDevice, console_port: VZVirtioConsolePort) {
    self
      .delegate
      .console_device_did_close(console_device, console_port);
  }
}

// SerialPort/VirtioConsoleDeviceConfiguration.swift

pub fn vz_virtio_console_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioConsoleDeviceConfiguration> {
  ffi::vz_virtio_console_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioConsoleDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/VirtioConsoleDeviceSerialPortConfiguration.swift

pub fn vz_virtio_console_device_serial_port_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioConsoleDeviceSerialPortConfiguration> {
  ffi::vz_virtio_console_device_serial_port_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioConsoleDeviceSerialPortConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/VirtioConsolePort.swift

impl VZVirtioConsolePort {
  /// Any `VZSerialPortAttachment`.
  pub fn vz_attachment(&self) -> Option<Retained<NSObject>> {
    // SAFETY: `attachment`, a retained `VZSerialPortAttachment` or `nil`.
    unsafe { retained_or_nil(ffi::vz_virtio_console_port_attachment(self.bridge_retained())) }
  }

  /// `attachment` is any `VZSerialPortAttachment`.
  pub fn vz_set_attachment(&self, attachment: Option<&NSObject>) {
    ffi::vz_virtio_console_port_set_attachment(self.bridge_retained(), borrowed_or_nil(attachment));
  }
}

// SerialPort/VirtioConsolePortConfiguration.swift

pub fn vz_virtio_console_port_configuration_bridge_cast(object: &NSObject) -> Option<VZVirtioConsolePortConfiguration> {
  ffi::vz_virtio_console_port_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioConsolePortConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/VirtioSocketDevice.swift

pub fn vz_virtio_socket_device_bridge_cast(object: &NSObject) -> Option<VZVirtioSocketDevice> {
  ffi::vz_virtio_socket_device_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioSocketDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_connect(
    &self,
    port: u32,
    completion_handler: impl FnOnce(Result<VZVirtioSocketConnection, Retained<NSError>>) + Send + 'static,
  ) {
    ffi::vz_virtio_socket_device_connect(
      self.bridge_retained(),
      port,
      Box::new(move |connection, error| {
        // SAFETY: `bridging(retainedAddress:)`, a retained
        // `VZVirtioSocketConnection`, or `0` when it failed.
        let connection: Option<Retained<NSObject>> = unsafe { retained_or_nil(connection) };

        completion_handler(match connection {
          Some(connection) => Ok(ffi::vz_virtio_socket_connection_bridge_from(borrowed(&*connection))),
          // SAFETY: `bridging(retainedError:)`, a retained `NSError`.
          None => Err(unsafe { retained_or_nil(error) }.expect("Swift passes a connection or an error")),
        });
      }),
    );
  }
}

// SerialPort/VirtioSocketDeviceConfiguration.swift

pub fn vz_virtio_socket_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioSocketDeviceConfiguration> {
  ffi::vz_virtio_socket_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioSocketDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// SerialPort/VirtioSocketListener.swift

impl VZVirtioSocketListener {
  /// The delegate set with [`Self::vz_set_delegate`].
  pub fn vz_delegate(&self) -> Option<Arc<dyn VZVirtioSocketListenerDelegate>> {
    let address = ffi::vz_virtio_socket_listener_delegate(self.bridge_retained());

    // SAFETY: `VZVirtioSocketListenerDelegateBridge::bridge_retained`'s
    // address, taken once, or `0` for none.
    (address != 0).then(|| *unsafe { Box::from_raw(address as *mut Arc<dyn VZVirtioSocketListenerDelegate>) })
  }

  pub fn vz_set_delegate(&self, delegate: Option<Arc<dyn VZVirtioSocketListenerDelegate>>) {
    match delegate {
      Some(delegate) => {
        ffi::vz_virtio_socket_listener_set_delegate(
          self.bridge_retained(),
          VZVirtioSocketListenerDelegateBridge { delegate },
        );
      }
      None => self.bridge_set_delegate_nil(),
    }
  }
}

/// `VZVirtioSocketListenerDelegate`, which Rust implements. Virtualization
/// calls it on the machine's queue.
pub trait VZVirtioSocketListenerDelegate: Any + Send + Sync {
  fn listener_should_accept_new_connection_from(
    &self,
    listener: VZVirtioSocketListener,
    connection: VZVirtioSocketConnection,
    socket_device: VZVirtioSocketDevice,
  ) -> bool;
}

/// A [`VZVirtioSocketListenerDelegate`] as Swift holds it.
pub struct VZVirtioSocketListenerDelegateBridge {
  delegate: Arc<dyn VZVirtioSocketListenerDelegate>,
}

impl VZVirtioSocketListenerDelegateBridge {
  /// Another reference to the delegate, for
  /// [`VZVirtioSocketListener::vz_delegate`] to take, by address.
  /// (swift-bridge's code for returning this type by value fails clippy.)
  fn bridge_retained(&self) -> usize {
    Box::into_raw(Box::new(Arc::clone(&self.delegate))) as usize
  }

  fn listener_should_accept_new_connection_from(
    &self,
    listener: VZVirtioSocketListener,
    connection: VZVirtioSocketConnection,
    socket_device: VZVirtioSocketDevice,
  ) -> bool {
    self
      .delegate
      .listener_should_accept_new_connection_from(listener, connection, socket_device)
  }
}

// Storage/DiskBlockDeviceStorageDeviceAttachment.swift

/// The descriptor becomes a `FileHandle` that owns it.
pub fn vz_disk_block_device_storage_device_attachment_init(
  file_handle: OwnedFd,
  read_only: bool,
  synchronization_mode: VZDiskSynchronizationMode,
) -> Result<VZDiskBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_disk_block_device_storage_device_attachment_init(
    borrowed(&*ns_file_handle_init_file_descriptor_close_on_dealloc(file_handle)),
    read_only,
    synchronization_mode,
  )
}

pub fn vz_disk_block_device_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZDiskBlockDeviceStorageDeviceAttachment> {
  ffi::vz_disk_block_device_storage_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZDiskBlockDeviceStorageDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  /// `fileHandle`'s descriptor, open as long as the attachment, which keeps
  /// the handle.
  pub fn vz_file_handle(&self) -> BorrowedFd<'_> {
    // SAFETY: `fileHandle`, a retained `FileHandle`.
    let handle: Retained<NSFileHandle> = unsafe {
      retained(ffi::vz_disk_block_device_storage_device_attachment_file_handle(
        self.bridge_retained(),
      ))
    };

    // SAFETY: the attachment's handle is fixed, and a handle's descriptor
    // stays open while it lives.
    unsafe { BorrowedFd::borrow_raw(ns_file_handle_file_descriptor(&handle)) }
  }
}

// Storage/DiskImageStorageDeviceAttachment.swift

pub fn vz_disk_image_storage_device_attachment_init(
  url: &NSURL,
  read_only: bool,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_disk_image_storage_device_attachment_init(borrowed(url), read_only)
}

pub fn vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
  url: &NSURL,
  read_only: bool,
  caching_mode: VZDiskImageCachingMode,
  synchronization_mode: VZDiskImageSynchronizationMode,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
    borrowed(url),
    read_only,
    caching_mode,
    synchronization_mode,
  )
}

/// `diskImage` crosses from disk-image-kit-sys's bridge, lent for the call.
pub fn vz_disk_image_storage_device_attachment_init_disk_image_caching_mode_synchronization_mode(
  disk_image: &disk_image_kit_sys::DiskImage,
  caching_mode: VZDiskImageCachingMode,
  synchronization_mode: VZDiskImageSynchronizationMode,
) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_disk_image_storage_device_attachment_init_disk_image_caching_mode_synchronization_mode(
    disk_image.bridge_borrowed(),
    caching_mode,
    synchronization_mode,
  )
}

pub fn vz_disk_image_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZDiskImageStorageDeviceAttachment> {
  ffi::vz_disk_image_storage_device_attachment_bridge_cast(borrowed(object)).ok()
}

impl VZDiskImageStorageDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe { retained(ffi::vz_disk_image_storage_device_attachment_url(self.bridge_retained())) }
  }
}

// Storage/NVMExpressControllerDeviceConfiguration.swift

/// `attachment` is any `VZStorageDeviceAttachment`.
pub fn vz_nvm_express_controller_device_configuration_init(
  attachment: &NSObject,
) -> VZNVMExpressControllerDeviceConfiguration {
  ffi::vz_nvm_express_controller_device_configuration_init(borrowed(attachment))
}

pub fn vz_nvm_express_controller_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZNVMExpressControllerDeviceConfiguration> {
  ffi::vz_nvm_express_controller_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZNVMExpressControllerDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Storage/NetworkBlockDeviceStorageDeviceAttachment.swift

pub fn vz_network_block_device_storage_device_attachment_init(
  url: &NSURL,
) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_network_block_device_storage_device_attachment_init(borrowed(url))
}

/// `TimeInterval` is its seconds.
pub fn vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
  url: &NSURL,
  timeout: f64,
  is_forced_read_only: bool,
  synchronization_mode: VZDiskSynchronizationMode,
) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, BridgedThrown> {
  ffi::vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
    borrowed(url),
    timeout,
    is_forced_read_only,
    synchronization_mode,
  )
}

pub fn vz_network_block_device_storage_device_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZNetworkBlockDeviceStorageDeviceAttachment> {
  ffi::vz_network_block_device_storage_device_attachment_bridge_cast(borrowed(object)).ok()
}

pub fn vz_network_block_device_storage_device_attachment_validate(url: &NSURL) -> Result<(), BridgedThrown> {
  ffi::vz_network_block_device_storage_device_attachment_validate(borrowed(url))
}

impl VZNetworkBlockDeviceStorageDeviceAttachment {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }

  pub fn vz_url(&self) -> Retained<NSURL> {
    // SAFETY: `url`, a retained `NSURL`.
    unsafe {
      retained(ffi::vz_network_block_device_storage_device_attachment_url(
        self.bridge_retained(),
      ))
    }
  }

  /// The delegate set with [`Self::vz_set_delegate`].
  pub fn vz_delegate(&self) -> Option<Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>> {
    let address = ffi::vz_network_block_device_storage_device_attachment_delegate(self.bridge_retained());

    // SAFETY: `VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge::bridge_retained`'s
    // address, taken once, or `0` for none.
    (address != 0)
      .then(|| *unsafe { Box::from_raw(address as *mut Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>) })
  }

  pub fn vz_set_delegate(&self, delegate: Option<Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>>) {
    match delegate {
      Some(delegate) => {
        ffi::vz_network_block_device_storage_device_attachment_set_delegate(
          self.bridge_retained(),
          VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge { delegate },
        );
      }
      None => self.bridge_set_delegate_nil(),
    }
  }
}

/// `VZNetworkBlockDeviceStorageDeviceAttachmentDelegate`, which Rust
/// implements.
pub trait VZNetworkBlockDeviceStorageDeviceAttachmentDelegate: Any + Send + Sync {
  fn attachment_was_connected(&self, attachment: VZNetworkBlockDeviceStorageDeviceAttachment);

  fn attachment_did_encounter_error(
    &self,
    attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
    error: Retained<NSError>,
  );
}

/// A [`VZNetworkBlockDeviceStorageDeviceAttachmentDelegate`] as Swift holds
/// it.
pub struct VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge {
  delegate: Arc<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>,
}

impl VZNetworkBlockDeviceStorageDeviceAttachmentDelegateBridge {
  /// Another reference to the delegate, for
  /// [`VZNetworkBlockDeviceStorageDeviceAttachment::vz_delegate`] to take, by
  /// address. (swift-bridge's code for returning this type by value fails
  /// clippy.)
  fn bridge_retained(&self) -> usize {
    Box::into_raw(Box::new(Arc::clone(&self.delegate))) as usize
  }

  fn attachment_was_connected(&self, attachment: VZNetworkBlockDeviceStorageDeviceAttachment) {
    self.delegate.attachment_was_connected(attachment);
  }

  fn attachment_did_encounter_error(
    &self,
    attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
    error: *mut c_void,
  ) {
    // SAFETY: `error`, a retained `NSError`.
    let error = unsafe { retained(error) };

    self
      .delegate
      .attachment_did_encounter_error(attachment, error);
  }
}

// Storage/StorageDeviceConfiguration.swift

/// `this` is any `VZStorageDeviceConfiguration`; the attachment, any
/// `VZStorageDeviceAttachment`.
pub fn vz_storage_device_configuration_attachment(this: &NSObject) -> Retained<NSObject> {
  // SAFETY: `attachment`, a retained `VZStorageDeviceAttachment`.
  unsafe { retained(ffi::vz_storage_device_configuration_attachment(borrowed(this))) }
}

// Storage/USBMassStorageDevice.swift

pub fn vz_usb_mass_storage_device_init(configuration: &VZUSBMassStorageDeviceConfiguration) -> VZUSBMassStorageDevice {
  ffi::vz_usb_mass_storage_device_init(configuration.bridge_retained())
}

pub fn vz_usb_mass_storage_device_bridge_cast(object: &NSObject) -> Option<VZUSBMassStorageDevice> {
  ffi::vz_usb_mass_storage_device_bridge_cast(borrowed(object)).ok()
}

impl VZUSBMassStorageDevice {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Storage/USBMassStorageDeviceConfiguration.swift

/// `attachment` is any `VZStorageDeviceAttachment`.
pub fn vz_usb_mass_storage_device_configuration_init(attachment: &NSObject) -> VZUSBMassStorageDeviceConfiguration {
  ffi::vz_usb_mass_storage_device_configuration_init(borrowed(attachment))
}

pub fn vz_usb_mass_storage_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZUSBMassStorageDeviceConfiguration> {
  ffi::vz_usb_mass_storage_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZUSBMassStorageDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}

// Storage/VirtioBlockDeviceConfiguration.swift

/// `attachment` is any `VZStorageDeviceAttachment`.
pub fn vz_virtio_block_device_configuration_init(attachment: &NSObject) -> VZVirtioBlockDeviceConfiguration {
  ffi::vz_virtio_block_device_configuration_init(borrowed(attachment))
}

pub fn vz_virtio_block_device_configuration_bridge_cast(object: &NSObject) -> Option<VZVirtioBlockDeviceConfiguration> {
  ffi::vz_virtio_block_device_configuration_bridge_cast(borrowed(object)).ok()
}

impl VZVirtioBlockDeviceConfiguration {
  pub fn bridge_object(&self) -> &NSObject {
    // SAFETY: this one's own object.
    unsafe { bridge_object(self, self.bridge_borrowed()) }
  }
}
