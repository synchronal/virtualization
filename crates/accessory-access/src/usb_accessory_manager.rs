use std::fmt;
use std::sync::Arc;

use crate::platform::{self, ffi};
use crate::usb_accessory_listener::listening;
use crate::{Error, USBAccessory, USBAccessoryListener, USBAccessoryMatchingCriteria, foundation};

/// [`AAUSBAccessoryManager`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymanager)
pub struct USBAccessoryManager {
  inner: ffi::AAUSBAccessoryManager,
}

impl USBAccessoryManager {
  /// [`shared`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymanager/shared)
  pub fn shared() -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::aa_usb_accessory_manager_shared(),
    })
  }

  /// [`registerListener(_:matchingCriteria:completionHandler:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymanager/registerlistener(_:matchingcriteria:completionhandler:)):
  /// the accessories already connected. The bridge holds `listener` until
  /// it is unregistered.
  pub fn register_listener(
    &self,
    listener: &Arc<dyn USBAccessoryListener>,
    matching_criteria: &[&USBAccessoryMatchingCriteria],
    completion_handler: impl FnOnce(Result<Vec<USBAccessory>, Error>) + Send + 'static,
  ) {
    let matching_criteria: Vec<_> = matching_criteria
      .iter()
      .map(|criteria| &criteria.inner)
      .collect();

    self.inner.aa_register_listener(
      listening(listener),
      listener_key(listener),
      &matching_criteria,
      move |result| {
        completion_handler(
          result
            .map(|accessories| {
              accessories
                .into_iter()
                .map(|inner| USBAccessory { inner })
                .collect()
            })
            .map_err(|error| foundation::error(&error)),
        );
      },
    );
  }

  /// [`unregisterListener(_:completionHandler:)`](https://developer.apple.com/documentation/accessoryaccess/aausbaccessorymanager/unregisterlistener(_:completionhandler:)):
  /// `listener` is the `Arc` it was registered with.
  pub fn unregister_listener(
    &self,
    listener: &Arc<dyn USBAccessoryListener>,
    completion_handler: impl FnOnce() + Send + 'static,
  ) {
    self
      .inner
      .aa_unregister_listener(listening(listener), listener_key(listener), completion_handler);
  }
}

/// What the bridge knows `listener` by: its object's address.
fn listener_key(listener: &Arc<dyn USBAccessoryListener>) -> usize {
  Arc::as_ptr(listener).cast::<()>().addr()
}

impl Clone for USBAccessoryManager {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for USBAccessoryManager {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

#[cfg(test)]
mod tests {
  #[cfg(not(target_os = "macos"))]
  #[test]
  fn is_unsupported_off_macos() {
    use super::*;

    assert!(matches!(USBAccessoryManager::shared(), Err(Error::Unsupported)));
  }
}
