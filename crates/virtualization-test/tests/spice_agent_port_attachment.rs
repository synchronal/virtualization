use virtualization::{
  ConsolePortConfiguration, Error, SpiceAgentPortAttachment, VirtioConsoleDeviceConfiguration,
  VirtioConsolePortConfiguration,
};
use virtualization_test::linux_configuration;

#[test]
fn names_the_spice_agent_port() {
  assert_eq!(
    SpiceAgentPortAttachment::spice_agent_port_name().unwrap(),
    "com.redhat.spice.0"
  );
}

#[test]
fn shares_the_clipboard_at_first() {
  assert!(SpiceAgentPortAttachment::new().unwrap().shares_clipboard());
}

#[test]
fn keeps_whether_it_shares_the_clipboard() {
  let attachment = SpiceAgentPortAttachment::new().unwrap();

  attachment.set_shares_clipboard(false);

  assert!(!attachment.shares_clipboard());
}

#[test]
fn is_the_attachment_of_the_port_it_was_given_to() {
  let port = VirtioConsolePortConfiguration::new().unwrap();
  let attachment = SpiceAgentPortAttachment::new().unwrap();

  port.set_attachment(Some(&attachment));

  assert_eq!(
    format!("{:?}", port.attachment::<SpiceAgentPortAttachment>().unwrap()),
    format!("{attachment:?}")
  );
}

/// A configuration with one console device, whose port 0 is `port`.
fn with_port(name: &str, port: &VirtioConsolePortConfiguration) -> Result<(), Error> {
  let configuration = linux_configuration(name);
  let device = VirtioConsoleDeviceConfiguration::new().unwrap();

  device.ports().set(0, Some(port));
  configuration.set_console_devices(&[&device]);

  configuration.validate()
}

#[test]
fn validates_on_the_spice_agent_port() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some(&SpiceAgentPortAttachment::spice_agent_port_name().unwrap()));
  port.set_attachment(Some(&SpiceAgentPortAttachment::new().unwrap()));

  with_port("spice_validates_on_the_spice_agent_port", &port).unwrap();
}

/// Apple documents that the spice agent port must not be a console;
/// validation doesn't check.
#[test]
fn validates_on_a_console_port() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some(&SpiceAgentPortAttachment::spice_agent_port_name().unwrap()));
  port.set_attachment(Some(&SpiceAgentPortAttachment::new().unwrap()));
  port.set_is_console(true);

  with_port("spice_validates_on_a_console_port", &port).unwrap();
}

#[test]
fn validates_on_a_port_of_another_name() {
  let port = VirtioConsolePortConfiguration::new().unwrap();

  port.set_name(Some("other"));
  port.set_attachment(Some(&SpiceAgentPortAttachment::new().unwrap()));

  with_port("spice_validates_on_a_port_of_another_name", &port).unwrap();
}

#[test]
fn a_clone_is_the_same_object() {
  let attachment = SpiceAgentPortAttachment::new().unwrap();

  attachment.clone().set_shares_clipboard(false);

  assert!(!attachment.shares_clipboard());
}

#[test]
fn debugs_as_its_description() {
  assert!(format!("{:?}", SpiceAgentPortAttachment::new().unwrap()).starts_with("<VZSpiceAgentPortAttachment: 0x"));
}
