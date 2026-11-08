//! Rust bindings for Apple's [Virtualization] framework.
//!
//! Virtualization as Apple ships it, with nothing added. Names drop `VZ` and
//! members are snake case; Apple's documentation is this crate's.
//!
//! Where Rust differs from Swift:
//!
//! - Objects are shared references, as in Swift: [`Clone`] is the same
//!   object, setters take `&self`, and nothing is `Send` or `Sync` but
//!   [`VirtualMachine`] and [`MacOSInstaller`], which move onto the machine's
//!   queue, a [`CustomVirtioDevice`] and its queues, their elements, and its
//!   shared memory regions, which move onto the device's, and what Apple
//!   declares `Sendable`. [`Debug`] is the object's `description`.
//! - An abstract class is a sealed trait, implemented by its subclasses. A
//!   member that returns one takes the subclass to cast it to, as Swift's
//!   `as?`, and returns `None` when it is another. An array of one is a slice
//!   of trait objects going in, and a `Vec` of each element cast coming out.
//! - `URL` is [`url::Url`] and `UUID` is [`uuid::Uuid`]; a dispatch queue is
//!   [`dispatch2`]'s.
//!   A `TimeInterval` is its seconds, an `f64`, and an `OperatingSystemVersion`
//!   is [`OperatingSystemVersion`]. A `FileHandle` is its descriptor: an
//!   [`OwnedFd`](std::os::fd::OwnedFd) going in, a
//!   [`BorrowedFd`](std::os::fd::BorrowedFd) coming out.
//! - A completion handler is a closure, called on the queue Virtualization
//!   calls Swift's on. A delegate is a trait object, held as weakly as Swift
//!   holds its delegate.
//! - DiskImageKit's types are [`disk_image_kit`]'s, vmnet's are
//!   [`vmnet_framework`]'s, and AccessoryAccess's are [`accessory_access`]'s.
//!   Security's `SecCertificate` is [`SecCertificate`], made from and read as
//!   its DER bytes. Core Graphics' `CGSize` is [`CGSize`], its `width` and `height`.
//!   AppKit's `NSScreen` is [`NSScreen`], from `screens` and `main`, which
//!   take a [`MainThreadMarker`]: they're `@MainActor`.
//! - Host memory a shared memory region maps is [`MemoryPages`], from
//!   Foundation's `NSAllocateMemoryPages`, which the region holds while it's
//!   mapped: never a raw pointer the guest could outlive.
//! - An enum has an `Unknown(rawValue)` case, for one newer than this crate.
//! - Initializers, class members, throwing members, and members that take or
//!   return a URL return `Result<_, Error>`.
//!
//! Off macOS, the same API compiles, and every initializer and class member
//! fails with [`Error::Unsupported`].
//!
//! [Virtualization]: https://developer.apple.com/documentation/virtualization

mod app_kit;
mod boot_loader;
mod core_graphics;
mod custom_virtio_devices;
mod directory_sharing;
mod error;
mod foundation;
mod graphics;
mod guest_memory_mapping;
mod guest_provisioning_options;
mod macos;
mod network;
mod other_devices;
mod platform;
mod sealed;
mod security;
mod serial_port;
mod storage;
mod virtual_machine;
mod virtual_machine_configuration;
mod virtual_machine_view;

