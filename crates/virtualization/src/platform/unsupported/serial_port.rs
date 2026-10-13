use std::any::Any;
use std::os::fd::{BorrowedFd, OwnedFd};
use std::sync::Arc;

use super::{BridgedNil, BridgedThrown, GUARDED, NSError, NSObject, NSURL, Never, Retained};

pub(crate) fn vz_console_port_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_console_port_configuration_set_attachment(this: &NSObject, _attachment: Option<&NSObject>) {
  this.never()
}

pub(crate) struct VZFileHandleSerialPortAttachment {
  never: Never,
}

pub(crate) fn vz_file_handle_serial_port_attachment_init(
  _file_handle_for_reading: Option<OwnedFd>,
  _file_handle_for_writing: Option<OwnedFd>,
) -> VZFileHandleSerialPortAttachment {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_file_handle_serial_port_attachment_bridge_cast(
  object: &NSObject,
) -> Option<VZFileHandleSerialPortAttachment> {
  object.never()
}

impl VZFileHandleSerialPortAttachment {
  pub(crate) fn bridge_retained(&self) -> VZFileHandleSerialPortAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_file_handle_for_reading(&self) -> Option<BorrowedFd<'_>> {
    match self.never {}
  }

  pub(crate) fn vz_file_handle_for_writing(&self) -> Option<BorrowedFd<'_>> {
    match self.never {}
  }
}

pub(crate) struct VZFileSerialPortAttachment {
  never: Never,
}

