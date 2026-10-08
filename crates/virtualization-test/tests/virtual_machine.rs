use std::ptr::NonNull;
use std::sync::Arc;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::{Error, VirtualMachine, VirtualMachineDelegate, VirtualMachineState};
use virtualization_test::{completed, linux_configuration, new_file_url, on_queue, serial_queue};

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