pub use crate::app_kit::{MainThreadMarker, NSScreen};
pub use crate::boot_loader::{
  BootLoader, EFIBootLoader, EFISignature, EFISignatureDatabaseConfiguration, EFISignatureList, EFIVariableStore,
  EFIVariableStoreInitializationOptions, GenericMachineIdentifier, GenericPlatformConfiguration, LinuxBootLoader,
  PlatformConfiguration,
};
pub use crate::core_graphics::CGSize;
pub use crate::custom_virtio_devices::{
  CustomVirtioDevice, CustomVirtioDeviceConfiguration, CustomVirtioDeviceConfigurationDelegate,
  CustomVirtioDeviceDelegate, CustomVirtioDeviceDelegateProvider, CustomVirtioDeviceProvider,
  NegotiatedVirtioFeatureSet, VirtioDeviceSpecificConfiguration, VirtioFeatureSet, VirtioQueue, VirtioQueueElement,
  VirtioSharedMemoryRegion, VirtioSharedMemoryRegionConfiguration,
};
pub use crate::directory_sharing::{
  DirectoryShare, DirectorySharingDevice, DirectorySharingDeviceConfiguration, MultipleDirectoryShare, SharedDirectory,
  SingleDirectoryShare, VirtioFileSystemDevice, VirtioFileSystemDeviceConfiguration,
};
pub use crate::error::{Error, ErrorCode, NSError, error_domain};
pub use crate::foundation::{MemoryPages, OperatingSystemVersion, ns_page_size};
pub use crate::graphics::{
  AudioDeviceConfiguration, AudioInputStreamSource, AudioOutputStreamSink, GraphicsDevice, GraphicsDeviceConfiguration,
  GraphicsDisplay, GraphicsDisplayConfiguration, GraphicsDisplayObserver, HostAudioInputStreamSource,
  HostAudioOutputStreamSink, KeyboardConfiguration, MacGraphicsDevice, MacGraphicsDeviceConfiguration,
  MacGraphicsDisplay, MacGraphicsDisplayConfiguration, MacKeyboardConfiguration, MacTrackpadConfiguration,
  PointingDeviceConfiguration, USBKeyboardConfiguration, USBScreenCoordinatePointingDeviceConfiguration,
  VirtioGraphicsDevice, VirtioGraphicsDeviceConfiguration, VirtioGraphicsScanout, VirtioGraphicsScanoutConfiguration,
  VirtioSoundDeviceConfiguration, VirtioSoundDeviceInputStreamConfiguration,
  VirtioSoundDeviceOutputStreamConfiguration, VirtioSoundDeviceStreamConfiguration,
};
pub use crate::guest_memory_mapping::GuestMemoryMapping;
pub use crate::guest_provisioning_options::GuestProvisioningOptions;
pub use crate::macos::{
  MacAuxiliaryStorage, MacAuxiliaryStorageInitializationOptions, MacGuestProvisioningOptions, MacHardwareModel,
  MacMachineIdentifier, MacOSBootLoader, MacOSConfigurationRequirements, MacOSInstaller, MacOSRestoreImage,
  MacOSVirtualMachineStartOptions, MacPlatformConfiguration,
};
pub use crate::network::{
  BridgedNetworkDeviceAttachment, BridgedNetworkInterface, FileHandleNetworkDeviceAttachment, MACAddress,
  NATNetworkDeviceAttachment, NetworkDevice, NetworkDeviceAttachment, NetworkDeviceConfiguration,
  VirtioNetworkDeviceConfiguration, VmnetNetworkDeviceAttachment,
};
pub use crate::other_devices::{
  EntropyDeviceConfiguration, MemoryBalloonDevice, MemoryBalloonDeviceConfiguration, USBController,
  USBControllerConfiguration, USBControllerDelegate, USBDevice, USBDeviceConfiguration, USBPassthroughDevice,
  USBPassthroughDeviceConfiguration, VirtioEntropyDeviceConfiguration, VirtioTraditionalMemoryBalloonDevice,
  VirtioTraditionalMemoryBalloonDeviceConfiguration, XHCIController, XHCIControllerConfiguration,
};
pub use crate::security::SecCertificate;
pub use crate::serial_port::{
  ConsoleDevice, ConsoleDeviceConfiguration, ConsolePortConfiguration, FileHandleSerialPortAttachment,
  FileSerialPortAttachment, SerialPortAttachment, SerialPortConfiguration, SocketDevice, SocketDeviceConfiguration,
  SpiceAgentPortAttachment, VirtioConsoleDevice, VirtioConsoleDeviceConfiguration, VirtioConsoleDeviceDelegate,
  VirtioConsoleDeviceSerialPortConfiguration, VirtioConsolePort, VirtioConsolePortArray,
  VirtioConsolePortConfiguration, VirtioConsolePortConfigurationArray, VirtioSocketConnection, VirtioSocketDevice,
  VirtioSocketDeviceConfiguration, VirtioSocketListener, VirtioSocketListenerDelegate,
};
pub use crate::storage::{
  DiskBlockDeviceStorageDeviceAttachment, DiskImageCachingMode, DiskImageStorageDeviceAttachment,
  DiskImageSynchronizationMode, DiskSynchronizationMode, NVMExpressControllerDeviceConfiguration,
  NetworkBlockDeviceStorageDeviceAttachment, NetworkBlockDeviceStorageDeviceAttachmentDelegate, StorageDevice,
  StorageDeviceAttachment, StorageDeviceConfiguration, USBMassStorageDevice, USBMassStorageDeviceConfiguration,
  VirtioBlockDeviceConfiguration,
};
pub use crate::virtual_machine::{
  VirtualMachine, VirtualMachineDelegate, VirtualMachineStartOptions, VirtualMachineState,
};
pub use crate::virtual_machine_configuration::VirtualMachineConfiguration;
pub use crate::virtual_machine_view::{VirtualMachineView, VirtualMachineViewAdaptor};

