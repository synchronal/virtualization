use dispatch2::{DispatchQueue, DispatchRetained};

use crate::foundation;
use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{DescriptorType, DeviceRequest, DeviceRequestRecipientValue, DeviceRequestTypeValue, Error, NSMutableData};

/// [`IOUSBHostObject`](https://developer.apple.com/documentation/iousbhost/iousbhostobject)
pub trait Object: Sealed {
  /// [`destroy()`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/destroy())
  fn destroy(&self) {
    ffi::iousbhost_object_destroy(self.bridge_object());
  }

  /// [`destroy(options:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/destroy(options:))
  fn destroy_with_options(&self, options: ObjectDestroyOptions) {
    ffi::iousbhost_object_destroy_options(self.bridge_object(), options.raw_value);
  }

  /// [`ioService`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/ioservice):
  /// an `io_service_t`.
  fn io_service(&self) -> u32 {
    ffi::iousbhost_object_io_service(self.bridge_object())
  }

  /// [`queue`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/queue)
  fn queue(&self) -> DispatchRetained<DispatchQueue> {
    ffi::iousbhost_object_queue(self.bridge_object())
  }

  /// [`deviceDescriptor`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/devicedescriptor):
  /// its `bLength` bytes.
  fn device_descriptor(&self) -> Option<Vec<u8>> {
    ffi::iousbhost_object_device_descriptor(self.bridge_object())
  }

  /// [`capabilityDescriptors`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/capabilitydescriptors):
  /// the BOS descriptor's `wTotalLength` bytes, its device capability
  /// descriptors with it.
  fn capability_descriptors(&self) -> Option<Vec<u8>> {
    ffi::iousbhost_object_capability_descriptors(self.bridge_object())
  }

  /// [`configurationDescriptor(with:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/configurationdescriptor(with:)):
  /// its `wTotalLength` bytes, its interface and endpoint descriptors with
  /// it.
  fn configuration_descriptor_with(&self, index: isize) -> Result<Vec<u8>, Error> {
    ffi::iousbhost_object_configuration_descriptor_with(self.bridge_object(), index).map_err(foundation::thrown)
  }

  /// [`configurationDescriptor(withConfigurationValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/configurationdescriptor(withconfigurationvalue:)):
  /// its `wTotalLength` bytes, its interface and endpoint descriptors with
  /// it.
  fn configuration_descriptor_with_configuration_value(&self, configuration_value: isize) -> Result<Vec<u8>, Error> {
    ffi::iousbhost_object_configuration_descriptor_with_configuration_value(self.bridge_object(), configuration_value)
      .map_err(foundation::thrown)
  }

  /// [`descriptorWithType:length:index:languageID:requestType:requestRecipient:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/descriptorwithtype:length:index:languageid:requesttype:requestrecipient:error:):
  /// its `length` bytes. `descriptor_type` is `type`.
  fn descriptor_with_type(
    &self,
    descriptor_type: DescriptorType,
    index: isize,
    language_id: isize,
    request_type: DeviceRequestTypeValue,
    request_recipient: DeviceRequestRecipientValue,
  ) -> Result<Vec<u8>, Error> {
    ffi::iousbhost_object_descriptor_with_type(
      self.bridge_object(),
      descriptor_type.into(),
      index,
      language_id,
      request_type.into(),
      request_recipient.into(),
    )
    .map_err(foundation::thrown)
  }

  /// [`stringWithIndex:languageID:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/stringwithindex:languageid:error:)
  fn string_with_index(&self, index: isize, language_id: isize) -> Result<String, Error> {
    ffi::iousbhost_object_string_with_index(self.bridge_object(), index, language_id).map_err(foundation::thrown)
  }

  /// [`ioData(withCapacity:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/iodata(withcapacity:))
  fn io_data_with_capacity(&self, capacity: isize) -> Result<NSMutableData, Error> {
    ffi::iousbhost_object_io_data_with_capacity(self.bridge_object(), capacity)
      .map(foundation::mutable_data)
      .map_err(foundation::thrown)
  }

