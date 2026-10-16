//===----------------------------------------------------------------------===//
// `VZVirtualMachineConfiguration`.
//===----------------------------------------------------------------------===//

// Function names are Rust's, which are snake_case.
// swift-format-ignore-file: AlwaysUseLowerCamelCase

import Virtualization

func vz_virtual_machine_configuration_init() -> VZVirtualMachineConfiguration {
  VZVirtualMachineConfiguration()
}

func vz_virtual_machine_configuration_minimum_allowed_memory_size() -> UInt64 {
  VZVirtualMachineConfiguration.minimumAllowedMemorySize
}

func vz_virtual_machine_configuration_maximum_allowed_memory_size() -> UInt64 {
  VZVirtualMachineConfiguration.maximumAllowedMemorySize
}

func vz_virtual_machine_configuration_minimum_allowed_cpu_count() -> Int {
  VZVirtualMachineConfiguration.minimumAllowedCPUCount
}

func vz_virtual_machine_configuration_maximum_allowed_cpu_count() -> Int {
  VZVirtualMachineConfiguration.maximumAllowedCPUCount
}

func vz_virtual_machine_configuration_boot_loader(this: VZVirtualMachineConfiguration) -> UInt {
  bridging(retainedAddress: this.bootLoader)
}

/// `boot_loader` is any `VZBootLoader`, or `0` for `nil`.
func vz_virtual_machine_configuration_set_boot_loader(this: VZVirtualMachineConfiguration, boot_loader: UInt) {
  this.bootLoader = bridging(borrowedAddress: boot_loader) as VZBootLoader?
}

func vz_virtual_machine_configuration_platform(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.platform)
}

/// `platform` is any `VZPlatformConfiguration`.
func vz_virtual_machine_configuration_set_platform(this: VZVirtualMachineConfiguration, platform: UnsafeRawPointer) {
  this.platform = bridging(borrowed: platform)
}

func vz_virtual_machine_configuration_audio_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.audioDevices as NSArray)
}

/// `audio_devices` is an array of any `VZAudioDeviceConfiguration`.
func vz_virtual_machine_configuration_set_audio_devices(
  this: VZVirtualMachineConfiguration,
  audio_devices: UnsafeRawPointer
) {
  this.audioDevices = bridging(borrowedArray: audio_devices)
}

func vz_virtual_machine_configuration_console_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.consoleDevices as NSArray)
}

/// `console_devices` is an array of any `VZConsoleDeviceConfiguration`.
func vz_virtual_machine_configuration_set_console_devices(
  this: VZVirtualMachineConfiguration,
  console_devices: UnsafeRawPointer
) {
  this.consoleDevices = bridging(borrowedArray: console_devices)
}

func vz_virtual_machine_configuration_directory_sharing_devices(this: VZVirtualMachineConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.directorySharingDevices as NSArray)
}

/// `directory_sharing_devices` is an array of any
/// `VZDirectorySharingDeviceConfiguration`.
func vz_virtual_machine_configuration_set_directory_sharing_devices(
  this: VZVirtualMachineConfiguration,
  directory_sharing_devices: UnsafeRawPointer
) {
  this.directorySharingDevices = bridging(borrowedArray: directory_sharing_devices)
}

func vz_virtual_machine_configuration_entropy_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.entropyDevices as NSArray)
}

/// `entropy_devices` is an array of any `VZEntropyDeviceConfiguration`.
func vz_virtual_machine_configuration_set_entropy_devices(
  this: VZVirtualMachineConfiguration,
  entropy_devices: UnsafeRawPointer
) {
  this.entropyDevices = bridging(borrowedArray: entropy_devices)
}

func vz_virtual_machine_configuration_memory_balloon_devices(this: VZVirtualMachineConfiguration)
  -> UnsafeMutableRawPointer
{
  bridging(retained: this.memoryBalloonDevices as NSArray)
}

/// `memory_balloon_devices` is an array of any
/// `VZMemoryBalloonDeviceConfiguration`.
func vz_virtual_machine_configuration_set_memory_balloon_devices(
  this: VZVirtualMachineConfiguration,
  memory_balloon_devices: UnsafeRawPointer
) {
  this.memoryBalloonDevices = bridging(borrowedArray: memory_balloon_devices)
}

