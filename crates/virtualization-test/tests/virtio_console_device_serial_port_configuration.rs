use virtualization::{
  FileHandleSerialPortAttachment, FileSerialPortAttachment, SerialPortConfiguration,
  VirtioConsoleDeviceSerialPortConfiguration,
};
use virtualization_test::{linux_configuration, new_file_url};

#[test]
fn has_no_attachment_when_new() {
  let port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();

  assert!(port.attachment::<FileSerialPortAttachment>().is_none());
}

#[test]
fn has_the_attachment_it_was_given() {
  let port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();
  let url = new_file_url("has_the_attachment_it_was_given.log");

  port.set_attachment(Some(&FileSerialPortAttachment::new(&url, false).unwrap()));

  assert_eq!(
    port
      .attachment::<FileSerialPortAttachment>()
      .unwrap()
      .url()
      .unwrap(),
    url
  );
  assert!(
    port
      .attachment::<FileHandleSerialPortAttachment>()
      .is_none()
  );

  port.set_attachment(None);

  assert!(port.attachment::<FileSerialPortAttachment>().is_none());
}

#[test]
fn is_a_serial_port_of_the_configuration_it_was_given_to() {
  let configuration = linux_configuration("is_a_serial_port_of_the_configuration_it_was_given_to");
  let port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();
  let url = new_file_url("is_a_serial_port_of_the_configuration_it_was_given_to.log");

  configuration.set_serial_ports(&[&port]);
  port.set_attachment(Some(&FileSerialPortAttachment::new(&url, false).unwrap()));

  let given = configuration.serial_ports::<VirtioConsoleDeviceSerialPortConfiguration>();

  assert_eq!(given.len(), 1);
  assert_eq!(
    given[0]
      .as_ref()
      .unwrap()
      .attachment::<FileSerialPortAttachment>()
      .unwrap()
      .url()
      .unwrap(),
    url
  );
  configuration.validate().unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();
  let clone = port.clone();

  port.set_attachment(Some(&FileHandleSerialPortAttachment::new(None, None).unwrap()));
  drop(port);

  assert!(
    clone
      .attachment::<FileHandleSerialPortAttachment>()
      .is_some()
  );
}

#[test]
fn debugs_as_its_description() {
  let port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();

  assert!(format!("{port:?}").starts_with("<VZVirtioConsoleDeviceSerialPortConfiguration: 0x"));
}
