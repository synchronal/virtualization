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

use std::ffi::c_void;

pub use foundation_sys::objc::{NSError, NSObject, NSURL, Retained, ns_url_absolute_string, ns_url_init_string};
use foundation_sys::objc::{borrowed, borrowed_or_nil, retained, retained_or_nil};

pub use self::ffi::{
  BridgedNil, BridgedThrown, VZDiskImageCachingMode, VZDiskImageStorageDeviceAttachment,
  VZDiskImageSynchronizationMode, VZLinuxBootLoader, VZVirtioBlockDeviceConfiguration,
  vz_virtio_block_device_configuration_validate_block_device_identifier, vz_virtual_machine_is_supported,
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

  // VirtualMachine.swift

  extern "Swift" {
    fn vz_virtual_machine_is_supported() -> bool;
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

  // Storage/StorageDeviceConfiguration.swift

  extern "Swift" {
    fn vz_storage_device_configuration_attachment(this: *const c_void) -> *mut c_void;
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

// Foundation.swift

/// What the last throwing call on this thread threw. Call it on that thread,
/// only after the call returned [`BridgedThrown`].
pub fn bridge_take_thrown_error() -> Retained<NSError> {
  // SAFETY: `bridge_take_thrown_error` returns a retained `NSError`.
  unsafe { retained(ffi::bridge_take_thrown_error()) }
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

// Storage/StorageDeviceConfiguration.swift

/// `this` is any `VZStorageDeviceConfiguration`; the attachment, any
/// `VZStorageDeviceAttachment`.
pub fn vz_storage_device_configuration_attachment(this: &NSObject) -> Retained<NSObject> {
  // SAFETY: `attachment`, a retained `VZStorageDeviceAttachment`.
  unsafe { retained(ffi::vz_storage_device_configuration_attachment(borrowed(this))) }
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
