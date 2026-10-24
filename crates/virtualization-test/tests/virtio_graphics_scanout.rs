use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::{
  CGSize, ErrorCode, GraphicsDevice, GraphicsDisplay, GraphicsDisplayObserver, VirtioGraphicsDevice,
  VirtioGraphicsDeviceConfiguration, VirtioGraphicsScanout, VirtioGraphicsScanoutConfiguration, VirtualMachine,
  VirtualMachineConfiguration,
};
use virtualization_test::{
  booting_machine, initial_ramdisk, linux_configuration, ns_error, on_queue, serial_queue, start,
};

/// Gives `configuration` a graphics device with one 1920 by 1080 scanout.
fn add_graphics_device(configuration: &VirtualMachineConfiguration) {
  let device = VirtioGraphicsDeviceConfiguration::new().unwrap();

  device.set_scanouts(&[&VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap()]);
  configuration.set_graphics_devices(&[&device]);
}

/// `machine`'s scanout. Call it on the machine's queue.
fn scanout(machine: &VirtualMachine) -> VirtioGraphicsScanout {
  let devices = machine.graphics_devices::<VirtioGraphicsDevice>();
  let displays = devices[0]
    .as_ref()
    .unwrap()
    .displays::<VirtioGraphicsScanout>();

  displays[0].clone().unwrap()
}

/// Runs `check` on the scanout of a machine made on its queue and not
/// started.
fn with_scanout(name: &str, check: impl FnOnce(&VirtioGraphicsScanout) + Send) {
  let queue = serial_queue(name);

  on_queue(&queue, || {
    let configuration = linux_configuration(name);

    add_graphics_device(&configuration);
    check(&scanout(&VirtualMachine::with_queue(&configuration, &queue).unwrap()));
  });
}

/// A machine with a scanout, started, its guest running.
fn running_machine(name: &str) -> (DispatchRetained<DispatchQueue>, VirtualMachine) {
  let (queue, machine, mut console) = booting_machine(
    name,
    Some(&initial_ramdisk(name, "echo ready\nsleep 60")),
    add_graphics_device,
  );

  start(&queue, &machine);
  console.read_until("ready");

  (queue, machine)
}

const SIZE: CGSize = CGSize {
  width: 800.0,
  height: 600.0,
};

/// What an observer is told, each with the display's size then.
#[derive(Debug, PartialEq)]
enum Told {
  Began(CGSize),
  Ended(CGSize),
}

/// Tells its sender what it is told.
struct Observer(Mutex<Sender<Told>>);

/// An `Observer`, and what it is told.
fn observer() -> (Arc<dyn GraphicsDisplayObserver>, Receiver<Told>) {
  let (sender, receiver) = mpsc::channel();

  (Arc::new(Observer(Mutex::new(sender))), receiver)
}

impl GraphicsDisplayObserver for Observer {
  fn display_did_begin_reconfiguration(&self, display: &dyn GraphicsDisplay) {
    let _ = self
      .0
      .lock()
      .unwrap()
      .send(Told::Began(display.size_in_pixels()));
  }

  fn display_did_end_reconfiguration(&self, display: &dyn GraphicsDisplay) {
    let _ = self
      .0
      .lock()
      .unwrap()
      .send(Told::Ended(display.size_in_pixels()));
  }
}

/// What `receiver` is told within five seconds.
fn told(receiver: &Receiver<Told>) -> Told {
  receiver.recv_timeout(Duration::from_secs(5)).unwrap()
}

/// Does nothing it is told.
struct Ignores;

impl GraphicsDisplayObserver for Ignores {}

#[test]
fn has_the_size_of_its_configuration() {
  with_scanout("has_the_size_of_its_configuration", |scanout| {
    assert_eq!(
      scanout.size_in_pixels(),
      CGSize {
        width: 1920.0,
        height: 1080.0
      }
    );
  });
}

#[test]
fn fails_as_virtualization_does_to_reconfigure_its_size_before_the_machine_starts() {
  with_scanout(
    "fails_as_virtualization_does_to_reconfigure_its_size_before_the_machine_starts",
    |scanout| {
      let error = ns_error(scanout.reconfigure_with_size_in_pixels(SIZE));

      assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
      assert_eq!(
        ErrorCode::new(error.code).unwrap(),
        ErrorCode::InvalidVirtualMachineConfiguration,
        "{error:#?}"
      );
    },
  );
}