  /// [`data(withCapacity:options:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/data(withcapacity:options:))
  fn data_with_capacity(&self, capacity: isize, options: ObjectDataOptions) -> Result<NSMutableData, Error> {
    ffi::iousbhost_object_data_with_capacity_options(self.bridge_object(), capacity, options.raw_value)
      .map(foundation::mutable_data)
      .map_err(foundation::thrown)
  }

  /// [`sendDeviceRequest:data:bytesTransferred:completionTimeout:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/senddevicerequest:data:bytestransferred:completiontimeout:error:):
  /// `bytesTransferred`. `data` `None` is `nil`.
  fn send_device_request(
    &self,
    request: DeviceRequest,
    data: Option<&NSMutableData>,
    completion_timeout: f64,
  ) -> Result<isize, Error> {
    ffi::iousbhost_object_send_device_request(
      self.bridge_object(),
      request.into(),
      data.map(foundation::ns_mutable_data),
      completion_timeout,
    )
    .map_err(foundation::thrown)
  }

  /// [`enqueueDeviceRequest:data:completionTimeout:error:completionHandler:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/enqueuedevicerequest:data:completiontimeout:error:completionhandler:).
  /// `data` and `completion_handler` `None` are `nil`.
  fn enqueue_device_request(
    &self,
    request: DeviceRequest,
    data: Option<&NSMutableData>,
    completion_timeout: f64,
    completion_handler: Option<CompletionHandler>,
  ) -> Result<(), Error> {
    ffi::iousbhost_object_enqueue_device_request(
      self.bridge_object(),
      request.into(),
      data.map(foundation::ns_mutable_data),
      completion_timeout,
      completion_handler,
    )
    .map_err(foundation::thrown)
  }

  /// [`abortDeviceRequestsWithOption:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/abortdevicerequestswithoption:error:)
  fn abort_device_requests_with_option(&self, option: AbortOption) -> Result<(), Error> {
    ffi::iousbhost_object_abort_device_requests_with_option(self.bridge_object(), option.into())
      .map_err(foundation::thrown)
  }

  /// [`deviceAddress`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/deviceaddress)
  fn device_address(&self) -> isize {
    ffi::iousbhost_object_device_address(self.bridge_object())
  }

  /// [`frameNumberWithTime:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/framenumberwithtime:):
  /// the frame number and `time`, an `IOUSBHostTime`.
  fn frame_number_with_time(&self) -> (u64, u64) {
    ffi::iousbhost_object_frame_number_with_time(self.bridge_object())
  }

  /// [`currentMicroframeWithTime:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/currentmicroframewithtime:error:):
  /// the microframe number and `time`, an `IOUSBHostTime`.
  fn current_microframe_with_time(&self) -> Result<(u64, u64), Error> {
    ffi::iousbhost_object_current_microframe_with_time(self.bridge_object()).map_err(foundation::thrown)
  }

  /// [`referenceMicroframeWithTime:error:`](https://developer.apple.com/documentation/iousbhost/iousbhostobject/referencemicroframewithtime:error:):
  /// the microframe number and `time`, an `IOUSBHostTime`.
  fn reference_microframe_with_time(&self) -> Result<(u64, u64), Error> {
    ffi::iousbhost_object_reference_microframe_with_time(self.bridge_object()).map_err(foundation::thrown)
  }
}

/// [`IOUSBHostCompletionHandler`](https://developer.apple.com/documentation/iousbhost/iousbhostcompletionhandler):
/// called with an `IOReturn` and the bytes transferred.
pub type CompletionHandler = Box<dyn FnOnce(i32, isize) + Send + 'static>;

