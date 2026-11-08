use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use url::Url;
use virtualization::{
  CustomVirtioDevice, CustomVirtioDeviceConfiguration, CustomVirtioDeviceDelegate, ErrorCode, GenericMachineIdentifier,
  GenericPlatformConfiguration, VirtioDeviceSpecificConfiguration, VirtualMachineConfiguration,
};
use virtualization_test::{
  CustomDevice, CustomDeviceMachine, ENTROPY, READ_HWRNG, completed, custom_device_machine,
  custom_device_machine_configured, new_file_url, ns_error, start, stop,
};

/// Reads the guest's hardware random number generator, then waits for a line
/// on the console before powering off.
const READ_THEN_WAIT: &str =
  "echo \"rng:$(head -c 16 /dev/hwrng | od -An -tx1 | tr -d ' \\n')\"\nread line\npoweroff -f";

#[test]
fn a_guest_reads_what_the_device_writes() {
  let mut machine = custom_device_machine(
    "a_guest_reads_what_the_device_writes",
    READ_HWRNG,
    CustomDevice::default(),
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(
    machine.console.read_until("rng:").trim_end(),
    format!("rng:{}", format!("{ENTROPY:02x}").repeat(16))
  );
}

/// Lists the guest's PCI devices, each as `vendor:device:class`, after `pci:`.
const PCI: &str = "echo \"pci:$(for d in /sys/bus/pci/devices/*; do echo \"$(cat $d/vendor):$(cat $d/device):$(cat \
                   $d/class)\"; done | tr '\\n' ' ')\"";

/// By its vendor (Red Hat's virtio), its device (`0x1040` plus its Virtio
/// device ID), and its PCI class and subclass.
#[test]
fn a_guest_has_the_device_by_its_ids() {
  let mut machine = custom_device_machine(
    "a_guest_has_the_device_by_its_ids",
    PCI,
    CustomDevice::default(),
    |configuration| configuration.set_pci_subclass_id(0x80),
  );

  start(&machine.queue, &machine.machine);
  let line = machine.console.read_until("pci:");

  assert!(line.contains("0x1af4:0x1044:0xff8000"), "{line}");
}

#[test]
fn is_made_from_its_configuration_on_its_queue() {
  let machine = custom_device_machine(
    "is_made_from_its_configuration_on_its_queue",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|configuration, device| format!("{}:{:p}", configuration.device_id(), &*device.device_queue())),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), format!("4:{:p}", &*machine.device_queue));
}

#[test]
fn has_no_negotiated_features_or_queues_until_the_driver_is_ok() {
  let machine = custom_device_machine(
    "has_no_negotiated_features_or_queues_until_the_driver_is_ok",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| {
        format!(
          "{}:{}",
          device.negotiated_features().is_some(),
          device.queue(0).is_some()
        )
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), "false:false");
}

