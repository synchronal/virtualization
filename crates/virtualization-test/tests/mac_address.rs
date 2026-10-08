use virtualization::MACAddress;

#[test]
fn has_the_ethernet_address_it_was_made_with() {
  let address = MACAddress::new([0x02, 0x23, 0x45, 0xab, 0xcd, 0xef]).unwrap();

  assert_eq!(address.ethernet_address(), [0x02, 0x23, 0x45, 0xab, 0xcd, 0xef]);
  assert_eq!(address.string(), "02:23:45:ab:cd:ef");
}

#[test]
fn reads_a_string_in_either_case() {
  let address = MACAddress::with_string("02:23:45:AB:cd:EF")
    .unwrap()
    .unwrap();

  assert_eq!(address.ethernet_address(), [0x02, 0x23, 0x45, 0xab, 0xcd, 0xef]);
  assert_eq!(address.string(), "02:23:45:ab:cd:ef");
}

#[test]
fn is_none_from_a_malformed_string() {
  assert!(MACAddress::with_string("02:23:45:ab:cd").unwrap().is_none());
  assert!(MACAddress::with_string("not an address").unwrap().is_none());
}

#[test]
fn is_random_unicast_and_locally_administered_when_random() {
  let address = MACAddress::random_locally_administered().unwrap();

  assert!(address.is_unicast_address());
  assert!(address.is_locally_administered_address());
  assert!(!address.is_universally_administered_address());
  assert!(!address.is_multicast_address());
  assert!(!address.is_broadcast_address());
  assert_ne!(
    address.ethernet_address(),
    MACAddress::random_locally_administered()
      .unwrap()
      .ethernet_address()
  );
}

/// Virtualization's broadcast address isn't also multicast, though its group
/// bit is set.
#[test]
fn is_broadcast_when_all_ones() {
  let address = MACAddress::new([0xff; 6]).unwrap();

  assert_eq!(
    (
      address.is_broadcast_address(),
      address.is_multicast_address(),
      address.is_unicast_address()
    ),
    (true, false, false)
  );
}

#[test]
fn is_multicast_with_the_group_bit() {
  let address = MACAddress::new([0x01, 0, 0x5e, 0, 0, 1]).unwrap();

  assert!(address.is_multicast_address());
  assert!(!address.is_broadcast_address());
  assert!(!address.is_unicast_address());
}

#[test]
fn is_universally_administered_without_the_local_bit() {
  let address = MACAddress::new([0x00, 0x1b, 0x63, 0x84, 0x45, 0xe6]).unwrap();

  assert!(address.is_universally_administered_address());
  assert!(!address.is_locally_administered_address());
  assert!(address.is_unicast_address());
}

#[test]
fn a_clone_is_the_same_address() {
  let address = MACAddress::random_locally_administered().unwrap();

  assert_eq!(address.clone().ethernet_address(), address.ethernet_address());
}

/// Virtualization's description of a MAC address is its string.
#[test]
fn debugs_as_its_description() {
  let address = MACAddress::new([0x02, 0x23, 0x45, 0xab, 0xcd, 0xef]).unwrap();

  assert_eq!(format!("{address:?}"), "02:23:45:ab:cd:ef");
}
