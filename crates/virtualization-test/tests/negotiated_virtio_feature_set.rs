use virtualization::NegotiatedVirtioFeatureSet;
use virtualization_test::{CustomDevice, READ_HWRNG, custom_device_machine, start};

/// What `inspect` made of the features the device negotiated with the
/// guest's `virtio_rng`, after offering `optional` as its optional features'
/// `subset0`.
fn negotiated(name: &str, optional: u32, inspect: fn(&NegotiatedVirtioFeatureSet) -> String) -> String {
  let machine = custom_device_machine(
    name,
    READ_HWRNG,
    CustomDevice {
      driver_ok: Box::new(move |device| inspect(&device.negotiated_features().unwrap())),
      ..CustomDevice::default()
    },
    |configuration| configuration.optional_features().set_subset0(optional),
  );

  start(&machine.queue, &machine.machine);

  machine.event("driver_ok")
}

/// `VIRTIO_F_VERSION_1` (bit 32), and the optional features Virtualization
/// always offers, which `virtio_rng` accepts: indirect descriptors, event
/// indexes, and bit 34.
#[test]
fn has_what_the_driver_accepted() {
  let reported = negotiated("has_what_the_driver_accepted", 0, |features| {
    format!("{:#x}:{:#x}", features.subset0(), features.subset1())
  });

  assert_eq!(reported, "0x30000000:0x5");
}

/// `virtio_rng` knows no device-specific feature, so declines bit 0.
#[test]
fn leaves_out_optional_features_the_driver_declines() {
  let reported = negotiated("leaves_out_optional_features_the_driver_declines", 1, |features| {
    format!("{:#x}", features.subset0())
  });

  assert_eq!(reported, "0x30000000");
}

#[test]
fn a_clone_is_the_same_object() {
  let reported = negotiated(
    "negotiated_virtio_feature_set_a_clone_is_the_same_object",
    0,
    |features| (format!("{:?}", features.clone()) == format!("{features:?}")).to_string(),
  );

  assert_eq!(reported, "true");
}

#[test]
fn debugs_as_its_description() {
  let reported = negotiated(
    "negotiated_virtio_feature_set_debugs_as_its_description",
    0,
    |features| format!("{features:?}"),
  );

  assert!(reported.starts_with("<VZNegotiatedVirtioFeatureSet: 0x"), "{reported}");
}
