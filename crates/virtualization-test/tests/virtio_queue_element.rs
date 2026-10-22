use virtualization::VirtioQueueElement;
use virtualization_test::{CustomDevice, ENTROPY, READ_HWRNG, custom_device_machine, start};

/// What Virtualization fails to read or write more than a buffer holds with:
/// `internalError`.
const INVALID_SIZE: &str = "Internal Virtualization error. Invalid size. (VZErrorDomain 1)";

/// What an element a guest's `virtio_rng` put on its queue reports, after
/// `answer` answers it.
fn answered(name: &str, answer: fn(&VirtioQueueElement) -> String) -> String {
  let machine = custom_device_machine(
    name,
    READ_HWRNG,
    CustomDevice {
      element: Box::new(move |_, _, element| answer(element)),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);

  machine.event("element")
}

/// `virtio_rng` asks for random bytes: an element of write buffers only.
#[test]
fn has_only_write_buffers_from_virtio_rng() {
  let reported = answered("has_only_write_buffers_from_virtio_rng", |element| {
    let reported = format!(
      "{}:{}:{}:{}:{}",
      element.read_buffers_byte_count(),
      element.read_buffers_available_byte_count(),
      element.write_buffers_byte_count(),
      element.write_buffers_available_byte_count(),
      element.written_byte_count()
    );

    element.write(&[ENTROPY]).unwrap();
    reported
  });

  assert_eq!(reported, "0:0:64:64:0");
}

#[test]
fn counts_what_is_written() {
  let reported = answered("counts_what_is_written", |element| {
    element.write(&[ENTROPY; 4]).unwrap();
    element.write_buffer(&[ENTROPY; 6]).unwrap();

    format!(
      "{}:{}",
      element.write_buffers_available_byte_count(),
      element.written_byte_count()
    )
  });

  assert_eq!(reported, "54:10");
}

#[test]
fn fails_to_write_more_than_is_available() {
  let reported = answered("fails_to_write_more_than_is_available", |element| {
    let available = element.write_buffers_available_byte_count().cast_unsigned();
    let failed = element.write(&vec![ENTROPY; available + 1]).unwrap_err();

    element.write(&[ENTROPY]).unwrap();
    failed.to_string()
  });

  assert_eq!(reported, INVALID_SIZE);
}

#[test]
fn fails_to_write_a_buffer_larger_than_is_available() {
  let reported = answered("fails_to_write_a_buffer_larger_than_is_available", |element| {
    let available = element.write_buffers_available_byte_count().cast_unsigned();
    let failed = element
      .write_buffer(&vec![ENTROPY; available + 1])
      .unwrap_err();

    element.write(&[ENTROPY]).unwrap();
    failed.to_string()
  });

  assert_eq!(reported, INVALID_SIZE);
}

#[test]
fn has_no_read_buffers_to_read_from_virtio_rng() {
  let reported = answered("has_no_read_buffers_to_read_from_virtio_rng", |element| {
    let mut buffer = [0; 1];
    let reported = format!(
      "{:?}:{}:{}:{}",
      element.read_buffers(),
      element.read_bytes(1).unwrap_err(),
      element.peek_into_read_buffers(1).unwrap_err(),
      element.read_bytes_into_buffer(&mut buffer).unwrap_err()
    );

    element.write(&[ENTROPY]).unwrap();
    reported
  });

  assert_eq!(reported, format!("[]:{INVALID_SIZE}:{INVALID_SIZE}:{INVALID_SIZE}"));
}

#[test]
fn reads_nothing_of_no_bytes() {
  let reported = answered("reads_nothing_of_no_bytes", |element| {
    let reported = format!(
      "{:?}:{:?}:{:?}",
      element.read_bytes(0),
      element.peek_into_read_buffers(0),
      element.read_bytes_into_buffer(&mut [])
    );

    element.write(&[ENTROPY]).unwrap();
    reported
  });

  assert_eq!(reported, "Ok([]):Ok([]):Ok(())");
}

#[test]
fn a_clone_is_the_same_object() {
  let reported = answered("virtio_queue_element_a_clone_is_the_same_object", |element| {
    element.clone().write(&[ENTROPY; 3]).unwrap();

    element.written_byte_count().to_string()
  });

  assert_eq!(reported, "3");
}

#[test]
fn debugs_as_its_description() {
  let reported = answered("virtio_queue_element_debugs_as_its_description", |element| {
    element.write(&[ENTROPY]).unwrap();

    format!("{element:?}")
  });

  assert!(reported.starts_with("<VZVirtioQueueElement: 0x"), "{reported}");
}
