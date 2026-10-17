//! On the main thread, where AppKit's members run (`main_thread_tests`), with
//! machines on the main queue, where a view shows them.

use virtualization::{
  MainThreadMarker, VirtioGraphicsDeviceConfiguration, VirtioGraphicsScanoutConfiguration, VirtualMachine,
  VirtualMachineView, VirtualMachineViewAdaptor,
};
use virtualization_test::{linux_configuration, main_thread_tests};

/// A machine with a graphics device to show, on the main queue, as a view
/// needs.
fn machine(name: &str) -> VirtualMachine {
  let configuration = linux_configuration(name);
  let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

  device.set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);
  configuration.set_graphics_devices(&[&device]);

  VirtualMachine::new(&configuration).unwrap()
}

fn has_no_virtual_machine_when_new(mtm: MainThreadMarker) {
  assert!(VirtualMachineView::new(mtm).virtual_machine().is_none());
}

fn has_the_virtual_machine_it_was_given(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);
  let machine = machine("view_has_the_virtual_machine_it_was_given");

  view.set_virtual_machine(Some(&machine));

  assert_eq!(format!("{:?}", view.virtual_machine().unwrap()), format!("{machine:?}"));

  view.set_virtual_machine(None);

  assert!(view.virtual_machine().is_none());
}

/// A view keeps only a machine with a graphics device to show.
fn has_no_virtual_machine_when_given_one_without_graphics(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);

  view.set_virtual_machine(Some(
    &VirtualMachine::new(&linux_configuration(
      "has_no_virtual_machine_when_given_one_without_graphics",
    ))
    .unwrap(),
  ));

  assert!(view.virtual_machine().is_none());
}

fn has_no_adaptor_when_new(mtm: MainThreadMarker) {
  assert!(VirtualMachineView::new(mtm).adaptor().is_none());
}

fn has_the_adaptor_it_was_given(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);
  let adaptor = VirtualMachineViewAdaptor::new(&machine("view_has_the_adaptor_it_was_given")).unwrap();

  view.set_adaptor(Some(&adaptor));

  assert!(view.adaptor().is_some());

  view.set_adaptor(None);

  assert!(view.adaptor().is_none());
}

/// Apple documents that setting one resets the other.
fn has_no_virtual_machine_once_given_an_adaptor(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);
  let machine = machine("has_no_virtual_machine_once_given_an_adaptor");

  view.set_virtual_machine(Some(&machine));
  view.set_adaptor(Some(&VirtualMachineViewAdaptor::new(&machine).unwrap()));

  assert!(view.virtual_machine().is_none());
}

fn has_no_adaptor_once_given_a_virtual_machine(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);
  let machine = machine("has_no_adaptor_once_given_a_virtual_machine");

  view.set_adaptor(Some(&VirtualMachineViewAdaptor::new(&machine).unwrap()));
  view.set_virtual_machine(Some(&machine));

  assert!(view.adaptor().is_none());
}

fn does_not_capture_system_keys_when_new(mtm: MainThreadMarker) {
  assert!(!VirtualMachineView::new(mtm).captures_system_keys());
}

fn captures_system_keys_when_told_to(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);

  view.set_captures_system_keys(true);

  assert!(view.captures_system_keys());
}

fn does_not_automatically_reconfigure_its_display_when_new(mtm: MainThreadMarker) {
  assert!(!VirtualMachineView::new(mtm).automatically_reconfigures_display());
}

fn automatically_reconfigures_its_display_when_told_to(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);

  view.set_automatically_reconfigures_display(true);

  assert!(view.automatically_reconfigures_display());
}

fn has_its_ns_view(mtm: MainThreadMarker) {
  assert!(!VirtualMachineView::new(mtm).ns_view().is_null());
}

fn a_clone_is_the_same_object(mtm: MainThreadMarker) {
  let view = VirtualMachineView::new(mtm);

  view.clone().set_captures_system_keys(true);

  assert!(view.captures_system_keys());
  assert_eq!(view.clone().ns_view(), view.ns_view());
}

fn debugs_as_its_description(mtm: MainThreadMarker) {
  assert!(format!("{:?}", VirtualMachineView::new(mtm)).starts_with("<VZVirtualMachineView: 0x"));
}

fn an_adaptor_debugs_as_swift_describes_it(_mtm: MainThreadMarker) {
  let adaptor = VirtualMachineViewAdaptor::new(&machine("an_adaptor_debugs_as_swift_describes_it")).unwrap();

  assert!(
    format!("{adaptor:?}").starts_with("VZVirtualMachineViewAdaptor("),
    "{adaptor:?}"
  );
  assert_eq!(format!("{:?}", adaptor.clone()), format!("{adaptor:?}"));
}

fn an_adaptor_is_send_and_sync(_mtm: MainThreadMarker) {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<VirtualMachineViewAdaptor>();
}

fn main() {
  main_thread_tests(&[
    ("has_no_virtual_machine_when_new", has_no_virtual_machine_when_new),
    (
      "has_no_virtual_machine_when_given_one_without_graphics",
      has_no_virtual_machine_when_given_one_without_graphics,
    ),
    (
      "has_the_virtual_machine_it_was_given",
      has_the_virtual_machine_it_was_given,
    ),
    ("has_no_adaptor_when_new", has_no_adaptor_when_new),
    ("has_the_adaptor_it_was_given", has_the_adaptor_it_was_given),
    (
      "has_no_virtual_machine_once_given_an_adaptor",
      has_no_virtual_machine_once_given_an_adaptor,
    ),
    (
      "has_no_adaptor_once_given_a_virtual_machine",
      has_no_adaptor_once_given_a_virtual_machine,
    ),
    (
      "does_not_capture_system_keys_when_new",
      does_not_capture_system_keys_when_new,
    ),
    ("captures_system_keys_when_told_to", captures_system_keys_when_told_to),
    (
      "does_not_automatically_reconfigure_its_display_when_new",
      does_not_automatically_reconfigure_its_display_when_new,
    ),
    (
      "automatically_reconfigures_its_display_when_told_to",
      automatically_reconfigures_its_display_when_told_to,
    ),
    ("has_its_ns_view", has_its_ns_view),
    ("a_clone_is_the_same_object", a_clone_is_the_same_object),
    ("debugs_as_its_description", debugs_as_its_description),
    (
      "an_adaptor_debugs_as_swift_describes_it",
      an_adaptor_debugs_as_swift_describes_it,
    ),
    ("an_adaptor_is_send_and_sync", an_adaptor_is_send_and_sync),
  ]);
}
