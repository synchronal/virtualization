use std::any::Any;
use std::fmt;
use std::sync::{Arc, Weak};

use url::Url;

use crate::platform::{self, ffi};
use crate::sealed::Sealed;
use crate::{DiskSynchronizationMode, Error, StorageDeviceAttachment, foundation};

/// [`VZNetworkBlockDeviceStorageDeviceAttachment`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment)
pub struct NetworkBlockDeviceStorageDeviceAttachment {
  inner: ffi::VZNetworkBlockDeviceStorageDeviceAttachment,
}

impl NetworkBlockDeviceStorageDeviceAttachment {
  /// [`init(url:)`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/init(url:)-6q4m7)
  pub fn new(url: &Url) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_network_block_device_storage_device_attachment_init(&*foundation::ns_url(url)?)
        .map_err(foundation::thrown)?,
    })
  }

  /// [`init(url:timeout:isForcedReadOnly:synchronizationMode:)`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/init(url:timeout:isforcedreadonly:synchronizationmode:)).
  /// `TimeInterval` is its seconds.
  pub fn with_timeout_and_is_forced_read_only_and_synchronization_mode(
    url: &Url,
    timeout: f64,
    is_forced_read_only: bool,
    synchronization_mode: DiskSynchronizationMode,
  ) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner:
        ffi::vz_network_block_device_storage_device_attachment_init_timeout_is_forced_read_only_synchronization_mode(
          &*foundation::ns_url(url)?,
          timeout,
          is_forced_read_only,
          synchronization_mode.into(),
        )
        .map_err(foundation::thrown)?,
    })
  }

  /// [`validate(_:)`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/validate(_:))
  pub fn validate(url: &Url) -> Result<(), Error> {
    platform::supported()?;

    ffi::vz_network_block_device_storage_device_attachment_validate(&*foundation::ns_url(url)?)
      .map_err(foundation::thrown)
  }

  /// [`url`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/url)
  pub fn url(&self) -> Result<Url, Error> {
    foundation::url(&self.inner.vz_url())
  }

  /// [`timeout`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/timeout).
  /// `TimeInterval` is its seconds.
  pub fn timeout(&self) -> f64 {
    self.inner.vz_timeout()
  }

  /// [`isForcedReadOnly`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/isforcedreadonly)
  pub fn is_forced_read_only(&self) -> bool {
    self.inner.vz_is_forced_read_only()
  }

  /// [`synchronizationMode`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/synchronizationmode)
  pub fn synchronization_mode(&self) -> DiskSynchronizationMode {
    self.inner.vz_synchronization_mode().into()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/delegate):
  /// `None` when there is none, or it has been dropped.
  pub fn delegate(&self) -> Option<Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate>> {
    let delegating: Arc<dyn Any + Send + Sync> = self.inner.vz_delegate()?;

    delegating.downcast::<Delegating>().ok()?.0.upgrade()
  }

  /// [`delegate`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachment/delegate),
  /// held weakly, as in Swift.
  pub fn set_delegate(&self, delegate: Option<&Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate>>) {
    self.inner.vz_set_delegate(delegate.map(|delegate| {
      Arc::new(Delegating(Arc::downgrade(delegate)))
        as Arc<dyn ffi::VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>
    }));
  }
}

impl Clone for NetworkBlockDeviceStorageDeviceAttachment {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for NetworkBlockDeviceStorageDeviceAttachment {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

impl Sealed for NetworkBlockDeviceStorageDeviceAttachment {
  fn bridge_object(&self) -> &ffi::NSObject {
    self.inner.bridge_object()
  }

  fn bridge_cast(object: &ffi::NSObject) -> Option<Self> {
    ffi::vz_network_block_device_storage_device_attachment_bridge_cast(object).map(|inner| Self { inner })
  }
}

impl StorageDeviceAttachment for NetworkBlockDeviceStorageDeviceAttachment {}

/// [`VZNetworkBlockDeviceStorageDeviceAttachmentDelegate`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachmentdelegate)
///
/// Its methods are optional, as in Swift: unimplemented, they do nothing.
pub trait NetworkBlockDeviceStorageDeviceAttachmentDelegate: Send + Sync {
  /// [`attachmentWasConnected(_:)`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachmentdelegate/attachmentwasconnected(_:))
  fn attachment_was_connected(&self, _attachment: &NetworkBlockDeviceStorageDeviceAttachment) {}

  /// [`attachment(_:didEncounterError:)`](https://developer.apple.com/documentation/virtualization/vznetworkblockdevicestoragedeviceattachmentdelegate/attachment(_:didencountererror:))
  fn attachment_did_encounter_error(&self, _attachment: &NetworkBlockDeviceStorageDeviceAttachment, _error: Error) {}
}

/// A delegate as Virtualization holds it: weakly.
struct Delegating(Weak<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate>);

/// Called from Swift. Off macOS, `platform::unsupported` has the stand-in.
#[cfg(target_os = "macos")]
impl ffi::VZNetworkBlockDeviceStorageDeviceAttachmentDelegate for Delegating {
  fn attachment_was_connected(&self, attachment: ffi::VZNetworkBlockDeviceStorageDeviceAttachment) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.attachment_was_connected(&NetworkBlockDeviceStorageDeviceAttachment { inner: attachment });
    }
  }

  fn attachment_did_encounter_error(
    &self,
    attachment: ffi::VZNetworkBlockDeviceStorageDeviceAttachment,
    error: ffi::Retained<ffi::NSError>,
  ) {
    if let Some(delegate) = self.0.upgrade() {
      delegate.attachment_did_encounter_error(
        &NetworkBlockDeviceStorageDeviceAttachment { inner: attachment },
        foundation::error(&error),
      );
    }
  }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
  use super::*;

  #[test]
  fn is_unsupported_off_macos() {
    let url = Url::parse("nbd://localhost:10809/export").unwrap();

    assert!(matches!(
      NetworkBlockDeviceStorageDeviceAttachment::new(&url),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      NetworkBlockDeviceStorageDeviceAttachment::with_timeout_and_is_forced_read_only_and_synchronization_mode(
        &url,
        1.0,
        true,
        DiskSynchronizationMode::Full,
      ),
      Err(Error::Unsupported)
    ));
    assert!(matches!(
      NetworkBlockDeviceStorageDeviceAttachment::validate(&url),
      Err(Error::Unsupported)
    ));
  }
}
