use virtualization::{Error, FileSerialPortAttachment};
use virtualization_test::{file_url, new_file_url};

#[test]
fn has_the_url_and_append_it_was_given() {
  let url = new_file_url("has_the_url_and_append_it_was_given.log");

  let attachment = FileSerialPortAttachment::new(&url, true).unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.append());
}

#[test]
fn fails_as_virtualization_does_in_a_missing_directory() {
  let url = file_url("/nowhere/console.log");

  let Err(Error::NSError(error)) = FileSerialPortAttachment::new(&url, false) else {
    panic!("attached a file in a missing directory");
  };

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn a_clone_is_the_same_object() {
  let url = new_file_url("a_clone_is_the_same_object.log");
  let attachment = FileSerialPortAttachment::new(&url, false).unwrap();
  let clone = attachment.clone();

  drop(attachment);

  assert_eq!(clone.url().unwrap(), url);
}

#[test]
fn debugs_as_its_description() {
  let attachment = FileSerialPortAttachment::new(&new_file_url("debugs_as_its_description.log"), false).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZFileSerialPortAttachment: 0x"));
}