#[test]
fn has_negotiated_features_once_the_driver_is_ok() {
  let machine = custom_device_machine(
    "has_negotiated_features_once_the_driver_is_ok",
    READ_HWRNG,
    CustomDevice {
      driver_ok: Box::new(|device| device.negotiated_features().is_some().to_string()),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(machine.event("driver_ok"), "true");
}

#[test]
fn has_the_queues_it_was_configured_with_once_the_driver_is_ok() {
  let machine = custom_device_machine(
    "has_the_queues_it_was_configured_with_once_the_driver_is_ok",
    READ_HWRNG,
    CustomDevice {
      driver_ok: Box::new(|device| {
        format!(
          "{:?}:{}",
          device.queue(0).map(|queue| queue.queue_index()),
          device.queue(1).is_some()
        )
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(machine.event("driver_ok"), "Some(0):false");
}

#[test]
fn has_no_shared_memory_regions_unless_configured() {
  let machine = custom_device_machine(
    "has_no_shared_memory_regions_unless_configured",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| device.shared_memory_regions().len().to_string()),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), "0");
}

#[test]
fn tells_its_delegate_it_will_pause_resume_and_stop() {
  let machine = custom_device_machine(
    "tells_its_delegate_it_will_pause_resume_and_stop",
    READ_THEN_WAIT,
    CustomDevice::default(),
    |_| {},
  );

  start(&machine.queue, &machine.machine);
  machine.event("element");
  completed(&machine.queue, |completion_handler| {
    machine.machine.pause(completion_handler)
  })
  .unwrap();
  machine.event("will_pause");
  completed(&machine.queue, |completion_handler| {
    machine.machine.resume(completion_handler)
  })
  .unwrap();
  machine.event("will_resume");
  stop(&machine.queue, &machine.machine);
  machine.event("will_stop");
}

#[test]
fn tells_its_delegate_it_will_reset_when_it_asks_to() {
  let machine = custom_device_machine(
    "tells_its_delegate_it_will_reset_when_it_asks_to",
    READ_THEN_WAIT,
    CustomDevice {
      driver_ok: Box::new(|device| {
        device.request_reset();
        String::new()
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  machine.event("will_reset");
}

#[test]
fn updates_its_device_specific_configuration() {
  let (sender, receiver) = mpsc::channel();
  let sender = Mutex::new(sender);
  let machine = custom_device_machine(
    "updates_its_device_specific_configuration",
    READ_THEN_WAIT,
    CustomDevice {
      driver_ok: Box::new(move |device| {
        let sender: Sender<_> = sender.lock().unwrap().clone();

        device.update(
          &VirtioDeviceSpecificConfiguration::new(&[5, 6, 7, 8]).unwrap(),
          move |result| {
            sender
              .send(result.map_err(|error| error.to_string()))
              .unwrap();
          },
        );
        String::new()
      }),
      ..CustomDevice::default()
    },
    |configuration| {
      configuration
        .set_device_specific_configuration(Some(&VirtioDeviceSpecificConfiguration::new(&[1, 2, 3, 4]).unwrap()));
    },
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(receiver.recv().unwrap(), Ok(()));
}

/// Gives `configuration` the generic platform with the machine identifier
/// `identifier`, as a machine restored from a save needs the one saved, and
/// lets `device` save and restore.
fn saving(
  identifier: &[u8],
) -> impl FnOnce(&VirtualMachineConfiguration, &CustomVirtioDeviceConfiguration) + Send + use<'_> {
  move |configuration, device| {
    let platform = GenericPlatformConfiguration::new().unwrap();

    platform.set_machine_identifier(
      &GenericMachineIdentifier::with_data_representation(identifier)
        .unwrap()
        .unwrap(),
    );
    configuration.set_platform(&platform);
    device.set_supports_save_restore(true);
  }
}

/// Saves `machine`, paused, once its device's driver is ok.
fn save(machine: &CustomDeviceMachine, url: &Url) -> Result<(), virtualization::Error> {
  let virtual_machine = &machine.machine;

  start(&machine.queue, virtual_machine);
  machine.event("driver_ok");
  completed(&machine.queue, |completion_handler| {
    virtual_machine.pause(completion_handler)
  })
  .unwrap();

  completed(&machine.queue, |completion_handler| {
    virtual_machine
      .save_machine_state_to(url, completion_handler)
      .unwrap();
  })
}

#[test]
fn saves_and_restores_the_state_its_delegate_gives() {
  let name = "saves_and_restores_the_state_its_delegate_gives";
  let url = new_file_url(&format!("{name}.vzvmsave"));
  let identifier = GenericMachineIdentifier::new()
    .unwrap()
    .data_representation();
  let device = || CustomDevice {
    save_state: Some(vec![1, 2, 3]),
    ..CustomDevice::default()
  };
  let saved = custom_device_machine_configured(name, READ_THEN_WAIT, device(), saving(&identifier));

  save(&saved, &url).unwrap();
  saved.event("save_state");
  stop(&saved.queue, &saved.machine);

  let restored = custom_device_machine_configured(name, READ_THEN_WAIT, device(), saving(&identifier));

  completed(&restored.queue, |completion_handler| {
    restored
      .machine
      .restore_machine_state_from(&url, completion_handler)
      .unwrap();
  })
  .unwrap();

  assert_eq!(restored.event("should_restore"), "[1, 2, 3]");
}

#[test]
fn fails_to_save_without_state_from_its_delegate() {
  let name = "fails_to_save_without_state_from_its_delegate";
  let identifier = GenericMachineIdentifier::new()
    .unwrap()
    .data_representation();
  let machine = custom_device_machine_configured(name, READ_THEN_WAIT, CustomDevice::default(), saving(&identifier));
  let error = ns_error(save(&machine, &new_file_url(&format!("{name}.vzvmsave"))));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(ErrorCode::new(error.code).unwrap(), ErrorCode::Save, "{error:#?}");
  machine.event("save_state");
}

struct Delegate;

impl CustomVirtioDeviceDelegate for Delegate {}

/// Whether `device` has the delegate it is given, and then none once set to
/// none, as `has:none`.
fn delegate_given(device: &CustomVirtioDevice) -> String {
  let delegate: Arc<dyn CustomVirtioDeviceDelegate> = Arc::new(Delegate);

  device.set_delegate(Some(&delegate));
  let has = device
    .delegate()
    .is_some_and(|given| Arc::ptr_eq(&given, &delegate));
  device.set_delegate(None);

  format!("{has}:{}", device.delegate().is_none())
}

#[test]
fn has_the_delegate_it_was_given() {
  let machine = custom_device_machine(
    "custom_virtio_device_has_the_delegate_it_was_given",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| delegate_given(device)),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), "true:true");
}

#[test]
fn holds_its_delegate_weakly() {
  let machine = custom_device_machine(
    "custom_virtio_device_holds_its_delegate_weakly",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| {
        let delegate: Arc<dyn CustomVirtioDeviceDelegate> = Arc::new(Delegate);

        device.set_delegate(Some(&delegate));
        drop(delegate);

        device.delegate().is_none().to_string()
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), "true");
}

#[test]
fn a_clone_is_the_same_object() {
  let machine = custom_device_machine(
    "custom_virtio_device_a_clone_is_the_same_object",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| (format!("{:?}", device.clone()) == format!("{device:?}")).to_string()),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert_eq!(machine.event("created"), "true");
}

#[test]
fn debugs_as_its_description() {
  let machine = custom_device_machine(
    "custom_virtio_device_debugs_as_its_description",
    READ_HWRNG,
    CustomDevice {
      created: Box::new(|_, device| format!("{device:?}")),
      ..CustomDevice::default()
    },
    |_| {},
  );

  assert!(
    machine
      .event("created")
      .starts_with("<VZCustomVirtioDevice: 0x")
  );
}
