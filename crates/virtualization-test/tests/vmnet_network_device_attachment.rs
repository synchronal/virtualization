use virtualization::vmnet_framework::{Mode, Network, NetworkConfiguration};
use virtualization::{NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration, VmnetNetworkDeviceAttachment};
use virtualization_test::{guest_network, linux_configuration};

fn network() -> Network {
  Network::new(&NetworkConfiguration::new(Mode::Host).unwrap()).unwrap()
}

#[test]
fn keeps_the_network_it_was_given() {
  let network = network();
  let given = format!("{network:?}");
  let attachment = VmnetNetworkDeviceAttachment::new(&network).unwrap();

  drop(network);

  assert_eq!(format!("{:?}", attachment.network()), given);
}

#[test]
fn validates_in_a_configuration() {
  let configuration = linux_configuration("vmnet_network_device_attachment_validates_in_a_configuration");
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  device.set_attachment(Some(&VmnetNetworkDeviceAttachment::new(&network()).unwrap()));
  configuration.set_network_devices(&[&device]);

  configuration.validate().unwrap();
}

#[test]
fn a_guest_takes_a_dhcp_lease_in_the_networks_subnet() {
  let guest = guest_network("a_guest_takes_a_dhcp_lease_in_the_networks_subnet", |_, _| {});
  let lease = guest.lease.expect("the guest got no lease").octets();
  let (subnet, mask) = (guest.subnet.0.octets(), guest.subnet.1.octets());

  assert!(
    (0..4).all(|index| lease[index] & mask[index] == subnet[index] & mask[index]),
    "{:?} outside {:?}",
    guest.lease,
    guest.subnet
  );
}

#[test]
fn is_the_attachment_of_the_device_it_was_given_to() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let attachment = VmnetNetworkDeviceAttachment::new(&network()).unwrap();

  device.set_attachment(Some(&attachment));

  assert_eq!(
    format!("{:?}", device.attachment::<VmnetNetworkDeviceAttachment>().unwrap()),
    format!("{attachment:?}")
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let attachment = VmnetNetworkDeviceAttachment::new(&network()).unwrap();

  assert_eq!(format!("{:?}", attachment.clone()), format!("{attachment:?}"));
}

#[test]
fn debugs_as_its_description() {
  let attachment = VmnetNetworkDeviceAttachment::new(&network()).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZVmnetNetworkDeviceAttachment: 0x"));
}