// Each class is `Send + Sync` off macOS exactly where virtualization-sys makes
// it so on macOS.
#[cfg(test)]
apple_bridge_foundation::assert_auto_traits! {
  send_sync: [
    CustomVirtioDevice,
    EFISignature,
    EFISignatureDatabaseConfiguration,
    EFISignatureList,
    MacGuestProvisioningOptions,
    MacOSInstaller,
    MemoryPages,
    SecCertificate,
    VirtioQueue,
    VirtioQueueElement,
    VirtioSharedMemoryRegion,
    VirtualMachine,
    VirtualMachineViewAdaptor,
  ],
  local: [
    BridgedNetworkDeviceAttachment,
    BridgedNetworkInterface,
    CustomVirtioDeviceConfiguration,
    CustomVirtioDeviceDelegateProvider,
    DiskBlockDeviceStorageDeviceAttachment,
    DiskImageStorageDeviceAttachment,
    EFIBootLoader,
    EFIVariableStore,
    FileHandleNetworkDeviceAttachment,
    FileHandleSerialPortAttachment,
    FileSerialPortAttachment,
    GenericMachineIdentifier,
    GenericPlatformConfiguration,
    GuestMemoryMapping,
    HostAudioInputStreamSource,
    HostAudioOutputStreamSink,
    LinuxBootLoader,
    MACAddress,
    MacAuxiliaryStorage,
    MacGraphicsDevice,
    MacGraphicsDeviceConfiguration,
    MacGraphicsDisplay,
    MacGraphicsDisplayConfiguration,
    MacHardwareModel,
    MacKeyboardConfiguration,
    MacMachineIdentifier,
    MacOSBootLoader,
    MacOSConfigurationRequirements,
    MacOSRestoreImage,
    MacOSVirtualMachineStartOptions,
    MacPlatformConfiguration,
    MacTrackpadConfiguration,
    MainThreadMarker,
    MultipleDirectoryShare,
    NATNetworkDeviceAttachment,
    NSScreen,
    NVMExpressControllerDeviceConfiguration,
    NegotiatedVirtioFeatureSet,
    NetworkBlockDeviceStorageDeviceAttachment,
    NetworkDevice,
    SharedDirectory,
    SingleDirectoryShare,
    SpiceAgentPortAttachment,
    USBKeyboardConfiguration,
    USBMassStorageDevice,
    USBMassStorageDeviceConfiguration,
    USBPassthroughDevice,
    USBPassthroughDeviceConfiguration,
    USBScreenCoordinatePointingDeviceConfiguration,
    VirtioBlockDeviceConfiguration,
    VirtioConsoleDevice,
    VirtioConsoleDeviceConfiguration,
    VirtioConsoleDeviceSerialPortConfiguration,
    VirtioConsolePort,
    VirtioConsolePortArray,
    VirtioConsolePortConfiguration,
    VirtioConsolePortConfigurationArray,
    VirtioDeviceSpecificConfiguration,
    VirtioEntropyDeviceConfiguration,
    VirtioFeatureSet,
    VirtioFileSystemDevice,
    VirtioFileSystemDeviceConfiguration,
    VirtioGraphicsDevice,
    VirtioGraphicsDeviceConfiguration,
    VirtioGraphicsScanout,
    VirtioGraphicsScanoutConfiguration,
    VirtioNetworkDeviceConfiguration,
    VirtioSharedMemoryRegionConfiguration,
    VirtioSocketConnection,
    VirtioSocketDevice,
    VirtioSocketDeviceConfiguration,
    VirtioSocketListener,
    VirtioSoundDeviceConfiguration,
    VirtioSoundDeviceInputStreamConfiguration,
    VirtioSoundDeviceOutputStreamConfiguration,
    VirtioTraditionalMemoryBalloonDevice,
    VirtioTraditionalMemoryBalloonDeviceConfiguration,
    VirtualMachineConfiguration,
    VirtualMachineView,
    VmnetNetworkDeviceAttachment,
    XHCIController,
    XHCIControllerConfiguration,
  ],
}