pub(crate) fn vz_file_serial_port_attachment_init(
  _url: &NSURL,
  _append: bool,
) -> Result<VZFileSerialPortAttachment, BridgedThrown> {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_file_serial_port_attachment_bridge_cast(object: &NSObject) -> Option<VZFileSerialPortAttachment> {
  object.never()
}

impl VZFileSerialPortAttachment {
  pub(crate) fn bridge_retained(&self) -> VZFileSerialPortAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_url(&self) -> Retained<NSURL> {
    match self.never {}
  }

  pub(crate) fn vz_append(&self) -> bool {
    match self.never {}
  }
}

pub(crate) fn vz_serial_port_configuration_attachment(this: &NSObject) -> Option<Retained<NSObject>> {
  this.never()
}

pub(crate) fn vz_serial_port_configuration_set_attachment(this: &NSObject, _attachment: Option<&NSObject>) {
  this.never()
}

pub(crate) struct VZSpiceAgentPortAttachment {
  never: Never,
}

pub(crate) fn vz_spice_agent_port_attachment_init() -> VZSpiceAgentPortAttachment {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_spice_agent_port_attachment_bridge_cast(object: &NSObject) -> Option<VZSpiceAgentPortAttachment> {
  object.never()
}

pub(crate) fn vz_spice_agent_port_attachment_spice_agent_port_name() -> String {
  unreachable!("{GUARDED}")
}

impl VZSpiceAgentPortAttachment {
  pub(crate) fn bridge_retained(&self) -> VZSpiceAgentPortAttachment {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_shares_clipboard(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_shares_clipboard(&self, _shares_clipboard: bool) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsoleDevice {
  never: Never,
}

pub(crate) fn vz_virtio_console_device_bridge_cast(object: &NSObject) -> Option<VZVirtioConsoleDevice> {
  object.never()
}

impl VZVirtioConsoleDevice {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsoleDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_delegate(&self) -> Option<Arc<dyn VZVirtioConsoleDeviceDelegate>> {
    match self.never {}
  }

  pub(crate) fn vz_set_delegate(&self, _delegate: Option<Arc<dyn VZVirtioConsoleDeviceDelegate>>) {
    match self.never {}
  }

  pub(crate) fn vz_ports(&self) -> VZVirtioConsolePortArray {
    match self.never {}
  }
}

pub(crate) trait VZVirtioConsoleDeviceDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZVirtioConsoleDeviceDelegate for T {}

pub(crate) struct VZVirtioConsoleDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_console_device_configuration_init() -> VZVirtioConsoleDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_console_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioConsoleDeviceConfiguration> {
  object.never()
}

impl VZVirtioConsoleDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsoleDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_ports(&self) -> VZVirtioConsolePortConfigurationArray {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsoleDeviceSerialPortConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_console_device_serial_port_configuration_init() -> VZVirtioConsoleDeviceSerialPortConfiguration
{
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_console_device_serial_port_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioConsoleDeviceSerialPortConfiguration> {
  object.never()
}

impl VZVirtioConsoleDeviceSerialPortConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsoleDeviceSerialPortConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsolePort {
  never: Never,
}

impl VZVirtioConsolePort {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsolePort {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_name(&self) -> Option<String> {
    match self.never {}
  }

  pub(crate) fn vz_attachment(&self) -> Option<Retained<NSObject>> {
    match self.never {}
  }

  pub(crate) fn vz_set_attachment(&self, _attachment: Option<&NSObject>) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsolePortArray {
  never: Never,
}

impl VZVirtioConsolePortArray {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsolePortArray {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_maximum_port_count(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_subscript(&self, _port_index: isize) -> Result<VZVirtioConsolePort, BridgedNil> {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsolePortConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_console_port_configuration_init() -> VZVirtioConsolePortConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_console_port_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioConsolePortConfiguration> {
  object.never()
}

impl VZVirtioConsolePortConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsolePortConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_name(&self) -> Option<String> {
    match self.never {}
  }

  pub(crate) fn vz_set_name(&self, _name: Option<&str>) {
    match self.never {}
  }

  pub(crate) fn vz_is_console(&self) -> bool {
    match self.never {}
  }

  pub(crate) fn vz_set_is_console(&self, _is_console: bool) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioConsolePortConfigurationArray {
  never: Never,
}

impl VZVirtioConsolePortConfigurationArray {
  pub(crate) fn bridge_retained(&self) -> VZVirtioConsolePortConfigurationArray {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_maximum_port_count(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_set_maximum_port_count(&self, _maximum_port_count: u32) {
    match self.never {}
  }

  pub(crate) fn vz_subscript(&self, _port_index: isize) -> Result<VZVirtioConsolePortConfiguration, BridgedNil> {
    match self.never {}
  }

  pub(crate) fn vz_set_subscript(&self, _port_index: isize, _value: VZVirtioConsolePortConfiguration) {
    match self.never {}
  }

  pub(crate) fn bridge_set_subscript_nil(&self, _port_index: isize) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSocketConnection {
  never: Never,
}

impl VZVirtioSocketConnection {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSocketConnection {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_destination_port(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_source_port(&self) -> u32 {
    match self.never {}
  }

  pub(crate) fn vz_file_descriptor(&self) -> i32 {
    match self.never {}
  }

  pub(crate) fn vz_close(&self) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSocketDevice {
  never: Never,
}

pub(crate) fn vz_virtio_socket_device_bridge_cast(object: &NSObject) -> Option<VZVirtioSocketDevice> {
  object.never()
}

impl VZVirtioSocketDevice {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSocketDevice {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_set_socket_listener(&self, _listener: VZVirtioSocketListener, _port: u32) {
    match self.never {}
  }

  pub(crate) fn vz_remove_socket_listener(&self, _port: u32) {
    match self.never {}
  }

  pub(crate) fn vz_connect(
    &self,
    _port: u32,
    _completion_handler: impl FnOnce(Result<VZVirtioSocketConnection, Retained<NSError>>) + Send + 'static,
  ) {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSocketDeviceConfiguration {
  never: Never,
}

pub(crate) fn vz_virtio_socket_device_configuration_init() -> VZVirtioSocketDeviceConfiguration {
  unreachable!("{GUARDED}")
}

pub(crate) fn vz_virtio_socket_device_configuration_bridge_cast(
  object: &NSObject,
) -> Option<VZVirtioSocketDeviceConfiguration> {
  object.never()
}

impl VZVirtioSocketDeviceConfiguration {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSocketDeviceConfiguration {
    match self.never {}
  }

  pub(crate) fn bridge_object(&self) -> &NSObject {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}

pub(crate) struct VZVirtioSocketListener {
  never: Never,
}

pub(crate) fn vz_virtio_socket_listener_init() -> VZVirtioSocketListener {
  unreachable!("{GUARDED}")
}

impl VZVirtioSocketListener {
  pub(crate) fn bridge_retained(&self) -> VZVirtioSocketListener {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }

  pub(crate) fn vz_delegate(&self) -> Option<Arc<dyn VZVirtioSocketListenerDelegate>> {
    match self.never {}
  }

  pub(crate) fn vz_set_delegate(&self, _delegate: Option<Arc<dyn VZVirtioSocketListenerDelegate>>) {
    match self.never {}
  }
}

pub(crate) trait VZVirtioSocketListenerDelegate: Any + Send + Sync {}

impl<T: Any + Send + Sync> VZVirtioSocketListenerDelegate for T {}
