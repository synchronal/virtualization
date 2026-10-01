//! FFI to Apple's [Virtualization] framework, through swift-bridge. Use the
//! `virtualization` crate instead; this one holds every `unsafe`.
//!
//! Signatures are Apple's, as far as swift-bridge can carry them:
//!
//! - Classes keep Apple's names. Members are methods `vz_<member>` and
//!   `vz_set_<member>`; Foundation's are `ns_<member>`.
//! - Initializers and class members are free functions:
//!   `vz_<type>_init[_<later labels>]`, `vz_<type>_<member>`.
//! - `bridge_*` members cross what swift-bridge can't: `bridge_retained` is
//!   another reference to the same object (opaque Swift types pass only by
//!   value), and `NSError`'s collections are read by index.
//! - swift-bridge can't pass `Option` of an opaque Swift type. One returned is
//!   `Result<T, BridgedNil>`; a setter taking one is `vz_set_<member>` for a
//!   value and `bridge_set_<member>_nil` for `nil`.
//! - A throwing function returns `Result<T, BridgedThrown>`, and
//!   [`ffi::bridge_take_thrown_error`] then returns the `NSError` it threw.
//!   (swift-bridge 0.1.59 calls a function whose `Result` has an opaque type
//!   on both sides more than once, so the error can't be the `Err`.)
//! - Enums keep Apple's names and cases, plus `Unknown(rawValue)` for a case
//!   newer than this crate.
//!
//! One bridge module, because swift-bridge can't share an opaque Swift type
//! (`NSURL`, `NSError`) between modules. It is grouped as the Swift package is.
//!
//! macOS only: the build fails elsewhere.
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

// No doc comments inside the module: swift-bridge can't parse them. Shared
// enums are renamed on the Swift side, where Apple's names are taken.
#[swift_bridge::bridge]
pub mod ffi {
  // Foundation.swift

  enum BridgedNil {
    Nil,
  }

  enum BridgedThrown {
    Thrown,
  }

  extern "Swift" {
    type NSURL;

    fn ns_url_init_string(string: &str) -> Result<NSURL, BridgedNil>;
    fn ns_absolute_string(self: &NSURL) -> String;
  }

  extern "Swift" {
    type NSError;

    fn bridge_take_thrown_error() -> NSError;
    fn ns_domain(self: &NSError) -> String;
    fn ns_code(self: &NSError) -> isize;
    fn ns_localized_description(self: &NSError) -> String;
    fn ns_localized_failure_reason(self: &NSError) -> Option<String>;
    fn ns_localized_recovery_suggestion(self: &NSError) -> Option<String>;
    fn bridge_user_info_count(self: &NSError) -> usize;
    fn bridge_user_info_key_at(self: &NSError, index: usize) -> String;
    fn bridge_user_info_value_description_at(self: &NSError, index: usize) -> String;
    fn bridge_underlying_errors_count(self: &NSError) -> usize;
    fn bridge_underlying_error_at(self: &NSError, index: usize) -> NSError;
  }

  // VirtualMachine.swift

  extern "Swift" {
    fn vz_virtual_machine_is_supported() -> bool;
  }

  // BootLoader/LinuxBootLoader.swift

  extern "Swift" {
    type VZLinuxBootLoader;

    fn vz_linux_boot_loader_init(kernel_url: NSURL) -> VZLinuxBootLoader;
    fn bridge_retained(self: &VZLinuxBootLoader) -> VZLinuxBootLoader;
    fn ns_description(self: &VZLinuxBootLoader) -> String;
    fn vz_kernel_url(self: &VZLinuxBootLoader) -> NSURL;
    fn vz_set_kernel_url(self: &VZLinuxBootLoader, kernel_url: NSURL);
    fn vz_command_line(self: &VZLinuxBootLoader) -> String;
    fn vz_set_command_line(self: &VZLinuxBootLoader, command_line: &str);
    fn vz_initial_ramdisk_url(self: &VZLinuxBootLoader) -> Result<NSURL, BridgedNil>;
    fn vz_set_initial_ramdisk_url(self: &VZLinuxBootLoader, initial_ramdisk_url: NSURL);
    fn bridge_set_initial_ramdisk_url_nil(self: &VZLinuxBootLoader);
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
      url: NSURL,
      read_only: bool,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown>;
    fn vz_disk_image_storage_device_attachment_init_caching_mode_synchronization_mode(
      url: NSURL,
      read_only: bool,
      caching_mode: VZDiskImageCachingMode,
      synchronization_mode: VZDiskImageSynchronizationMode,
    ) -> Result<VZDiskImageStorageDeviceAttachment, BridgedThrown>;
    fn bridge_retained(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageStorageDeviceAttachment;
    fn ns_description(self: &VZDiskImageStorageDeviceAttachment) -> String;
    fn vz_url(self: &VZDiskImageStorageDeviceAttachment) -> NSURL;
    fn vz_is_read_only(self: &VZDiskImageStorageDeviceAttachment) -> bool;
    fn vz_caching_mode(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageCachingMode;
    fn vz_synchronization_mode(self: &VZDiskImageStorageDeviceAttachment) -> VZDiskImageSynchronizationMode;
  }
}
