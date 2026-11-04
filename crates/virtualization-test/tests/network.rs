use std::net::{Ipv4Addr, Ipv6Addr};

use virtualization::vmnet_framework::{Error, Mode, Network, NetworkConfiguration, Return};

fn network(mode: Mode) -> Network {
  Network::new(&NetworkConfiguration::new(mode).unwrap()).unwrap()
}

#[test]
fn has_the_ipv4_subnet_it_was_configured_with() {
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  configuration
    .set_ipv4_subnet(Ipv4Addr::new(192, 168, 200, 0), Ipv4Addr::new(255, 255, 255, 0))
    .unwrap();

  assert_eq!(
    Network::new(&configuration).unwrap().ipv4_subnet(),
    (Ipv4Addr::new(192, 168, 200, 0), Ipv4Addr::new(255, 255, 255, 0))
  );
}

#[test]
fn has_the_ipv6_prefix_it_was_configured_with() {
  let prefix: Ipv6Addr = "fd00:1234:5678:9abc::".parse().unwrap();
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  configuration.set_ipv6_prefix(prefix, 64).unwrap();

  assert_eq!(Network::new(&configuration).unwrap().ipv6_prefix(), (prefix, 64));
}

#[test]
fn has_a_24_bit_subnet_of_192_168_and_a_64_bit_unique_local_prefix_by_default() {
  for mode in [Mode::Host, Mode::Shared] {
    let network = network(mode);
    let (subnet, mask) = network.ipv4_subnet();
    let (prefix, prefix_len) = network.ipv6_prefix();

    assert_eq!(subnet.octets()[..2], [192, 168], "{mode:?}");
    assert_eq!(mask, Ipv4Addr::new(255, 255, 255, 0), "{mode:?}");
    assert!(prefix.is_unique_local(), "{mode:?}");
    assert_eq!(prefix_len, 64, "{mode:?}");
  }
}

#[test]
fn fails_as_vmnet_does_with_an_ipv6_prefix_longer_than_an_address() {
  let configuration = NetworkConfiguration::new(Mode::Host).unwrap();

  configuration
    .set_ipv6_prefix("fd00::".parse().unwrap(), 200)
    .unwrap();

  assert!(matches!(
    Network::new(&configuration),
    Err(Error::Return(Return::Failure))
  ));
}

#[test]
fn a_clone_is_the_same_object() {
  let network = network(Mode::Host);

  assert_eq!(format!("{:?}", network.clone()), format!("{network:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", network(Mode::Host)).starts_with("<vmnet_network 0x"));
}
