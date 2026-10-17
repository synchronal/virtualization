use std::ffi::c_void;
use std::fmt;

use crate::platform::{self, ffi};
use crate::{Error, MainThreadMarker, VirtualMachine};

/// [`VZVirtualMachineView`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview)
///
/// AppKit's, so made only on the main thread, and neither `Send` nor `Sync`:
/// it stays there. AppKit's own members are the `NSView`'s, through
/// [`Self::ns_view`].
pub struct VirtualMachineView {
  inner: ffi::VZVirtualMachineView,
}

impl VirtualMachineView {
  /// [`init()`](https://developer.apple.com/documentation/objectivec/nsobject-swift.class/init()),
  /// on the main thread.
  pub fn new(mtm: MainThreadMarker) -> Self {
    Self {
      inner: ffi::vz_virtual_machine_view_init(mtm.inner),
    }
  }

  /// The view's `NSView *`, for AppKit through objc2-app-kit: valid as long
  /// as this view, which holds it.
  pub fn ns_view(&self) -> *mut c_void {
    self.inner.bridge_borrowed().cast_mut()
  }

  /// [`virtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/virtualmachine)
  pub fn virtual_machine(&self) -> Option<VirtualMachine> {
    self
      .inner
      .vz_virtual_machine()
      .ok()
      .map(|inner| VirtualMachine { inner })
  }

  /// [`virtualMachine`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/virtualmachine)
  pub fn set_virtual_machine(&self, virtual_machine: Option<&VirtualMachine>) {
    match virtual_machine {
      Some(virtual_machine) => {
        self
          .inner
          .vz_set_virtual_machine(virtual_machine.inner.bridge_retained());
      }
      None => self.inner.bridge_set_virtual_machine_nil(),
    }
  }

  /// [`capturesSystemKeys`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/capturessystemkeys)
  pub fn captures_system_keys(&self) -> bool {
    self.inner.vz_captures_system_keys()
  }

  /// [`capturesSystemKeys`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/capturessystemkeys)
  pub fn set_captures_system_keys(&self, captures_system_keys: bool) {
    self.inner.vz_set_captures_system_keys(captures_system_keys);
  }

  /// [`automaticallyReconfiguresDisplay`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/automaticallyreconfiguresdisplay)
  pub fn automatically_reconfigures_display(&self) -> bool {
    self.inner.vz_automatically_reconfigures_display()
  }

  /// [`automaticallyReconfiguresDisplay`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/automaticallyreconfiguresdisplay)
  pub fn set_automatically_reconfigures_display(&self, automatically_reconfigures_display: bool) {
    self
      .inner
      .vz_set_automatically_reconfigures_display(automatically_reconfigures_display);
  }

  /// [`adaptor`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/adaptor)
  pub fn adaptor(&self) -> Option<VirtualMachineViewAdaptor> {
    self
      .inner
      .vz_adaptor()
      .ok()
      .map(|inner| VirtualMachineViewAdaptor { inner })
  }

  /// [`adaptor`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineview/adaptor)
  pub fn set_adaptor(&self, adaptor: Option<&VirtualMachineViewAdaptor>) {
    match adaptor {
      Some(adaptor) => self.inner.vz_set_adaptor(adaptor.inner.bridge_retained()),
      None => self.inner.bridge_set_adaptor_nil(),
    }
  }
}

impl Clone for VirtualMachineView {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// The object's `description`.
impl fmt::Debug for VirtualMachineView {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.ns_description())
  }
}

/// [`VZVirtualMachineViewAdaptor`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineviewadaptor)
///
/// `Send` and `Sync`, as Apple declares it `Sendable`: made on the machine's
/// queue, given to a view on the main thread.
pub struct VirtualMachineViewAdaptor {
  inner: ffi::BridgedVZVirtualMachineViewAdaptor,
}

impl VirtualMachineViewAdaptor {
  /// [`init(virtualMachine:)`](https://developer.apple.com/documentation/virtualization/vzvirtualmachineviewadaptor/init(virtualmachine:))
  pub fn new(virtual_machine: &VirtualMachine) -> Result<Self, Error> {
    platform::supported()?;

    Ok(Self {
      inner: ffi::vz_virtual_machine_view_adaptor_init_virtual_machine(&virtual_machine.inner),
    })
  }
}

/// The same value: Swift's is a struct, which copies.
impl Clone for VirtualMachineViewAdaptor {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.bridge_retained(),
    }
  }
}

/// Swift's `String(describing:)`.
impl fmt::Debug for VirtualMachineViewAdaptor {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&self.inner.bridge_description())
  }
}
