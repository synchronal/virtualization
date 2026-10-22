use virtualization_test::{CustomDevice, READ_HWRNG, custom_device_machine, fill_with_entropy, start};

#[test]
fn is_the_queue_an_element_came_from() {
  let machine = custom_device_machine(
    "is_the_queue_an_element_came_from",
    READ_HWRNG,
    CustomDevice {
      element: Box::new(|_, queue, element| {
        fill_with_entropy(element);
        format!("{}:{}", queue.queue_index(), queue.queue_size())
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(machine.event("element"), "0:256");
}

#[test]
fn a_clone_is_the_same_object() {
  let machine = custom_device_machine(
    "virtio_queue_a_clone_is_the_same_object",
    READ_HWRNG,
    CustomDevice {
      driver_ok: Box::new(|device| {
        let queue = device.queue(0).unwrap();

        (format!("{:?}", queue.clone()) == format!("{queue:?}")).to_string()
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert_eq!(machine.event("driver_ok"), "true");
}

#[test]
fn debugs_as_its_description() {
  let machine = custom_device_machine(
    "virtio_queue_debugs_as_its_description",
    READ_HWRNG,
    CustomDevice {
      driver_ok: Box::new(|device| format!("{:?}", device.queue(0).unwrap())),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  assert!(machine.event("driver_ok").starts_with("<VZVirtioQueue: 0x"));
}
