use std::io::Write;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, mpsc};

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::{
  Error, GenericMachineIdentifier, GenericPlatformConfiguration, VirtualMachine, VirtualMachineConfiguration,
  VirtualMachineDelegate, VirtualMachineState,
};
use virtualization_test::{
  booting_machine, completed, initial_ramdisk, linux_configuration, new_file_url, on_queue, serial_queue, start, stop,
};

struct Delegate;

impl VirtualMachineDelegate for Delegate {}

fn linux_machine(name: &str) -> (DispatchRetained<DispatchQueue>, VirtualMachine) {
  let queue = serial_queue(name);
  let machine = on_queue(&queue, || {
    VirtualMachine::with_queue(&linux_configuration(name), &queue).unwrap()
  });

  (queue, machine)
}

fn ns_error(result: Result<(), Error>) -> Box<virtualization::NSError> {
  match result {
    Err(Error::NSError(error)) => error,
    other => panic!("expected Virtualization's error, got {other:?}"),
  }
}

#[test]
fn is_supported_on_the_host() {
  assert!(VirtualMachine::is_supported().unwrap());
}

#[test]
fn is_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}

  assert_send_sync::<VirtualMachine>();
}

#[test]
fn is_stopped_when_new() {
  let (queue, machine) = linux_machine("is_stopped_when_new");

  on_queue(&queue, || {
    assert_eq!(machine.state(), VirtualMachineState::Stopped);
    assert!(machine.can_start());
    assert!(!machine.can_stop());
    assert!(!machine.can_pause());
    assert!(!machine.can_resume());
    assert!(!machine.can_request_stop());
    assert!(machine.network_devices().is_empty());
  });
}

#[test]
fn has_the_queue_it_was_given() {
  let (queue, machine) = linux_machine("has_the_queue_it_was_given");

  assert_eq!(DispatchRetained::as_ptr(&machine.queue()), NonNull::from(&*queue));
}

#[test]
fn has_the_main_queue_when_given_none() {
  let machine = VirtualMachine::new(&linux_configuration("has_the_main_queue_when_given_none")).unwrap();

  assert_eq!(
    DispatchRetained::as_ptr(&machine.queue()),
    NonNull::from(DispatchQueue::main())
  );
}

#[test]
fn has_the_delegate_it_was_given() {
  let (queue, machine) = linux_machine("has_the_delegate_it_was_given");
  let delegate: Arc<dyn VirtualMachineDelegate> = Arc::new(Delegate);

  on_queue(&queue, || {
    assert!(machine.delegate().is_none());

    machine.set_delegate(Some(&delegate));

    assert!(Arc::ptr_eq(&machine.delegate().unwrap(), &delegate));

    machine.set_delegate(None);

    assert!(machine.delegate().is_none());
  });
}

#[test]
fn holds_its_delegate_weakly() {
  let (queue, machine) = linux_machine("holds_its_delegate_weakly");
  let delegate: Arc<dyn VirtualMachineDelegate> = Arc::new(Delegate);

  on_queue(&queue, || machine.set_delegate(Some(&delegate)));
  drop(delegate);

  assert!(on_queue(&queue, || machine.delegate()).is_none());
}

#[test]
fn fails_as_virtualization_does_to_start_with_a_placeholder_kernel() {
  let (queue, machine) = linux_machine("fails_to_start_with_a_placeholder_kernel");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine.start(completion_handler);
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 1, "{error:#?}");
  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Error);
}

#[test]
fn fails_as_virtualization_does_to_stop_a_stopped_machine() {
  let (queue, machine) = linux_machine("fails_to_stop_a_stopped_machine");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine.stop(completion_handler);
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 4, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_pause_a_stopped_machine() {
  let (queue, machine) = linux_machine("fails_to_pause_a_stopped_machine");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine.pause(completion_handler);
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 4, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_resume_a_stopped_machine() {
  let (queue, machine) = linux_machine("fails_to_resume_a_stopped_machine");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine.resume(completion_handler);
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 4, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_request_a_stopped_machine_stop() {
  let (queue, machine) = linux_machine("fails_to_request_a_stopped_machine_stop");

  let error = ns_error(on_queue(&queue, || machine.request_stop()));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 3, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_save_a_stopped_machine() {
  let (queue, machine) = linux_machine("fails_to_save_a_stopped_machine");
  let url = new_file_url("fails_to_save_a_stopped_machine.vzvmsave");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine
      .save_machine_state_to(&url, completion_handler)
      .unwrap();
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 3, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_to_restore_from_a_missing_file() {
  let (queue, machine) = linux_machine("fails_to_restore_from_a_missing_file");
  let url = new_file_url("fails_to_restore_from_a_missing_file.vzvmsave");

  let error = ns_error(completed(&queue, |completion_handler| {
    machine
      .restore_machine_state_from(&url, completion_handler)
      .unwrap();
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 12, "{error:#?}");
  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Stopped);
}

/// A guest that echoes each line it reads, once it says it's ready.
const ECHO: &str = "echo ready\nwhile read line; do echo \"echo:$line\"; done";

#[test]
fn boots_linux_to_its_console() {
  let (queue, machine, mut console) = booting_machine("boots_linux_to_its_console", None, |_| {});

  start(&queue, &machine);

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);
  console.read_until("Kernel panic - not syncing: VFS: Unable to mount root fs");

  stop(&queue, &machine);

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Stopped);
}

#[test]
fn reads_and_writes_its_console() {
  let name = "reads_and_writes_its_console";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, ECHO)), |_| {});

  start(&queue, &machine);
  console.read_until("ready");
  writeln!(console.input, "hello").unwrap();

  console.read_until("echo:hello");
}

#[test]
fn pauses_and_resumes_a_running_machine() {
  let name = "pauses_and_resumes_a_running_machine";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, ECHO)), |_| {});

  start(&queue, &machine);
  console.read_until("ready");

  on_queue(&queue, || {
    assert!(machine.can_pause());
    assert!(machine.can_stop());
    assert!(machine.can_request_stop());
    assert!(!machine.can_resume());
    assert!(!machine.can_start());
  });
  completed(&queue, |completion_handler| machine.pause(completion_handler)).unwrap();

  on_queue(&queue, || {
    assert_eq!(machine.state(), VirtualMachineState::Paused);
    assert!(machine.can_resume());
    assert!(!machine.can_pause());
  });
  completed(&queue, |completion_handler| machine.resume(completion_handler)).unwrap();

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);
  writeln!(console.input, "resumed").unwrap();
  console.read_until("echo:resumed");
}

