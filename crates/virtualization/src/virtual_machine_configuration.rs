use std::fmt;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{
  AudioDeviceConfiguration, BootLoader, ConsoleDeviceConfiguration, DirectorySharingDeviceConfiguration,
  EntropyDeviceConfiguration, Error, GraphicsDeviceConfiguration, KeyboardConfiguration,
  MemoryBalloonDeviceConfiguration, NetworkDeviceConfiguration, PlatformConfiguration, PointingDeviceConfiguration,
  SerialPortConfiguration, SocketDeviceConfiguration, StorageDeviceConfiguration, USBControllerConfiguration,
  foundation,
};

/// [`VZVirtualMachineConfiguration`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration)
///
/// An array of an abstract class is a slice of trait objects going in. Coming
/// out, each element is cast to the subclass given, as Swift's `as? T`:
/// `None` where an element isn't a `T`.
pub struct VirtualMachineConfiguration {
  inner: ffi::VZVirtualMachineConfiguration,
}

/// Each element as its abstract class.
fn bridge_objects<'a, T: Sealed + ?Sized>(elements: &[&'a T]) -> Vec<&'a ffi::NSObject> {
  elements
    .iter()
    .map(|element| element.bridge_object())
    .collect()
}

/// Each element `as? T`.
fn bridge_cast_each<T: Sealed>(elements: Vec<ffi::Retained<ffi::NSObject>>) -> Vec<Option<T>> {
  elements
    .iter()
    .map(|element| T::bridge_cast(element))
    .collect()
}

