use virtualization::{
  BridgedNetworkDeviceAttachment, BridgedNetworkInterface, NetworkDeviceConfiguration, VirtioNetworkDeviceConfiguration,
};
use virtualization_test::{linux_configuration, ns_error};

/// An attachment of the host's first bridgeable interface.
fn attachment() -> BridgedNetworkDeviceAttachment {
  BridgedNetworkDeviceAttachment::new(&BridgedNetworkInterface::network_interfaces().unwrap()[0]).unwrap()
}

#[test]
fn has_the_interface_it_was_given() {
  let interface = &BridgedNetworkInterface::network_interfaces().unwrap()[0];
  let attachment = BridgedNetworkDeviceAttachment::new(interface).unwrap();

  assert_eq!(attachment.interface().identifier(), interface.identifier());
}

#[test]
fn fails_validation_without_the_networking_entitlement() {
  let configuration = linux_configuration("fails_validation_without_the_networking_entitlement");
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();

  device.set_attachment(Some(&attachment()));
  configuration.set_network_devices(&[&device]);

  let error = ns_error(configuration.validate());

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
  assert!(
    error
      .localized_failure_reason
      .as_deref()
      .is_some_and(|reason| reason.contains("com.apple.vm.networking")),
    "{error:#?}"
  );
}

#[test]
fn is_the_attachment_of_the_device_it_was_given_to() {
  let device = VirtioNetworkDeviceConfiguration::new().unwrap();
  let attachment = attachment();

  device.set_attachment(Some(&attachment));

  assert_eq!(
    format!(
      "{:?}",
      device
        .attachment::<BridgedNetworkDeviceAttachment>()
        .unwrap()
    ),
    format!("{attachment:?}")
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let attachment = attachment();

  assert_eq!(format!("{:?}", attachment.clone()), format!("{attachment:?}"));
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", attachment()).starts_with("<VZBridgedNetworkDeviceAttachment: 0x"));
}
