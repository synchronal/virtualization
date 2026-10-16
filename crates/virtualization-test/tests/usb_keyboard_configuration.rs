use virtualization::USBKeyboardConfiguration;
use virtualization_test::{guest_pci_devices, linux_configuration};

#[test]
fn is_a_keyboard_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_keyboard_of_the_configuration_it_was_given_to");

  configuration.set_keyboards(&[&USBKeyboardConfiguration::new().unwrap()]);

  assert!(matches!(
    configuration.keyboards::<USBKeyboardConfiguration>()[..],
    [Some(_)]
  ));
  configuration.validate().unwrap();
}

/// The suites' kernel has no USB drivers, so the guest has the keyboard's
/// controller, by its PCI class (serial bus, USB, xHCI), and no keyboard.
#[test]
fn a_guest_has_a_usb_controller_for_it() {
  let devices = guest_pci_devices("keyboard_a_guest_has_a_usb_controller_for_it", |configuration| {
    configuration.set_keyboards(&[&USBKeyboardConfiguration::new().unwrap()]);
  });

  assert!(
    devices.iter().any(|device| device.ends_with(":0x0c0330")),
    "{devices:?}"
  );
}

#[test]
fn a_guest_without_one_has_no_usb_controller() {
  let devices = guest_pci_devices("keyboard_a_guest_without_one_has_no_usb_controller", |_| {});

  assert!(
    !devices.iter().any(|device| device.ends_with(":0x0c0330")),
    "{devices:?}"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let keyboard = USBKeyboardConfiguration::new().unwrap();

  assert_eq!(format!("{:?}", keyboard.clone()), format!("{keyboard:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", USBKeyboardConfiguration::new().unwrap()).starts_with("<VZUSBKeyboardConfiguration: 0x"));
}
