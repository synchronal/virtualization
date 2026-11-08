use std::net::Ipv4Addr;

use dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::{
  MACAddress, NATNetworkDeviceAttachment, NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration, VirtualMachine,
};
use virtualization_test::{Console, booting_machine, initial_ramdisk, linux_configuration, start};

#[test]
fn has_a_random_locally_administered_mac_address_when_new() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  assert!(device.mac_address().is_locally_administered_address());
  assert!(device.mac_address().is_unicast_address());
  assert_ne!(
    device.mac_address().ethernet_address(),
    VirtioNetworkDeviceConfiguration::new()
      .unwrap()
      .mac_address()
      .ethernet_address()
  );
}

#[test]
fn has_the_mac_address_it_was_given() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  device.set_mac_address(&MACAddress::new([0x02, 0x23, 0x45, 0xab, 0xcd, 0xef]).unwrap());

  assert_eq!(device.mac_address().string(), "02:23:45:ab:cd:ef");
}

#[test]
fn has_no_attachment_when_new() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  assert!(device.attachment::<NATNetworkDeviceAttachment>().is_none());
}

#[test]
fn has_the_attachment_it_was_given() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  device.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));

  assert!(device.attachment::<NATNetworkDeviceAttachment>().is_some());

  device.set_attachment(None);

  assert!(device.attachment::<NATNetworkDeviceAttachment>().is_none());
}

#[test]
fn is_a_network_device_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_network_device_of_the_configuration_it_was_given_to");
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  device.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));
  configuration.set_network_devices(&[&device]);

  let given = configuration.network_devices::<VirtioNetworkDeviceConfiguration>();

  assert_eq!(given.len(), 1);
  assert_eq!(
    given[0].as_ref().unwrap().mac_address().string(),
    device.mac_address().string()
  );
  configuration.validate().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let clone = device.clone();

  device.set_mac_address(&MACAddress::new([0x02, 0, 0, 0, 0, 1]).unwrap());

  assert_eq!(clone.mac_address().string(), "02:00:00:00:00:01");
}

#[test]
fn debugs_as_its_description() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  assert!(format!("{device:?}").starts_with("<VZVirtioNetworkDeviceConfiguration: 0x"));
}

const MAC_ADDRESS: [u8; 6] = [0x02, 0x23, 0x45, 0xab, 0xcd, 0xef];

/// A started guest running `script` with a NAT-attached virtio network device
/// with `MAC_ADDRESS`.
fn nat_guest(name: &str, script: &str) -> (DispatchRetained<DispatchQueue>, VirtualMachine, Console) {
  let (queue, machine, console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |configuration| {
    let device = VirtioNetworkDeviceConfiguration::new().unwrap();

    device.set_mac_address(&MACAddress::new(MAC_ADDRESS).unwrap());
    device.set_attachment(Some(&NATNetworkDeviceAttachment::new().unwrap()));
    configuration.set_network_devices(&[&device]);
  });

  start(&queue, &machine);

  (queue, machine, console)
}

#[test]
fn a_guest_sees_the_mac_address() {
  let name = "a_guest_sees_the_mac_address";
  let (_queue, _machine, mut console) = nat_guest(name, "echo \"mac:$(cat /sys/class/net/eth0/address)\"");

  assert_eq!(console.read_until("mac:").trim_end(), "mac:02:23:45:ab:cd:ef");
}

/// udhcpc runs this when it has a lease, with the lease in its environment.
const LEASED: &str = r#"cat > /bin/leased <<'EOF'
#!/bin/sh
[ "$1" = bound ] || exit 0
ip addr add "$ip/$mask" dev "$interface"
echo "lease:$ip"
ping -c 1 -W 5 "$router" > /dev/null && echo ping:answered || echo ping:unanswered
EOF
chmod +x /bin/leased
ip link set eth0 up
udhcpc -i eth0 -n -q -s /bin/leased || echo lease:none"#;

#[test]
fn a_guest_gets_a_lease_from_nat_and_reaches_its_router() {
  let name = "a_guest_gets_a_lease_from_nat_and_reaches_its_router";
  let (_queue, _machine, mut console) = nat_guest(name, LEASED);

  let lease = console.read_until("lease:");
  lease
    .trim_end()
    .trim_start_matches("lease:")
    .parse::<Ipv4Addr>()
    .unwrap_or_else(|_| panic!("no lease: {lease:?}"));
  assert_eq!(console.read_until("ping:").trim_end(), "ping:answered");
}