func vz_virtual_machine_configuration_network_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.networkDevices as NSArray)
}

/// `network_devices` is an array of any `VZNetworkDeviceConfiguration`.
func vz_virtual_machine_configuration_set_network_devices(
  this: VZVirtualMachineConfiguration,
  network_devices: UnsafeRawPointer
) {
  this.networkDevices = bridging(borrowedArray: network_devices)
}

func vz_virtual_machine_configuration_serial_ports(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.serialPorts as NSArray)
}

/// `serial_ports` is an array of any `VZSerialPortConfiguration`.
func vz_virtual_machine_configuration_set_serial_ports(
  this: VZVirtualMachineConfiguration,
  serial_ports: UnsafeRawPointer
) {
  this.serialPorts = bridging(borrowedArray: serial_ports)
}

func vz_virtual_machine_configuration_socket_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.socketDevices as NSArray)
}

/// `socket_devices` is an array of any `VZSocketDeviceConfiguration`.
func vz_virtual_machine_configuration_set_socket_devices(
  this: VZVirtualMachineConfiguration,
  socket_devices: UnsafeRawPointer
) {
  this.socketDevices = bridging(borrowedArray: socket_devices)
}

func vz_virtual_machine_configuration_storage_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.storageDevices as NSArray)
}

/// `storage_devices` is an array of any `VZStorageDeviceConfiguration`.
func vz_virtual_machine_configuration_set_storage_devices(
  this: VZVirtualMachineConfiguration,
  storage_devices: UnsafeRawPointer
) {
  this.storageDevices = bridging(borrowedArray: storage_devices)
}

func vz_virtual_machine_configuration_keyboards(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.keyboards as NSArray)
}

/// `keyboards` is an array of any `VZKeyboardConfiguration`.
func vz_virtual_machine_configuration_set_keyboards(this: VZVirtualMachineConfiguration, keyboards: UnsafeRawPointer) {
  this.keyboards = bridging(borrowedArray: keyboards)
}

func vz_virtual_machine_configuration_pointing_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.pointingDevices as NSArray)
}

/// `pointing_devices` is an array of any `VZPointingDeviceConfiguration`.
func vz_virtual_machine_configuration_set_pointing_devices(
  this: VZVirtualMachineConfiguration,
  pointing_devices: UnsafeRawPointer
) {
  this.pointingDevices = bridging(borrowedArray: pointing_devices)
}

func vz_virtual_machine_configuration_graphics_devices(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.graphicsDevices as NSArray)
}

/// `graphics_devices` is an array of any `VZGraphicsDeviceConfiguration`.
func vz_virtual_machine_configuration_set_graphics_devices(
  this: VZVirtualMachineConfiguration,
  graphics_devices: UnsafeRawPointer
) {
  this.graphicsDevices = bridging(borrowedArray: graphics_devices)
}

func vz_virtual_machine_configuration_usb_controllers(this: VZVirtualMachineConfiguration) -> UnsafeMutableRawPointer {
  bridging(retained: this.usbControllers as NSArray)
}

/// `usb_controllers` is an array of any `VZUSBControllerConfiguration`.
func vz_virtual_machine_configuration_set_usb_controllers(
  this: VZVirtualMachineConfiguration,
  usb_controllers: UnsafeRawPointer
) {
  this.usbControllers = bridging(borrowedArray: usb_controllers)
}

extension VZVirtualMachineConfiguration {
  func vz_memory_size() -> UInt64 {
    memorySize
  }

  func vz_set_memory_size(memory_size: UInt64) {
    memorySize = memory_size
  }

  func vz_cpu_count() -> Int {
    cpuCount
  }

  func vz_set_cpu_count(cpu_count: Int) {
    cpuCount = cpu_count
  }

  func vz_label() -> RustString? {
    label?.intoRustString()
  }

  func vz_set_label(label: RustStr?) {
    self.label = label?.toString()
  }

  func vz_validate() throws(BridgedThrown) {
    try bridging {
      try validate()
    }
  }

  func vz_validate_save_restore_support() throws(BridgedThrown) {
    try bridging {
      try validateSaveRestoreSupport()
    }
  }
}
