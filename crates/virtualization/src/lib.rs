//! Rust bindings for Apple's [Virtualization] framework.
//!
//! Virtualization as Apple ships it, with nothing added. Names drop `VZ` and
//! members are snake case; Apple's documentation is this crate's.
//!
//! Where Rust differs from Swift:
//!
//! - Objects are shared references, as in Swift: [`Clone`] is the same
//!   object, setters take `&self`, and nothing is `Send` or `Sync` but
//!   [`VirtualMachine`], which moves onto its queue, and what Apple declares
//!   `Sendable`. [`Debug`] is the object's `description`.
//! - An abstract class is a sealed trait, implemented by its subclasses. A
//!   member that returns one takes the subclass to cast it to, as Swift's
//!   `as?`, and returns `None` when it is another. An array of one is a slice
//!   of trait objects going in, and a `Vec` of each element cast coming out.
//! - `URL` is [`url::Url`] and `UUID` is [`uuid::Uuid`], re-exported as
//!   [`url`] and [`uuid`]; a dispatch queue is [`dispatch2`]'s, re-exported.
//!   A `TimeInterval` is its seconds, an `f64`. A `FileHandle` is its descriptor: an
//!   [`OwnedFd`](std::os::fd::OwnedFd) going in, a
//!   [`BorrowedFd`](std::os::fd::BorrowedFd) coming out.
//! - A completion handler is a closure, called on the queue Virtualization
//!   calls Swift's on. A delegate is a trait object, held as weakly as Swift
//!   holds its delegate.
//! - DiskImageKit's types are [`disk_image_kit`]'s, vmnet's are [`vmnet`]'s,
//!   AccessoryAccess's are [`accessory_access`]'s, and IOUSBHost's
//!   [`io_usb_host`]'s, re-exported. Security's
//!   `SecCertificate` is [`SecCertificate`], made from and read as its DER
//!   bytes. Core Graphics' `CGSize` is [`CGSize`], its `width` and `height`.
//!   AppKit's `NSScreen` is [`NSScreen`], from `screens` and `main`, which
//!   take a [`MainThreadMarker`]: they're `@MainActor`.
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
mod directory_sharing;
mod error;
mod foundation;
mod graphics;
mod network;
mod other_devices;
mod platform;
mod sealed;
mod security;
mod serial_port;
mod storage;
mod virtual_machine;
mod virtual_machine_configuration;

pub use accessory_access::io_usb_host;
pub use {accessory_access, disk_image_kit, dispatch2, url, uuid, vmnet};

pub use crate::app_kit::{MainThreadMarker, NSScreen};
pub use crate::boot_loader::{
  BootLoader, EFIBootLoader, EFISignature, EFISignatureDatabaseConfiguration, EFISignatureList, EFIVariableStore,
  EFIVariableStoreInitializationOptions, GenericMachineIdentifier, GenericPlatformConfiguration, LinuxBootLoader,
  PlatformConfiguration,
};
pub use crate::core_graphics::CGSize;
pub use crate::directory_sharing::{
  DirectoryShare, DirectorySharingDevice, DirectorySharingDeviceConfiguration, MultipleDirectoryShare, SharedDirectory,
  SingleDirectoryShare, VirtioFileSystemDevice, VirtioFileSystemDeviceConfiguration,
};
pub use crate::error::{Error, NSError};
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
