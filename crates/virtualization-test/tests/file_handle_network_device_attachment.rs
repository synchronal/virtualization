use std::io::Write;
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixDatagram;
use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::{
  Error, ErrorCode, FileHandleNetworkDeviceAttachment, NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration,
  VirtualMachine,
};
use virtualization_test::{Console, booting_machine, initial_ramdisk, linux_configuration, ns_error, start};

/// An attachment of one end of a datagram socket pair, and the other end.
fn attachment() -> (FileHandleNetworkDeviceAttachment, UnixDatagram) {
  let (guest, host) = UnixDatagram::pair().unwrap();

  (
    FileHandleNetworkDeviceAttachment::new(OwnedFd::from(guest)).unwrap(),
    host,
  )
}

#[test]
fn has_a_maximum_transmission_unit_of_1500_when_new() {
  let (attachment, _host) = attachment();

  assert_eq!(attachment.maximum_transmission_unit(), 1500);
}

#[test]
fn has_the_maximum_transmission_unit_it_was_given() {
  let (attachment, _host) = attachment();

  attachment.set_maximum_transmission_unit(9000);

  assert_eq!(attachment.maximum_transmission_unit(), 9000);
}

#[test]
fn its_file_handle_is_the_socket_it_was_given() {
  let (attachment, host) = attachment();
  let socket = UnixDatagram::from(attachment.file_handle().try_clone_to_owned().unwrap());

  socket.send(b"frame").unwrap();
  let mut received = [0; 5];

  assert_eq!(host.recv(&mut received).unwrap(), 5);
  assert_eq!(&received, b"frame");
}

/// Validates a configuration with a Virtio network device whose attachment
/// has `maximum_transmission_unit`.
fn validate_with_maximum_transmission_unit(name: &str, maximum_transmission_unit: isize) -> Result<(), Error> {
  let configuration = linux_configuration(name);
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let (attachment, _host) = attachment();

  attachment.set_maximum_transmission_unit(maximum_transmission_unit);
  device.set_attachment(Some(&attachment));
  configuration.set_network_devices(&[&device]);

  configuration.validate()
}

#[test]
fn validates_with_a_maximum_transmission_unit_from_1500_to_65535() {
  validate_with_maximum_transmission_unit("validates_with_mtu_1500", 1500).unwrap();
  validate_with_maximum_transmission_unit("validates_with_mtu_65535", 65535).unwrap();
}

#[test]
fn fails_validation_with_a_maximum_transmission_unit_outside_1500_to_65535() {
  for (name, maximum_transmission_unit) in [("fails_with_mtu_1499", 1499), ("fails_with_mtu_65536", 65536)] {
    let error = ns_error(validate_with_maximum_transmission_unit(name, maximum_transmission_unit));

    assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
    assert_eq!(
      ErrorCode::new(error.code).unwrap(),
      ErrorCode::InvalidVirtualMachineConfiguration,
      "{error:#?}"
    );
  }
}

/// A started machine with a Virtio network device whose attachment is one
/// end of a socket pair.
struct NetworkedMachine {
  _queue: DispatchRetained<DispatchQueue>,
  _machine: VirtualMachine,
  mac_address: [u8; 6],
  /// The other end, which waits a second at most for the guest's frames.
  host: UnixDatagram,
  console: Console,
}

fn networked_machine(name: &str, maximum_transmission_unit: isize, script: &str) -> NetworkedMachine {
  let (guest, host) = UnixDatagram::pair().unwrap();
  let mut mac_address = [0; 6];
  let (queue, machine, console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |configuration| {
    let attachment = FileHandleNetworkDeviceAttachment::new(OwnedFd::from(guest)).unwrap();
    let device = VirtioNetworkDeviceConfiguration::new().unwrap();

    attachment.set_maximum_transmission_unit(maximum_transmission_unit);
    device.set_attachment(Some(&attachment));
    mac_address = device.mac_address().ethernet_address();
    configuration.set_network_devices(&[&device]);
  });

  host.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
  start(&queue, &machine);

  NetworkedMachine {
    _queue: queue,
    _machine: machine,
    mac_address,
    host,
    console,
  }
}

#[test]
fn sends_the_guests_frames_to_its_file_handle() {
  let name = "sends_the_guests_frames_to_its_file_handle";
  let mut networked = networked_machine(
    name,
    1500,
    "ip link set eth0 up\nip addr add 10.0.0.2/24 dev eth0\necho ready\nwhile read line; do ping -c 1 -W 1 10.0.0.1 \
     >/dev/null; echo pinged; done",
  );

  networked.console.read_until("ready");
  let mut frame = [0; 1514];

  for _ in 0..10 {
    writeln!(networked.console.input).unwrap();
    while let Ok(length) = networked.host.recv(&mut frame) {
      // An ARP request for 10.0.0.1, from the guest's address.
      if length >= 14 && frame[12..14] == [0x08, 0x06] {
        assert_eq!(frame[0..6], [0xff; 6]);
        assert_eq!(frame[6..12], networked.mac_address);
        return;
      }
    }
  }

  panic!("the guest's ARP request never arrived");
}

#[test]
fn delivers_its_file_handles_frames_to_the_guest() {
  let name = "delivers_its_file_handles_frames_to_the_guest";
  let mut networked = networked_machine(
    name,
    1500,
    "ip link set eth0 up\necho ready\nwhile read line; do echo \"rx:$(cat \
     /sys/class/net/eth0/statistics/rx_packets)\"; done",
  );

  networked.console.read_until("ready");
  // A broadcast frame of the local experimental EtherType, from a locally
  // administered address, padded to Ethernet's minimum.
  let mut frame = [0; 60];

  frame[0..6].copy_from_slice(&[0xff; 6]);
  frame[6..12].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x01]);
  frame[12..14].copy_from_slice(&[0x88, 0xb5]);
  networked.host.send(&frame).unwrap();

  networked
    .console
    .ask_until("rx:", |received| received != "0");
}

#[test]
fn gives_the_guest_its_maximum_transmission_unit() {
  let name = "gives_the_guest_its_maximum_transmission_unit";
  let mut networked = networked_machine(name, 9000, "echo \"mtu:$(cat /sys/class/net/eth0/mtu)\"");

  assert_eq!(networked.console.read_until("mtu:").trim_end(), "mtu:9000");
}

#[test]
fn is_the_attachment_of_the_device_it_was_given_to() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let (attachment, _host) = attachment();

  device.set_attachment(Some(&attachment));

  assert_eq!(
    format!(
      "{:?}",
      device
        .attachment::<FileHandleNetworkDeviceAttachment>()
        .unwrap()
    ),
    format!("{attachment:?}")
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let (attachment, _host) = attachment();

  attachment.clone().set_maximum_transmission_unit(9000);

  assert_eq!(attachment.maximum_transmission_unit(), 9000);
}

#[test]
fn debugs_as_its_description() {
  let (attachment, _host) = attachment();

  assert!(format!("{attachment:?}").starts_with("<VZFileHandleNetworkDeviceAttachment: 0x"));
}
