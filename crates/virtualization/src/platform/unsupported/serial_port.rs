use std::os::fd::{BorrowedFd, OwnedFd};

use super::{BridgedThrown, GUARDED, NSObject, NSURL, Never, Retained};

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
