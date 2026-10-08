use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use dispatch2::{DispatchQueue, DispatchRetained};
use url::Url;

use crate::platform::{self, ffi};
use crate::sealed::{Sealed, bridge_cast_each};
use crate::{
  ConsoleDevice, DirectorySharingDevice, Error, GraphicsDevice, MemoryBalloonDevice, NetworkDevice, SocketDevice,
  USBController, VirtualMachineConfiguration, foundation,
};

/// [`VZVirtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine)
///
/// `Send` and `Sync`, so it can move onto its queue, where Virtualization takes
/// every call on it but [`Self::queue`]; calling it elsewhere is the caller's
/// to avoid, as in Swift. Completion handlers and the delegate are called on
/// that queue.
pub struct VirtualMachine {
  inner: ffi::VZVirtualMachine,
}

impl VirtualMachine {
  /// [`init(configuration:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/init(configuration:))
  pub fn new(configuration: &VirtualMachineConfiguration) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtual_machine_init(&configuration.inner),
    })
  }

  /// [`init(configuration:queue:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/init(configuration:queue:))
  pub fn with_queue(configuration: &VirtualMachineConfiguration, queue: &DispatchQueue) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtual_machine_init_queue(&configuration.inner, queue),
    })
  }

  /// [`isSupported`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/issupported)
  pub fn is_supported() -> Result<bool, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_is_supported())
  }

  /// [`queue`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/queue)
  pub fn queue(&self) -> DispatchRetained<DispatchQueue> {
    self.inner.vz_queue()
  }

  /// [`state`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state)
  pub fn state(&self) -> VirtualMachineState {
    self.inner.vz_state().into()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/delegate):
  /// `None` when there is none, or it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn VirtualMachineDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/delegate),
  /// held weakly, as in Swift.
  pub fn set_delegate(&self, delegate: Option<&Arc<dyn VirtualMachineDelegate>>) {
    self.inner.vz_set_delegate(
      delegate.map(|delegate| Arc::new(Delegating(Arc::downgrade(delegate))) as Arc<dyn ffi::VZVirtualMachineDelegate>),
    );
  }

  /// [`canStart`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/canstart)
  pub fn can_start(&self) -> bool {
    self.inner.vz_can_start()
  }

  /// [`canStop`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/canstop)
  pub fn can_stop(&self) -> bool {
    self.inner.vz_can_stop()
  }

  /// [`canPause`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/canpause)
  pub fn can_pause(&self) -> bool {
    self.inner.vz_can_pause()
  }

  /// [`canResume`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/canresume)
  pub fn can_resume(&self) -> bool {
    self.inner.vz_can_resume()
  }

  /// [`canRequestStop`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/canrequeststop)
  pub fn can_request_stop(&self) -> bool {
    self.inner.vz_can_request_stop()
  }

  /// [`consoleDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/consoledevices)
  pub fn console_devices<T: ConsoleDevice>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_console_devices())
  }

  /// [`directorySharingDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/directorysharingdevices)
  pub fn directory_sharing_devices<T: DirectorySharingDevice>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_directory_sharing_devices())
  }

  /// [`graphicsDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/graphicsdevices)
  pub fn graphics_devices<T: GraphicsDevice>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_graphics_devices())
  }

  /// [`memoryBalloonDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/memoryballoondevices)
  pub fn memory_balloon_devices<T: MemoryBalloonDevice>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_memory_balloon_devices())
  }

  /// [`networkDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/networkdevices)
  pub fn network_devices(&self) -> Vec<NetworkDevice> {
    self
      .inner
      .vz_network_devices()
      .into_iter()
      .map(NetworkDevice::bridge_from)
      .collect()
  }

  /// [`socketDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/socketdevices)
  pub fn socket_devices<T: SocketDevice>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_socket_devices())
  }

  /// [`usbControllers`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/usbcontrollers)
  pub fn usb_controllers<T: USBController>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_usb_controllers())
  }

  /// [`start(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/start(completionhandler:))
  pub fn start(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .vz_start(move |error| completion_handler(foundation::completed(error)));
  }

  /// [`start(options:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/start(options:completionhandler:))
  pub fn start_with_options(
    &self,
    options: &dyn VirtualMachineStartOptions,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) {
    self
      .inner
      .vz_start_options(options.bridge_object(), move |error| {
        completion_handler(foundation::completed(error));
      });
  }

  /// [`stop(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/stop(completionhandler:))
  pub fn stop(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .vz_stop(move |error| completion_handler(foundation::completed(error)));
  }

  /// [`pause(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/pause(completionhandler:))
  pub fn pause(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .vz_pause(move |error| completion_handler(foundation::completed(error)));
  }

  /// [`resume(completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/resume(completionhandler:))
  pub fn resume(&self, completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static) {
    self
      .inner
      .vz_resume(move |error| completion_handler(foundation::completed(error)));
  }

  /// [`requestStop()`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/requeststop())
  pub fn request_stop(&self) -> Result<(), Error> {
    self.inner.vz_request_stop().map_err(foundation::thrown)
  }

  /// [`restoreMachineStateFrom(url:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/restoremachinestatefrom(url:completionhandler:)).
  /// Fails, without calling `completion_handler`, if Foundation can't
  /// represent `url`.
  pub fn restore_machine_state_from(
    &self,
    url: &Url,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) -> Result<(), Error> {
    self
      .inner
      .vz_restore_machine_state_from(&*foundation::ns_url(url)?, move |error| {
        completion_handler(foundation::completed(error));
      });
    Ok(())
  }

  /// [`saveMachineStateTo(url:completionHandler:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/savemachinestateto(url:completionhandler:)).
  /// Fails, without calling `completion_handler`, if Foundation can't
  /// represent `url`.
  pub fn save_machine_state_to(
    &self,
    url: &Url,
    completion_handler: impl FnOnce(Result<(), Error>) + Send + 'static,
  ) -> Result<(), Error> {
    self
      .inner
      .vz_save_machine_state_to(&*foundation::ns_url(url)?, move |error| {
        completion_handler(foundation::completed(error));
      });
    Ok(())
  }
}

