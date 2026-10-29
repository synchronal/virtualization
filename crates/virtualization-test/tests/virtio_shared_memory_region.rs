use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use virtualization::{
  CustomVirtioDevice, MemoryPages, VirtioSharedMemoryRegion, VirtioSharedMemoryRegionConfiguration, ns_page_size,
};
use virtualization_test::{CustomDevice, CustomDeviceMachine, READ_HWRNG, custom_device_machine, start};

/// The size of the region a device is configured with.
const REGION_SIZE: u64 = 0x40_0000;

/// What `inspect` reported of the device's region, from the device's
/// configuration with one region, ID 3, of `REGION_SIZE`. Once the machine
/// runs, if `running`; when it's made, if not.
fn region(
  name: &str,
  running: bool,
  inspect: impl Fn(&CustomVirtioDevice) -> String + Send + Sync + 'static,
) -> String {
  region_machine(name, running, inspect).0
}

/// As `region`, and the machine.
fn region_machine(
  name: &str,
  running: bool,
  inspect: impl Fn(&CustomVirtioDevice) -> String + Send + Sync + 'static,
) -> (String, CustomDeviceMachine) {
  let inspect: Arc<dyn Fn(&CustomVirtioDevice) -> String + Send + Sync> = Arc::new(inspect);
  let created = inspect.clone();
  let machine = custom_device_machine(
    name,
    READ_HWRNG,
    if running {
      CustomDevice {
        driver_ok: Box::new(move |device| inspect(device)),
        ..CustomDevice::default()
      }
    } else {
      CustomDevice {
        created: Box::new(move |_, device| created(device)),
        ..CustomDevice::default()
      }
    },
    |configuration| {
      configuration.set_shared_memory_regions(&[&VirtioSharedMemoryRegionConfiguration::new(3, REGION_SIZE).unwrap()]);
    },
  );

  if running {
    start(&machine.queue, &machine.machine);
    (machine.event("driver_ok"), machine)
  } else {
    (machine.event("created"), machine)
  }
}

#[test]
fn is_the_region_its_device_was_configured_with() {
  let reported = region("is_the_region_its_device_was_configured_with", false, |device| {
    let regions = device.shared_memory_regions();

    format!("{}:{}:{:#x}", regions.len(), regions[0].region_id(), regions[0].size())
  });

  assert_eq!(reported, format!("1:3:{REGION_SIZE:#x}"));
}

#[test]
fn is_the_same_region_each_time() {
  let reported = region("is_the_same_region_each_time", false, |device| {
    (format!("{:?}", device.shared_memory_regions()[0]) == format!("{:?}", device.shared_memory_regions()[0]))
      .to_string()
  });

  assert_eq!(reported, "true");
}

/// The first `count` things `operate` told its sender, given the device's
/// region once the machine runs.
fn operated(name: &str, count: usize, operate: fn(VirtioSharedMemoryRegion, Sender<String>)) -> Vec<String> {
  let (sender, receiver) = mpsc::channel();
  let sender = Mutex::new(sender);
  let (_, _machine) = region_machine(name, true, move |device| {
    operate(device.shared_memory_regions().remove(0), sender.lock().unwrap().clone());
    String::new()
  });

  (0..count)
    .map(|_| receiver.recv_timeout(Duration::from_secs(30)).unwrap())
    .collect()
}

/// A page of `MemoryPages`.
fn page() -> MemoryPages {
  MemoryPages::new(ns_page_size().unwrap()).unwrap()
}

#[test]
fn maps_and_unmaps_memory() {
  let reported = operated("maps_and_unmaps_memory", 2, |region, sender| {
    let size = ns_page_size().unwrap() as u64;
    let unmapping = region.clone();

    region.map_memory(&page(), size, move |mapped| {
      let _ = sender.send(format!("mapped:{mapped:?}"));
      unmapping.unmap_memory(size, size, move |unmapped| {
        let _ = sender.send(format!("unmapped:{unmapped:?}"));
      });
    });
  });

  assert_eq!(reported, ["mapped:Ok(())", "unmapped:Ok(())"]);
}

/// The first pages stay mapped, held by the region alone: the test keeps no
/// clone of either.
#[test]
fn fails_to_map_over_what_is_mapped() {
  let reported = operated("fails_to_map_over_what_is_mapped", 3, |region, sender| {
    let size = ns_page_size().unwrap() as u64;
    let again = region.clone();

    region.map_memory(&page(), size, move |mapped| {
      let _ = sender.send(format!("mapped:{mapped:?}"));
      again.clone().map_memory(&page(), size, move |remapped| {
        let _ = sender.send(remapped.unwrap_err().to_string());
        again.unmap_memory(size, size, move |unmapped| {
          let _ = sender.send(format!("unmapped:{unmapped:?}"));
        });
      });
    });
  });

  assert_eq!(
    reported,
    [
      "mapped:Ok(())",
      "An unsupported operation. Failed to map Virtio shared memory. (VZErrorDomain 10)",
      "unmapped:Ok(())",
    ]
  );
}

/// Mapping where an unmap still in flight covers keeps the new pages mapped,
/// held by the region alone, once the unmap completes.
#[test]
fn maps_where_an_unmap_in_flight_covers() {
  let reported = operated("maps_where_an_unmap_in_flight_covers", 4, |region, sender| {
    let size = ns_page_size().unwrap() as u64;
    let again = region.clone();

    region.map_memory(&page(), size, move |mapped| {
      let _ = sender.send(format!("mapped:{mapped:?}"));
      let unmapped_sender = sender.clone();
      again.unmap_memory(size, size, move |unmapped| {
        let _ = unmapped_sender.send(format!("unmapped:{unmapped:?}"));
      });
      let finally = again.clone();
      again.map_memory(&page(), size, move |remapped| {
        let _ = sender.send(format!("remapped:{remapped:?}"));
        finally.unmap_memory(size, size, move |unmapped| {
          let _ = sender.send(format!("unmapped:{unmapped:?}"));
        });
      });
    });
  });

  assert_eq!(
    reported,
    ["mapped:Ok(())", "unmapped:Ok(())", "remapped:Ok(())", "unmapped:Ok(())"]
  );
}

#[test]
fn fails_to_unmap_what_isnt_mapped() {
  let reported = operated("fails_to_unmap_what_isnt_mapped", 1, |region, sender| {
    region.unmap_memory(0, ns_page_size().unwrap() as u64, move |unmapped| {
      let _ = sender.send(unmapped.unwrap_err().to_string());
    });
  });

  assert_eq!(
    reported,
    ["An unsupported operation. Failed to unmap Virtio shared memory. (VZErrorDomain 10)"]
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let reported = region(
    "virtio_shared_memory_region_a_clone_is_the_same_object",
    false,
    |device| {
      let region = device.shared_memory_regions().remove(0);

      (format!("{:?}", region.clone()) == format!("{region:?}")).to_string()
    },
  );

  assert_eq!(reported, "true");
}

#[test]
fn debugs_as_its_description() {
  let reported = region(
    "virtio_shared_memory_region_debugs_as_its_description",
    false,
    |device| format!("{:?}", device.shared_memory_regions()[0]),
  );

  assert!(reported.starts_with("<VZVirtioSharedMemoryRegion: 0x"), "{reported}");
}