/// [`IOUSBHostObjectInitOptions`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectinitoptions):
/// an option set, which is its raw value. `IOUSBHostObjectInitOptionsNone` is
/// `new(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectInitOptions {
  /// [`rawValue`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectinitoptions/rawvalue)
  pub raw_value: usize,
}

impl ObjectInitOptions {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectinitoptions/init(rawvalue:))
  pub fn new(raw_value: usize) -> Self {
    Self { raw_value }
  }

  /// [`deviceCapture`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectinitoptions/devicecapture)
  pub fn device_capture() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_object_init_options_device_capture()))
  }

  /// [`deviceSeize`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectinitoptions/deviceseize)
  pub fn device_seize() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_object_init_options_device_seize()))
  }
}

/// [`IOUSBHostObjectDestroyOptions`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdestroyoptions):
/// an option set, which is its raw value. Swift's `[]` is `new(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectDestroyOptions {
  /// [`rawValue`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdestroyoptions/rawvalue)
  pub raw_value: usize,
}

impl ObjectDestroyOptions {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdestroyoptions/init(rawvalue:))
  pub fn new(raw_value: usize) -> Self {
    Self { raw_value }
  }

  /// [`deviceSurrender`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdestroyoptions/devicesurrender)
  pub fn device_surrender() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_object_destroy_options_device_surrender()))
  }
}

/// [`IOUSBHostObjectDataOptions`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdataoptions):
/// an option set, which is its raw value. Swift's `[]` is `new(0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectDataOptions {
  /// [`rawValue`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdataoptions/rawvalue)
  pub raw_value: usize,
}

impl ObjectDataOptions {
  /// [`init(rawValue:)`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdataoptions/init(rawvalue:))
  pub fn new(raw_value: usize) -> Self {
    Self { raw_value }
  }

  /// [`kernelUserShared`](https://developer.apple.com/documentation/iousbhost/iousbhostobjectdataoptions/kernelusershared)
  pub fn kernel_user_shared() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self::new(ffi::iousbhost_object_data_options_kernel_user_shared()))
  }
}

/// [`IOUSBHostAbortOption`](https://developer.apple.com/documentation/iousbhost/iousbhostabortoption)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AbortOption {
  /// [`asynchronous`](https://developer.apple.com/documentation/iousbhost/iousbhostabortoption/asynchronous)
  Asynchronous,
  /// [`synchronous`](https://developer.apple.com/documentation/iousbhost/iousbhostabortoption/synchronous)
  Synchronous,
  /// A case newer than this crate, by its raw value.
  Unknown(usize),
}

impl From<ffi::IOUSBHostAbortOption> for AbortOption {
  fn from(option: ffi::IOUSBHostAbortOption) -> Self {
    match option {
      ffi::IOUSBHostAbortOption::Asynchronous => Self::Asynchronous,
      ffi::IOUSBHostAbortOption::Synchronous => Self::Synchronous,
      ffi::IOUSBHostAbortOption::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

impl From<AbortOption> for ffi::IOUSBHostAbortOption {
  fn from(option: AbortOption) -> Self {
    match option {
      AbortOption::Asynchronous => Self::Asynchronous,
      AbortOption::Synchronous => Self::Synchronous,
      AbortOption::Unknown(raw_value) => Self::Unknown(raw_value),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maps_abort_options_both_ways() {
    for option in [
      AbortOption::Asynchronous,
      AbortOption::Synchronous,
      AbortOption::Unknown(7),
    ] {
      assert_eq!(AbortOption::from(ffi::IOUSBHostAbortOption::from(option)), option);
    }
  }

  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    assert!(matches!(ObjectInitOptions::device_capture(), Err(Error::Unsupported)));
    assert!(matches!(ObjectInitOptions::device_seize(), Err(Error::Unsupported)));
    assert!(matches!(
      ObjectDestroyOptions::device_surrender(),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      ObjectDataOptions::kernel_user_shared(),
      Err(Error::Unsupported)
    ));
  }
}
