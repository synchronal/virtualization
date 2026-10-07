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

use std::ffi::c_void;

use foundation_sys::objc::{NSArray, borrowed, borrowed_or_nil, ns_array, ns_array_objects, retained, retained_or_nil};
pub use foundation_sys::objc::{NSError, NSObject, NSURL, Retained, ns_url_absolute_string, ns_url_init_string};

pub use self::ffi::{
  BridgedNil, BridgedThrown, VZDiskImageCachingMode, VZDiskImageStorageDeviceAttachment,
  VZDiskImageSynchronizationMode, VZLinuxBootLoader, VZVirtioBlockDeviceConfiguration, VZVirtualMachineConfiguration,
  vz_virtio_block_device_configuration_validate_block_device_identifier, vz_virtual_machine_configuration_init,
  vz_virtual_machine_configuration_maximum_allowed_cpu_count,
  vz_virtual_machine_configuration_maximum_allowed_memory_size,
  vz_virtual_machine_configuration_minimum_allowed_cpu_count,
  vz_virtual_machine_configuration_minimum_allowed_memory_size, vz_virtual_machine_is_supported,
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
