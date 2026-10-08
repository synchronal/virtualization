use virtualization::{ConsolePortConfiguration, FileHandleSerialPortAttachment, VirtioConsolePortConfiguration};

#[test]
fn has_no_name_at_first() {
  assert_eq!(VirtioConsolePortConfiguration::new().unwrap().name(), None);
}

#[test]
fn keeps_its_name() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some("port"));

  assert_eq!(port.name().as_deref(), Some("port"));
}

#[test]
fn loses_its_name() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some("port"));
  port.set_name(None);

  assert_eq!(port.name(), None);
}

#[test]
fn is_not_a_console_at_first() {
  assert!(!VirtioConsolePortConfiguration::new().unwrap().is_console());
}

#[test]
fn keeps_whether_it_is_a_console() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_is_console(true);

  assert!(port.is_console());
}

#[test]
fn has_no_attachment_at_first() {
  assert!(
    VirtioConsolePortConfiguration::new()
      .unwrap()
      .attachment::<FileHandleSerialPortAttachment>()
      .is_none()
  );
}

#[test]
fn keeps_its_attachment() {
  let port = VirtioConsolePortConfiguration::new().unwrap();
  let attachment = FileHandleSerialPortAttachment::new(None, None).unwrap();

  port.set_attachment(Some(&attachment));

  assert_eq!(
    format!("{:?}", port.attachment::<FileHandleSerialPortAttachment>().unwrap()),
    format!("{attachment:?}")
  );
}

#[test]
fn loses_its_attachment() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_attachment(Some(&FileHandleSerialPortAttachment::new(None, None).unwrap()));
  port.set_attachment(None);

  assert!(
    port
      .attachment::<FileHandleSerialPortAttachment>()
      .is_none()
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.clone().set_name(Some("port"));

  assert_eq!(port.name().as_deref(), Some("port"));
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioConsolePortConfiguration::new().unwrap())
      .starts_with("<VZVirtioConsolePortConfiguration: 0x")
  );
}