impl Clone for VirtualMachine {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtualMachine {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZVirtualMachine.State`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum VirtualMachineState {
  /// [`stopped`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/stopped)
  Stopped,
  /// [`running`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/running)
  Running,
  /// [`paused`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/paused)
  Paused,
  /// [`error`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/error)
  Error,
  /// [`starting`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/starting)
  Starting,
  /// [`pausing`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/pausing)
  Pausing,
  /// [`resuming`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/resuming)
  Resuming,
  /// [`stopping`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/stopping)
  Stopping,
  /// [`saving`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/saving)
  Saving,
  /// [`restoring`](https://developer.apple.com/documentation/virtualization/vzvirtualmachine/state-swift.enum/restoring)
  Restoring,
  /// A case newer than this crate, by its raw value.
  Unknown(isize),
}

impl From<VirtualMachineState> for ffi::VZVirtualMachineState {
  fn from(state: VirtualMachineState) -> Self {
    match state {
      VirtualMachineState::Stopped => Self::Stopped,
      VirtualMachineState::Running => Self::Running,
      VirtualMachineState::Paused => Self::Paused,
      VirtualMachineState::Error => Self::Error,
      VirtualMachineState::Starting => Self::Starting,
      VirtualMachineState::Pausing => Self::Pausing,
      VirtualMachineState::Resuming => Self::Resuming,
      VirtualMachineState::Stopping => Self::Stopping,
      VirtualMachineState::Saving => Self::Saving,
      VirtualMachineState::Restoring => Self::Restoring,
      VirtualMachineState::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<ffi::VZVirtualMachineState> for VirtualMachineState {
  fn from(state: ffi::VZVirtualMachineState) -> Self {
    match state {
      ffi::VZVirtualMachineState::Stopped => Self::Stopped,
      ffi::VZVirtualMachineState::Running => Self::Running,
      ffi::VZVirtualMachineState::Paused => Self::Paused,
      ffi::VZVirtualMachineState::Error => Self::Error,
      ffi::VZVirtualMachineState::Starting => Self::Starting,
      ffi::VZVirtualMachineState::Pausing => Self::Pausing,
      ffi::VZVirtualMachineState::Resuming => Self::Resuming,
      ffi::VZVirtualMachineState::Stopping => Self::Stopping,
      ffi::VZVirtualMachineState::Saving => Self::Saving,
      ffi::VZVirtualMachineState::Restoring => Self::Restoring,
      ffi::VZVirtualMachineState::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

/// [`VZVirtualMachineDelegate`](https://developer.apple.com/documentation/virtualization/vzvirtualmachinedelegate)
///
/// Each method is optional, as in Swift: unimplemented, it does nothing.
/// Virtualization calls them on the machine's queue.
pub trait VirtualMachineDelegate: Send + Sync {
  /// [`guestDidStop(_:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachinedelegate/guestdidstop(_:))
  fn guest_did_stop(&self, _virtual_machine: &VirtualMachine) {}

