use std::io::Write;
use std::sync::{Arc, Mutex};

use virtualization::GuestMemoryMapping;
use virtualization_test::{CustomDevice, custom_device_machine, fill_with_entropy, start};

/// Reports where the guest's RAM starts, waits for a line on the console, then
/// reads `/dev/hwrng` once more.
const RAM_THEN_READ: &str = "echo \"ram:$(grep -m1 'System RAM' /proc/iomem | cut -d- -f1 | tr -d ' ')\"\nread \
                             line\nhead -c 16 /dev/hwrng > /dev/null\nread line";

/// What `inspect` made of the mapping of `length` bytes at the guest's RAM
/// plus `offset`, which the device asks for when it answers the guest's read
/// that follows the guest reporting where its RAM is.
fn mapping(name: &str, offset: u64, length: isize, inspect: fn(Option<GuestMemoryMapping>) -> String) -> String {
  let ram: Arc<Mutex<Option<u64>>> = Arc::default();
  let asked = ram.clone();
  let mut machine = custom_device_machine(
    name,
    RAM_THEN_READ,
    CustomDevice {
      element: Box::new(move |device, _, element| {
        fill_with_entropy(element);

        match *asked.lock().unwrap() {
          Some(ram) => format!("mapping:{}", inspect(device.guest_memory_mapping(ram + offset, length))),
          None => String::new(),
        }
      }),
      ..CustomDevice::default()
    },
    |_| {},
  );

  start(&machine.queue, &machine.machine);
  let line = machine.console.read_until("ram:");
  let start = u64::from_str_radix(line.trim_end().strip_prefix("ram:").unwrap(), 16).unwrap();

  *ram.lock().unwrap() = Some(start);
  writeln!(machine.console.input).unwrap();

  loop {
    if let Some(inspected) = machine.event("element").strip_prefix("mapping:") {
      return inspected.to_owned();
    }
  }
}

#[test]
fn maps_the_guest_memory_it_was_asked_for() {
  let reported = mapping("maps_the_guest_memory_it_was_asked_for", 0x1000, 0x2000, |mapping| {
    let mapping = mapping.unwrap();

    format!(
      "{:#x}:{:#x}:{}",
      mapping.physical_address() & 0xffff,
      mapping.length(),
      mapping.mutable_bytes().is_null()
    )
  });

  assert_eq!(reported, "0x1000:0x2000:false");
}

#[test]
fn maps_nothing_outside_the_guest_memory() {
  let reported = mapping(
    "maps_nothing_outside_the_guest_memory",
    512 * 1024 * 1024,
    0x1000,
    |mapping| mapping.is_none().to_string(),
  );

  assert_eq!(reported, "true");
}

#[test]
fn a_clone_is_the_same_object() {
  let reported = mapping(
    "guest_memory_mapping_a_clone_is_the_same_object",
    0,
    0x1000,
    |mapping| {
      let mapping = mapping.unwrap();

      (format!("{:?}", mapping.clone()) == format!("{mapping:?}")).to_string()
    },
  );

  assert_eq!(reported, "true");
}

#[test]
fn debugs_as_its_description() {
  let reported = mapping("guest_memory_mapping_debugs_as_its_description", 0, 0x1000, |mapping| {
    format!("{:?}", mapping.unwrap())
  });

  assert!(reported.starts_with("<VZGuestMemoryMapping: 0x"), "{reported}");
}
