use std::sync::Arc;

use dispatch2::{DispatchQueue, DispatchRetained};
use uuid::Uuid;
use virtualization::{
  ErrorCode, USBController, USBControllerConfiguration, USBControllerDelegate, USBDevice, USBDeviceConfiguration,
  USBMassStorageDevice, VirtualMachine, XHCIController, XHCIControllerConfiguration,
};
use virtualization_test::{
  Console, booting_machine, completed, initial_ramdisk, ns_error, on_queue, start,
  usb_mass_storage_device_configuration, with_usb_mass_storage_device, xhci_controller,
};

/// The UUID of the device a machine's configuration gives its controller.
const CONFIGURED: &str = "01234567-89ab-cdef-0123-456789abcdef";

/// A machine with an XHCI controller, and a USB mass storage device on it,
/// whose guest reports its PCI devices' classes and waits.
fn machine(name: &str) -> (DispatchRetained<DispatchQueue>, VirtualMachine, Console) {
  let script = "echo \"classes:$(cat /sys/bus/pci/devices/*/class | tr '\\n' ' ')\"\nread line";

  booting_machine(name, Some(&initial_ramdisk(name, script)), |machine| {
    let controller = XHCIControllerConfiguration::new().unwrap();
    let device = usb_mass_storage_device_configuration(name);

    device.set_uuid(&CONFIGURED.parse().unwrap());
    controller.set_usb_devices(&[&device]);
    machine.set_usb_controllers(&[&controller]);
  })
}

/// The UUID of each device on `machine`'s controller.
fn devices(queue: &DispatchQueue, machine: &VirtualMachine) -> Vec<Uuid> {
  on_queue(queue, || {
    xhci_controller(machine)
      .usb_devices::<USBMassStorageDevice>()
      .iter()
      .map(|device| device.as_ref().unwrap().uuid())
      .collect()
  })
}

#[test]
fn is_one_per_usb_controller_configuration() {
  let name = "is_one_per_usb_controller_configuration";

  let (queue, machine, _console) = booting_machine(name, None, |machine| {
    machine.set_usb_controllers(&[
      &XHCIControllerConfiguration::new().unwrap(),
      &XHCIControllerConfiguration::new().unwrap(),
    ]);
  });

  on_queue(&queue, || {
    assert!(matches!(
      machine.usb_controllers::<XHCIController>()[..],
      [Some(_), Some(_)]
    ));
  });
}

struct Delegate;

impl USBControllerDelegate for Delegate {}

/// Runs `check` on the controller of a machine with one, on its queue and not
/// started.
fn with_controller(name: &str, check: impl FnOnce(&XHCIController) + Send) {
  let (queue, machine, _console) = booting_machine(name, None, |machine| {
    machine.set_usb_controllers(&[&XHCIControllerConfiguration::new().unwrap()]);
  });

  on_queue(&queue, || check(&xhci_controller(&machine)));
}

#[test]
fn has_the_delegate_it_was_given() {
  with_controller("usb_controller_has_the_delegate_it_was_given", |controller| {
    let delegate: Arc<dyn USBControllerDelegate> = Arc::new(Delegate);

    assert!(controller.delegate().is_none());

    controller.set_delegate(Some(&delegate));

    assert!(Arc::ptr_eq(&controller.delegate().unwrap(), &delegate));

    controller.set_delegate(None);

    assert!(controller.delegate().is_none());
  });
}

#[test]
fn holds_its_delegate_weakly() {
  with_controller("usb_controller_holds_its_delegate_weakly", |controller| {
    let delegate: Arc<dyn USBControllerDelegate> = Arc::new(Delegate);

    controller.set_delegate(Some(&delegate));
    drop(delegate);

    assert!(controller.delegate().is_none());
  });
}

/// Apple documents that they appear once the machine starts.
#[test]
fn has_the_devices_of_its_configuration_before_the_machine_starts() {
  let (queue, machine, _console) = machine("has_the_devices_of_its_configuration_before_the_machine_starts");

  assert_eq!(devices(&queue, &machine), [CONFIGURED.parse::<Uuid>().unwrap()]);
}

/// The suites' kernel has no USB drivers, so the guest has the controller, by
/// its PCI class (serial bus, USB, XHCI), and no devices on it.
#[test]
fn a_guest_has_an_xhci_controller() {
  let (queue, machine, mut console) = machine("a_guest_has_an_xhci_controller");

  start(&queue, &machine);

  assert!(console.read_until("classes:").contains("0x0c0330"));
}

/// Apple documents that a device ever attached to a controller fails to
/// attach again. It attaches again once detached.
#[test]
fn attaches_and_detaches_a_device_while_running() {
  let (queue, machine, mut console) = machine("attaches_and_detaches_a_device_while_running");
  let configured: Uuid = CONFIGURED.parse().unwrap();
  let attached = with_usb_mass_storage_device(USBDevice::uuid);
  let attach = || {
    completed(&queue, |completion_handler| {
      with_usb_mass_storage_device(|device| xhci_controller(&machine).attach(device, completion_handler));
    })
  };
  let detach = || {
    completed(&queue, |completion_handler| {
      with_usb_mass_storage_device(|device| xhci_controller(&machine).detach(device, completion_handler));
    })
  };

  start(&queue, &machine);
  console.read_until("classes:");

  attach().unwrap();

  assert_eq!(devices(&queue, &machine), [configured, attached]);

  let error = ns_error(attach());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::DeviceAlreadyAttached,
    "{error:#?}"
  );

  detach().unwrap();

  assert_eq!(devices(&queue, &machine), [configured]);

  let error = ns_error(detach());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::DeviceNotFound,
    "{error:#?}"
  );

  attach().unwrap();

  assert_eq!(devices(&queue, &machine), [configured, attached]);
}

#[test]
fn a_clone_is_the_same_object() {
  let (queue, machine, _console) = machine("xhci_controller_a_clone_is_the_same_object");

  on_queue(&queue, || {
    let controller = xhci_controller(&machine);
    let clone = controller.clone();

    drop(controller);

    assert_eq!(clone.usb_devices::<USBMassStorageDevice>().len(), 1);
  });
}

#[test]
fn debugs_as_its_description() {
  let (queue, machine, _console) = machine("xhci_controller_debugs_as_its_description");

  on_queue(&queue, || {
    assert!(format!("{:?}", xhci_controller(&machine)).starts_with("<VZXHCIController: 0x"));
  });
}
