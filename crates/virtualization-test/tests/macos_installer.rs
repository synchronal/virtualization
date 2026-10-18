//! Each on a macOS machine's queue, where Virtualization takes an installer's
//! calls.

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{ErrorCode, MacOSInstaller, VirtualMachine};
use virtualization_test::{completed, file, macos_machine, ns_error, on_queue};

/// An installer for a machine from `macos_machine`, restoring from a file
/// that isn't a restore image.
fn installer(name: &str) -> (DispatchRetained<DispatchQueue>, VirtualMachine, MacOSInstaller, Url) {
  let (queue, machine) = macos_machine(name, |_| {});
  let url = file(&format!("{name}.ipsw"), b"not a restore image");
  let installer = on_queue(&queue, || MacOSInstaller::new(&machine, &url).unwrap());

  (queue, machine, installer, url)
}

#[test]
fn has_the_virtual_machine_it_was_given() {
  let (queue, machine, installer, _) = installer("installer_has_the_virtual_machine_it_was_given");

  assert_eq!(
    on_queue(&queue, || format!("{:?}", installer.virtual_machine())),
    format!("{machine:?}")
  );
}

#[test]
fn has_the_restore_image_url_it_was_given() {
  let (queue, _, installer, url) = installer("installer_has_the_restore_image_url_it_was_given");

  assert_eq!(on_queue(&queue, || installer.restore_image_url().unwrap()), url);
}

#[test]
fn holds_its_progress() {
  let (queue, _, installer, _) = installer("installer_holds_its_progress");
  let (first, second) = on_queue(&queue, || {
    (installer.progress() as usize, installer.progress() as usize)
  });

  assert_ne!(first, 0);
  assert_eq!(first, second);
}

#[test]
fn fails_as_virtualization_does_to_install_a_file_that_is_not_a_restore_image() {
  let (queue, _, installer, _) =
    installer("fails_as_virtualization_does_to_install_a_file_that_is_not_a_restore_image");

  let error = ns_error(completed(&queue, |completion_handler| {
    installer.install(completion_handler);
  }));

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::RestoreImageLoadFailed,
    "{error:#?}"
  );
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<MacOSInstaller>();
}

#[test]
fn a_clone_is_the_same_object() {
  let (queue, _, installer, _) = installer("installer_a_clone_is_the_same_object");

  on_queue(&queue, || {
    assert_eq!(format!("{:?}", installer.clone()), format!("{installer:?}"));
    assert_eq!(installer.clone().progress(), installer.progress());
  });
}

#[test]
fn debugs_as_its_description() {
  let (queue, _, installer, _) = installer("installer_debugs_as_its_description");

  assert!(on_queue(&queue, || format!("{installer:?}")).starts_with("<VZMacOSInstaller: 0x"));
}
