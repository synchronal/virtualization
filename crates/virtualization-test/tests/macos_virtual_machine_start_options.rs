use virtualization::{ErrorCode, MacGuestProvisioningOptions, MacOSVirtualMachineStartOptions, VirtualMachineState};
use virtualization_test::{completed, macos_machine, ns_error, on_queue, stop};

/// Guest provisioning options that validate.
fn guest_provisioning_options() -> MacGuestProvisioningOptions {
  let options = MacGuestProvisioningOptions::new().unwrap();

  options.set_full_name("Test User");
  options.set_username("test");
  options.set_password("password");

  options
}

#[test]
fn does_not_start_up_from_macos_recovery_when_new() {
  assert!(
    !MacOSVirtualMachineStartOptions::new()
      .unwrap()
      .start_up_from_macos_recovery()
  );
}

#[test]
fn has_the_start_up_from_macos_recovery_it_was_given() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();

  options.set_start_up_from_macos_recovery(true);

  assert!(options.start_up_from_macos_recovery());
}

#[test]
fn has_no_guest_provisioning_options_when_new() {
  assert!(
    MacOSVirtualMachineStartOptions::new()
      .unwrap()
      .guest_provisioning_options()
      .is_none()
  );
}

#[test]
fn has_a_copy_of_the_guest_provisioning_options_it_was_given() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();
  let guest_provisioning_options = guest_provisioning_options();

  options
    .set_guest_provisioning(Some(&guest_provisioning_options))
    .unwrap();
  guest_provisioning_options.set_username("changed");

  assert_eq!(options.guest_provisioning_options().unwrap().username(), "test");
}

#[test]
fn has_no_guest_provisioning_options_once_given_none() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();

  options
    .set_guest_provisioning(Some(&guest_provisioning_options()))
    .unwrap();
  options.set_guest_provisioning(None).unwrap();

  assert!(options.guest_provisioning_options().is_none());
}

#[test]
fn fails_as_virtualization_does_to_take_guest_provisioning_options_that_do_not_validate() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();

  let error = ns_error(options.set_guest_provisioning(Some(&MacGuestProvisioningOptions::new().unwrap())));

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::GuestProvisioningInvalidPassword,
    "{error:#?}"
  );
  assert!(options.guest_provisioning_options().is_none());
}

/// With nothing installed to boot, the machine still runs.
#[test]
fn starts_a_macos_machine() {
  let (queue, machine) = macos_machine("starts_a_macos_machine", |_| {});

  completed(&queue, |completion_handler| {
    let options = MacOSVirtualMachineStartOptions::new().unwrap();

    options.set_start_up_from_macos_recovery(true);
    machine.start_with_options(&options, completion_handler);
  })
  .unwrap();

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);

  stop(&queue, &machine);
}

#[test]
fn a_clone_is_the_same_object() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();

  options.clone().set_start_up_from_macos_recovery(true);

  assert!(options.start_up_from_macos_recovery());
}

#[test]
fn debugs_as_its_description() {
  let options = MacOSVirtualMachineStartOptions::new().unwrap();

  assert!(format!("{options:?}").starts_with("<VZMacOSVirtualMachineStartOptions: 0x"));
}