impl VirtualMachineConfiguration {
  /// [`init()`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/init())
  pub fn new() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtual_machine_configuration_init(),
    })
  }

  /// [`minimumAllowedMemorySize`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/minimumallowedmemorysize)
  pub fn minimum_allowed_memory_size() -> Result<u64, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_configuration_minimum_allowed_memory_size())
  }

  /// [`maximumAllowedMemorySize`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/maximumallowedmemorysize)
  pub fn maximum_allowed_memory_size() -> Result<u64, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_configuration_maximum_allowed_memory_size())
  }

  /// [`minimumAllowedCPUCount`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/minimumallowedcpucount)
  pub fn minimum_allowed_cpu_count() -> Result<isize, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_configuration_minimum_allowed_cpu_count())
  }

  /// [`maximumAllowedCPUCount`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/maximumallowedcpucount)
  pub fn maximum_allowed_cpu_count() -> Result<isize, Error> {
    platform::supported()?;

    Ok(ffi::vz_virtual_machine_configuration_maximum_allowed_cpu_count())
  }

  /// [`bootLoader`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/bootloader),
  /// as Swift's `as? T`: `None` when there is none, or it isn't a `T`.
  pub fn boot_loader<T: BootLoader>(&self) -> Option<T> {
    T::bridge_cast(&*self.inner.vz_boot_loader()?)
  }

  /// [`bootLoader`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/bootloader)
  pub fn set_boot_loader(&self, boot_loader: Option<&dyn BootLoader>) {
    self
      .inner
      .vz_set_boot_loader(boot_loader.map(|boot_loader| boot_loader.bridge_object()));
  }

  /// [`memorySize`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/memorysize)
  pub fn memory_size(&self) -> u64 {
    self.inner.vz_memory_size()
  }

  /// [`memorySize`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/memorysize)
  pub fn set_memory_size(&self, memory_size: u64) {
    self.inner.vz_set_memory_size(memory_size);
  }

  /// [`cpuCount`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/cpucount)
  pub fn cpu_count(&self) -> isize {
    self.inner.vz_cpu_count()
  }

  /// [`cpuCount`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/cpucount)
  pub fn set_cpu_count(&self, cpu_count: isize) {
    self.inner.vz_set_cpu_count(cpu_count);
  }

  /// [`platform`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/platform),
  /// as Swift's `as? T`: `None` when it isn't a `T`.
  pub fn platform<T: PlatformConfiguration>(&self) -> Option<T> {
    T::bridge_cast(&self.inner.vz_platform())
  }

  /// [`platform`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/platform)
  pub fn set_platform(&self, platform: &dyn PlatformConfiguration) {
    self.inner.vz_set_platform(platform.bridge_object());
  }

  /// [`audioDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/audiodevices)
  pub fn audio_devices<T: AudioDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_audio_devices())
  }

  /// [`audioDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/audiodevices)
  pub fn set_audio_devices(&self, audio_devices: &[&dyn AudioDeviceConfiguration]) {
    self
      .inner
      .vz_set_audio_devices(&bridge_objects(audio_devices));
  }

  /// [`consoleDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/consoledevices)
  pub fn console_devices<T: ConsoleDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_console_devices())
  }

  /// [`consoleDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/consoledevices)
  pub fn set_console_devices(&self, console_devices: &[&dyn ConsoleDeviceConfiguration]) {
    self
      .inner
      .vz_set_console_devices(&bridge_objects(console_devices));
  }

  /// [`directorySharingDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/directorysharingdevices)
  pub fn directory_sharing_devices<T: DirectorySharingDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_directory_sharing_devices())
  }

  /// [`directorySharingDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/directorysharingdevices)
  pub fn set_directory_sharing_devices(&self, directory_sharing_devices: &[&dyn DirectorySharingDeviceConfiguration]) {
    self
      .inner
      .vz_set_directory_sharing_devices(&bridge_objects(directory_sharing_devices));
  }

  /// [`entropyDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/entropydevices)
  pub fn entropy_devices<T: EntropyDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_entropy_devices())
  }

  /// [`entropyDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/entropydevices)
  pub fn set_entropy_devices(&self, entropy_devices: &[&dyn EntropyDeviceConfiguration]) {
    self
      .inner
      .vz_set_entropy_devices(&bridge_objects(entropy_devices));
  }

  /// [`memoryBalloonDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/memoryballoondevices)
  pub fn memory_balloon_devices<T: MemoryBalloonDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_memory_balloon_devices())
  }

  /// [`memoryBalloonDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/memoryballoondevices)
  pub fn set_memory_balloon_devices(&self, memory_balloon_devices: &[&dyn MemoryBalloonDeviceConfiguration]) {
    self
      .inner
      .vz_set_memory_balloon_devices(&bridge_objects(memory_balloon_devices));
  }

  /// [`networkDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/networkdevices)
  pub fn network_devices<T: NetworkDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_network_devices())
  }

  /// [`networkDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/networkdevices)
  pub fn set_network_devices(&self, network_devices: &[&dyn NetworkDeviceConfiguration]) {
    self
      .inner
      .vz_set_network_devices(&bridge_objects(network_devices));
  }

  /// [`serialPorts`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/serialports)
  pub fn serial_ports<T: SerialPortConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_serial_ports())
  }

  /// [`serialPorts`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/serialports)
  pub fn set_serial_ports(&self, serial_ports: &[&dyn SerialPortConfiguration]) {
    self
      .inner
      .vz_set_serial_ports(&bridge_objects(serial_ports));
  }

  /// [`socketDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/socketdevices)
  pub fn socket_devices<T: SocketDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_socket_devices())
  }

  /// [`socketDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/socketdevices)
  pub fn set_socket_devices(&self, socket_devices: &[&dyn SocketDeviceConfiguration]) {
    self
      .inner
      .vz_set_socket_devices(&bridge_objects(socket_devices));
  }

  /// [`storageDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/storagedevices)
  pub fn storage_devices<T: StorageDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_storage_devices())
  }

  /// [`storageDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/storagedevices)
  pub fn set_storage_devices(&self, storage_devices: &[&dyn StorageDeviceConfiguration]) {
    self
      .inner
      .vz_set_storage_devices(&bridge_objects(storage_devices));
  }

  /// [`keyboards`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/keyboards)
  pub fn keyboards<T: KeyboardConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_keyboards())
  }

  /// [`keyboards`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/keyboards)
  pub fn set_keyboards(&self, keyboards: &[&dyn KeyboardConfiguration]) {
    self.inner.vz_set_keyboards(&bridge_objects(keyboards));
  }

  /// [`pointingDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/pointingdevices)
  pub fn pointing_devices<T: PointingDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_pointing_devices())
  }

  /// [`pointingDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/pointingdevices)
  pub fn set_pointing_devices(&self, pointing_devices: &[&dyn PointingDeviceConfiguration]) {
    self
      .inner
      .vz_set_pointing_devices(&bridge_objects(pointing_devices));
  }

  /// [`graphicsDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/graphicsdevices)
  pub fn graphics_devices<T: GraphicsDeviceConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_graphics_devices())
  }

  /// [`graphicsDevices`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/graphicsdevices)
  pub fn set_graphics_devices(&self, graphics_devices: &[&dyn GraphicsDeviceConfiguration]) {
    self
      .inner
      .vz_set_graphics_devices(&bridge_objects(graphics_devices));
  }

  /// [`usbControllers`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/usbcontrollers)
  pub fn usb_controllers<T: USBControllerConfiguration>(&self) -> Vec<Option<T>> {
    bridge_cast_each(self.inner.vz_usb_controllers())
  }

  /// [`usbControllers`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/usbcontrollers)
  pub fn set_usb_controllers(&self, usb_controllers: &[&dyn USBControllerConfiguration]) {
    self
      .inner
      .vz_set_usb_controllers(&bridge_objects(usb_controllers));
  }

  /// [`label`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/label)
  pub fn label(&self) -> Option<String> {
    self.inner.vz_label()
  }

  /// [`label`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/label)
  pub fn set_label(&self, label: Option<&str>) {
    self.inner.vz_set_label(label);
  }

  /// [`validate()`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/validate())
  pub fn validate(&self) -> Result<(), Error> {
    self.inner.vz_validate().map_err(foundation::thrown)
  }

  /// [`validateSaveRestoreSupport()`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineconfiguration/validatesaverestoresupport())
  pub fn validate_save_restore_support(&self) -> Result<(), Error> {
    self
      .inner
      .vz_validate_save_restore_support()
      .map_err(foundation::thrown)
  }
}

impl Clone for VirtualMachineConfiguration {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtualMachineConfiguration {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(VirtualMachineConfiguration::new(), Err(Error::Unsupported)));
    assert!(matches!(
      VirtualMachineConfiguration::minimum_allowed_memory_size(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      VirtualMachineConfiguration::maximum_allowed_memory_size(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      VirtualMachineConfiguration::minimum_allowed_cpu_count(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      VirtualMachineConfiguration::maximum_allowed_cpu_count(),
      Err(Error::Unsupported)
    ));
  }
}
