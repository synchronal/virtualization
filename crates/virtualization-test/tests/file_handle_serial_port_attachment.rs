use std::os::fd::{AsRawFd, OwnedFd};

use virtualization::FileHandleSerialPortAttachment;

#[test]
fn has_the_file_handles_it_was_given() {
  let (reader, writer) = std::io::pipe().unwrap();
  let (reading, writing) = (reader.as_raw_fd(), writer.as_raw_fd());

  let attachment =
    FileHandleSerialPortAttachment::new(Some(OwnedFd::from(reader)), Some(OwnedFd::from(writer))).unwrap();

  assert_eq!(attachment.file_handle_for_reading().unwrap().as_raw_fd(), reading);
  assert_eq!(attachment.file_handle_for_writing().unwrap().as_raw_fd(), writing);
}

#[test]
fn has_no_file_handles_when_given_none() {
  let attachment = FileHandleSerialPortAttachment::new(None, None).unwrap();

  assert!(attachment.file_handle_for_reading().is_none());
  assert!(attachment.file_handle_for_writing().is_none());
}

#[test]
fn closes_its_file_handles_when_dropped() {
  let (reader, writer) = std::io::pipe().unwrap();

  drop(FileHandleSerialPortAttachment::new(None, Some(OwnedFd::from(writer))).unwrap());

  assert_eq!(std::io::read_to_string(reader).unwrap(), "");
}

#[test]
fn a_clone_is_the_same_object() {
  let (_reader, writer) = std::io::pipe().unwrap();
  let writing = writer.as_raw_fd();
  let attachment = FileHandleSerialPortAttachment::new(None, Some(OwnedFd::from(writer))).unwrap();
  let clone = attachment.clone();

  drop(attachment);

  assert_eq!(clone.file_handle_for_writing().unwrap().as_raw_fd(), writing);
}

#[test]
fn debugs_as_its_description() {
  let attachment = FileHandleSerialPortAttachment::new(None, None).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZFileHandleSerialPortAttachment: 0x"));
}