/// The guest has no input devices, so nothing in it receives the request.
#[test]
fn requests_a_stop_that_a_guest_without_a_power_button_ignores() {
  let name = "requests_a_stop_that_a_guest_without_a_power_button_ignores";
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, ECHO)), |_| {});

  start(&queue, &machine);
  console.read_until("ready");
  on_queue(&queue, || machine.request_stop()).unwrap();
  writeln!(console.input, "still running").unwrap();

  console.read_until("echo:still running");
  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);
}

#[test]
fn saves_and_restores_a_paused_machine() {
  let name = "saves_and_restores_a_paused_machine";
  let url = new_file_url("saves_and_restores_a_paused_machine.vzvmsave");
  let initial_ramdisk = initial_ramdisk(name, ECHO);
  let identifier = GenericMachineIdentifier::new()
    .unwrap()
    .data_representation();
  let with_identifier = |configuration: &VirtualMachineConfiguration| {
    let platform = GenericPlatformConfiguration::new().unwrap();

    platform.set_machine_identifier(
      &GenericMachineIdentifier::with_data_representation(&identifier)
        .unwrap()
        .unwrap(),
    );
    configuration.set_platform(&platform);
    configuration.validate_save_restore_support().unwrap();
  };
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk), with_identifier);

  start(&queue, &machine);
  console.read_until("ready");
  completed(&queue, |completion_handler| machine.pause(completion_handler)).unwrap();
  completed(&queue, |completion_handler| {
    machine
      .save_machine_state_to(&url, completion_handler)
      .unwrap()
  })
  .unwrap();

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Paused);
  stop(&queue, &machine);

  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk), with_identifier);
  completed(&queue, |completion_handler| {
    machine
      .restore_machine_state_from(&url, completion_handler)
      .unwrap()
  })
  .unwrap();

  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Paused);
  completed(&queue, |completion_handler| machine.resume(completion_handler)).unwrap();
  writeln!(console.input, "restored").unwrap();
  console.read_until("echo:restored");
}

/// Sends what stopped the machine.
struct Stops(Mutex<mpsc::Sender<Result<(), Error>>>);

impl VirtualMachineDelegate for Stops {
  fn guest_did_stop(&self, _virtual_machine: &VirtualMachine) {
    self.0.lock().unwrap().send(Ok(())).unwrap();
  }

  fn virtual_machine_did_stop_with_error(&self, _virtual_machine: &VirtualMachine, error: Error) {
    self.0.lock().unwrap().send(Err(error)).unwrap();
  }
}

#[test]
fn tells_its_delegate_when_the_guest_stops() {
  let name = "tells_its_delegate_when_the_guest_stops";
  let (queue, machine, _console) = booting_machine(name, Some(&initial_ramdisk(name, "poweroff -f")), |_| {});
  let (sender, receiver) = mpsc::channel();
  let delegate: Arc<dyn VirtualMachineDelegate> = Arc::new(Stops(Mutex::new(sender)));

  on_queue(&queue, || machine.set_delegate(Some(&delegate)));
  start(&queue, &machine);

  receiver.recv().unwrap().unwrap();
  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Stopped);
}

#[test]
fn a_clone_is_the_same_object() {
  let (queue, machine) = linux_machine("a_clone_is_the_same_object");
  let delegate: Arc<dyn VirtualMachineDelegate> = Arc::new(Delegate);

  on_queue(&queue, || {
    let clone = machine.clone();

    machine.set_delegate(Some(&delegate));

    assert!(Arc::ptr_eq(&clone.delegate().unwrap(), &delegate));
  });
}

#[test]
fn debugs_as_its_description() {
  let (queue, machine) = linux_machine("debugs_as_its_description");

  assert!(on_queue(&queue, || format!("{machine:?}")).starts_with("<VZVirtualMachine: 0x"));
}
