use std::net::{IpAddr, Ipv4Addr};

use virtualization::NetworkDeviceConfiguration;
use virtualization::vmnet_framework::{Error, Mode, Network, NetworkConfiguration, Return};
use virtualization_test::guest_network;

const IPPROTO_ICMP: u8 = 1;
const IPPROTO_TCP: u8 = 6;

#[test]
fn makes_a_configuration_in_host_and_shared_modes() {
  NetworkConfiguration::new(Mode::Host).unwrap();
  NetworkConfiguration::new(Mode::Shared).unwrap();
}

#[test]
fn fails_as_vmnet_does_in_bridged_mode() {
  assert!(matches!(
    NetworkConfiguration::new(Mode::Bridged),
    Err(Error::Return(Return::Failure))
  ));
}

#[test]
fn fails_as_vmnet_does_in_a_mode_it_has_no_case_for() {
  assert!(matches!(
    NetworkConfiguration::new(Mode::Unknown(42)),
    Err(Error::Return(Return::Failure))
  ));
}

#[test]
fn accepts_an_invalid_subnet_and_leaves_checking_it_to_network_creation() {
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  configuration
    .set_ipv4_subnet(Ipv4Addr::new(8, 8, 8, 0), Ipv4Addr::new(255, 0, 255, 0))
    .unwrap();

  assert!(matches!(
    Network::new(&configuration),
    Err(Error::Return(Return::Failure))
  ));
}

#[test]
fn makes_a_network_with_a_port_forwarding_rule() {
  let configuration = NetworkConfiguration::new(Mode::Shared).unwrap();

  configuration
    .add_port_forwarding_rule(IPPROTO_TCP, 80, 18080, IpAddr::V4(Ipv4Addr::new(192, 168, 65, 10)))
    .unwrap();
  configuration
    .add_port_forwarding_rule(IPPROTO_TCP, 80, 18081, "fd00::10".parse().unwrap())
    .unwrap();

  Network::new(&configuration).unwrap();
}

#[test]
fn fails_as_vmnet_does_to_make_a_network_forwarding_a_protocol_other_than_tcp_or_udp() {
  let configuration = NetworkConfiguration::new(Mode::Shared).unwrap();

  configuration
    .add_port_forwarding_rule(IPPROTO_ICMP, 80, 18082, IpAddr::V4(Ipv4Addr::new(192, 168, 65, 10)))
    .unwrap();

  assert!(matches!(
    Network::new(&configuration),
    Err(Error::Return(Return::Failure))
  ));
}

#[test]
fn makes_a_network_on_its_external_interface() {
  let configuration = NetworkConfiguration::new(Mode::Shared).unwrap();

  configuration.set_external_interface("en0").unwrap();

  Network::new(&configuration).unwrap();
}

#[test]
fn makes_a_network_with_each_service_disabled() {
  let disables: [fn(&NetworkConfiguration); 5] = [
    NetworkConfiguration::disable_router_advertisement,
    NetworkConfiguration::disable_nat66,
    NetworkConfiguration::disable_nat44,
    NetworkConfiguration::disable_dns_proxy,
    NetworkConfiguration::disable_dhcp,
  ];

  for (index, disable) in disables.into_iter().enumerate() {
    let configuration = NetworkConfiguration::new(Mode::Shared).unwrap();

    disable(&configuration);

    assert!(Network::new(&configuration).is_ok(), "disable #{index}");
  }
}

#[test]
fn gives_a_guest_its_dhcp_reservation() {
  let reservation = Ipv4Addr::new(192, 168, 200, 50);
  let guest = guest_network("gives_a_guest_its_dhcp_reservation", |configuration, device| {
    configuration
      .set_ipv4_subnet(Ipv4Addr::new(192, 168, 200, 1), Ipv4Addr::new(255, 255, 255, 0))
      .unwrap();
    configuration
      .add_dhcp_reservation(device.mac_address().ethernet_address(), reservation)
      .unwrap();
  });

  assert_eq!(guest.lease, Some(reservation));
}

#[test]
fn a_guest_gets_no_lease_with_dhcp_disabled() {
  let guest = guest_network("a_guest_gets_no_lease_with_dhcp_disabled", |configuration, _| {
    configuration.disable_dhcp();
  });

  assert_eq!(guest.lease, None);
}

#[test]
fn gives_a_guest_its_mtu() {
  let guest = guest_network("gives_a_guest_its_mtu", |configuration, _| {
    configuration.set_mtu(9000).unwrap();
  });

  assert_eq!(guest.mtu, 9000);
}

#[test]
fn a_clone_is_the_same_object() {
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  assert_eq!(format!("{:?}", configuration.clone()), format!("{configuration:?}"));
}

#[test]
fn debugs_as_its_description() {
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  assert!(format!("{configuration:?}").starts_with("<vmnet_network_configuration 0x"));
}