  /// [`virtualMachine(_:didStopWithError:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachinedelegate/virtualmachine(_:didstopwitherror:))
  fn virtual_machine_did_stop_with_error(&self, _virtual_machine: &VirtualMachine, _error: Error) {}

  /// [`virtualMachine(_:networkDevice:attachmentWasDisconnectedWithError:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachinedelegate/virtualmachine(_:networkdevice:attachmentwasdisconnectedwitherror:))
  fn virtual_machine_network_device_attachment_was_disconnected_with_error(
    &self,
    _virtual_machine: &VirtualMachine,
    _network_device: &NetworkDevice,
    _error: Error,
  ) {
  }
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn VirtualMachineDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZVirtualMachineDelegate for Delegating {
  fn guest_did_stop(&self, virtual_machine: ffi::VZVirtualMachine) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.guest_did_stop(&VirtualMachine { inner: virtual_machine });
    }
  }

  fn virtual_machine_did_stop_with_error(
    &self,
    virtual_machine: ffi::VZVirtualMachine,
    error: ffi::Retained<ffi::NSError>,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate
        .virtual_machine_did_stop_with_error(&VirtualMachine { inner: virtual_machine }, foundation::error(&error));
    }
  }

  fn virtual_machine_network_device_attachment_was_disconnected_with_error(
    &self,
    virtual_machine: ffi::VZVirtualMachine,
    network_device: ffi::VZNetworkDevice,
    error: ffi::Retained<ffi::NSError>,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.virtual_machine_network_device_attachment_was_disconnected_with_error(
        &VirtualMachine { inner: virtual_machine },
        &NetworkDevice::bridge_from(network_device),
        foundation::error(&error),
      );
    }
  }
}

/// [`VZVirtualMachineStartOptions`](https://developer.apple.com/documentation/virtualization/vzvirtualmachinestartoptions)
pub trait VirtualMachineStartOptions: Sealed {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn state_becomes_the_same_case_in_virtualization_sys() {
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Stopped),
      ffi::VZVirtualMachineState::Stopped
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Running),
      ffi::VZVirtualMachineState::Running
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Paused),
      ffi::VZVirtualMachineState::Paused
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Error),
      ffi::VZVirtualMachineState::Error
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Starting),
      ffi::VZVirtualMachineState::Starting
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Pausing),
      ffi::VZVirtualMachineState::Pausing
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Resuming),
      ffi::VZVirtualMachineState::Resuming
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Stopping),
      ffi::VZVirtualMachineState::Stopping
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Saving),
      ffi::VZVirtualMachineState::Saving
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Restoring),
      ffi::VZVirtualMachineState::Restoring
    ));
    assert!(matches!(
      ffi::VZVirtualMachineState::from(VirtualMachineState::Unknown(7)),
      ffi::VZVirtualMachineState::Unknown(7)
    ));
  }

  #[test]
  fn state_comes_from_the_same_case_in_virtualization_sys() {
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Stopped),
      VirtualMachineState::Stopped
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Running),
      VirtualMachineState::Running
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Paused),
      VirtualMachineState::Paused
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Error),
      VirtualMachineState::Error
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Starting),
      VirtualMachineState::Starting
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Pausing),
      VirtualMachineState::Pausing
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Resuming),
      VirtualMachineState::Resuming
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Stopping),
      VirtualMachineState::Stopping
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Saving),
      VirtualMachineState::Saving
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Restoring),
      VirtualMachineState::Restoring
    );
    assert_eq!(
      VirtualMachineState::from(ffi::VZVirtualMachineState::Unknown(7)),
      VirtualMachineState::Unknown(7)
    );
  }

  /// Its initializers take a configuration, which can't exist here.
  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtualMachine::is_supported(), Err(Error::Unsupported)));
  }
}