#[test]
fn fails_as_virtualization_does_to_reconfigure_before_the_machine_starts() {
  with_scanout(
    "fails_as_virtualization_does_to_reconfigure_before_the_machine_starts",
    |scanout| {
      let error =
        ns_error(scanout.reconfigure_with_configuration(&VirtioGraphicsScanoutConfiguration::new(800, 600).unwrap()));

      assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
      assert_eq!(
        ErrorCode::new(error.code).unwrap(),
        ErrorCode::InvalidVirtualMachineConfiguration,
        "{error:#?}"
      );
    },
  );
}

#[test]
fn fails_as_virtualization_does_on_a_size_that_is_not_positive() {
  with_scanout(
    "fails_as_virtualization_does_on_a_size_that_is_not_positive",
    |scanout| {
      let error = ns_error(scanout.reconfigure_with_size_in_pixels(CGSize {
        width: 0.0,
        height: 0.0,
      }));

      assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
      assert_eq!(
        ErrorCode::new(error.code).unwrap(),
        ErrorCode::InvalidVirtualMachineConfiguration,
        "{error:#?}"
      );
    },
  );
}

#[test]
fn holds_its_observer_weakly() {
  with_scanout("holds_its_observer_weakly", |scanout| {
    let observer: Arc<dyn GraphicsDisplayObserver> = Arc::new(Ignores);
    let held = Arc::downgrade(&observer);

    scanout.add_observer(&observer);
    drop(observer);

    assert!(held.upgrade().is_none());
  });
}

/// The suites' kernel has no virtio GPU driver: Virtualization resizes the
/// scanout without the guest.
#[test]
fn reconfigures_its_size_and_tells_its_observer() {
  let (queue, machine) = running_machine("reconfigures_its_size_and_tells_its_observer");
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);

    scanout.add_observer(&observer);
    scanout.reconfigure_with_size_in_pixels(SIZE).unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
  assert_eq!(on_queue(&queue, || scanout(&machine).size_in_pixels()), SIZE);
}

#[test]
fn reconfigures_to_a_configuration_and_tells_its_observer() {
  let (queue, machine) = running_machine("reconfigures_to_a_configuration_and_tells_its_observer");
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);

    scanout.add_observer(&observer);
    scanout
      .reconfigure_with_configuration(&VirtioGraphicsScanoutConfiguration::new(800, 600).unwrap())
      .unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
}

#[test]
fn does_not_tell_an_observer_it_removed() {
  let (queue, machine) = running_machine("does_not_tell_an_observer_it_removed");
  let (removed, removed_receiver) = observer();
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);

    scanout.add_observer(&removed);
    scanout.add_observer(&observer);
    scanout.remove_observer(&removed);
    scanout.reconfigure_with_size_in_pixels(SIZE).unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
  assert!(removed_receiver.try_recv().is_err());
}

#[test]
fn tells_an_observer_added_after_one_dropped() {
  let (queue, machine) = running_machine("tells_an_observer_added_after_one_dropped");
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);
    let dropped: Arc<dyn GraphicsDisplayObserver> = Arc::new(Ignores);

    scanout.add_observer(&dropped);
    drop(dropped);
    scanout.add_observer(&observer);
    scanout.reconfigure_with_size_in_pixels(SIZE).unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
}

#[test]
fn tells_an_observer_added_twice_twice() {
  let (queue, machine) = running_machine("tells_an_observer_added_twice_twice");
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);

    scanout.add_observer(&observer);
    scanout.add_observer(&observer);
    scanout.reconfigure_with_size_in_pixels(SIZE).unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
}

#[test]
fn does_not_tell_an_observer_added_twice_and_removed_once() {
  let (queue, machine) = running_machine("does_not_tell_an_observer_added_twice_and_removed_once");
  let (removed, removed_receiver) = observer();
  let (observer, receiver) = observer();

  on_queue(&queue, || {
    let scanout = scanout(&machine);

    scanout.add_observer(&removed);
    scanout.add_observer(&removed);
    scanout.add_observer(&observer);
    scanout.remove_observer(&removed);
    scanout.reconfigure_with_size_in_pixels(SIZE).unwrap();
  });

  assert_eq!(told(&receiver), Told::Began(SIZE));
  assert_eq!(told(&receiver), Told::Ended(SIZE));
  assert!(removed_receiver.try_recv().is_err());
}

#[test]
fn a_clone_is_the_same_object() {
  with_scanout("scanout_a_clone_is_the_same_object", |scanout| {
    assert_eq!(format!("{:?}", scanout.clone()), format!("{scanout:?}"));
  });
}

#[test]
fn debugs_as_its_description() {
  with_scanout("scanout_debugs_as_its_description", |scanout| {
    assert!(format!("{scanout:?}").starts_with("<VZVirtioGraphicsScanout: 0x"));
  });
}
