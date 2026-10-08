use virtualization::{NATNetworkDeviceAttachment, NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration};

#[test]
fn is_the_attachment_of_the_device_it_was_given_to() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let attachment = NATNetworkDeviceAttachment::new().unwrap();

  device.set_attachment(Some(&attachment));

  assert_eq!(
    format!("{:?}", device.attachment::<NATNetworkDeviceAttachment>().unwrap()),
    format!("{attachment:?}")
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let attachment = NATNetworkDeviceAttachment::new().unwrap();

  assert_eq!(format!("{:?}", attachment.clone()), format!("{attachment:?}"));
}

#[test]
fn debugs_as_its_description() {
  let attachment = NATNetworkDeviceAttachment::new().unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZNATNetworkDeviceAttachment: 0x"));
}
