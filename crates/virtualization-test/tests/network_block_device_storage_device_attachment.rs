use std::io::Write;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use virtualization::url::Url;
use virtualization::{
  DiskImageStorageDeviceAttachment, DiskSynchronizationMode, Error, NetworkBlockDeviceStorageDeviceAttachment,
  NetworkBlockDeviceStorageDeviceAttachmentDelegate, StorageDeviceConfiguration, VirtioBlockDeviceConfiguration,
  VirtualMachineState,
};
use virtualization_test::{booting_machine, completed, initial_ramdisk, nbd_server, on_queue, start, stop};

fn url(url: &str) -> Url {
  Url::parse(url).unwrap()
}

fn ns_error<T: std::fmt::Debug>(result: Result<T, Error>) -> Box<virtualization::NSError> {
  match result {
    Err(Error::NSError(error)) => error,
    other => panic!("expected Virtualization's error, got {other:?}"),
  }
}

struct Delegate;

impl NetworkBlockDeviceStorageDeviceAttachmentDelegate for Delegate {}

#[test]
fn has_virtualizations_timeout_and_modes_when_none_are_given() {
  let url = url("nbd://127.0.0.1:10809/export");
  let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url).unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert_eq!(attachment.timeout(), 5.0);
  assert!(!attachment.is_forced_read_only());
  assert_eq!(attachment.synchronization_mode(), DiskSynchronizationMode::Full);
}

#[test]
fn has_the_timeout_and_modes_it_was_made_with() {
  let url = url("nbd://127.0.0.1:10809/export");
  let attachment =
    NetworkBlockDeviceStorageDeviceAttachment::with_timeout_and_is_forced_read_only_and_synchronization_mode(
      &url,
      2.5,
      true,
      DiskSynchronizationMode::None,
    )
    .unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert_eq!(attachment.timeout(), 2.5);
  assert!(attachment.is_forced_read_only());
  assert_eq!(attachment.synchronization_mode(), DiskSynchronizationMode::None);
}

#[test]
fn validates_an_nbd_url_over_tcp_or_a_unix_socket() {
  NetworkBlockDeviceStorageDeviceAttachment::validate(&url("nbd://127.0.0.1:10809/export")).unwrap();
  NetworkBlockDeviceStorageDeviceAttachment::validate(&url("nbd://127.0.0.1")).unwrap();
  NetworkBlockDeviceStorageDeviceAttachment::validate(&url("nbd+unix:///export?socket=/tmp/nbd.sock")).unwrap();
}

#[test]
fn fails_as_virtualization_does_to_validate_another_scheme() {
  for other in ["nbds://127.0.0.1/export", "http://127.0.0.1/", "file:///tmp/disk.img"] {
    let error = ns_error(NetworkBlockDeviceStorageDeviceAttachment::validate(&url(other)));

    assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
    assert_eq!(error.code, 2, "{error:#?}");
  }
}

#[test]
fn fails_as_virtualization_does_on_another_scheme() {
  let error = ns_error(NetworkBlockDeviceStorageDeviceAttachment::new(&url(
    "http://127.0.0.1/",
  )));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn has_the_delegate_it_was_given() {
  let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url("nbd://127.0.0.1")).unwrap();
  let delegate: Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate> = Arc::new(Delegate);

  assert!(attachment.delegate().is_none());

  attachment.set_delegate(Some(&delegate));

  assert!(Arc::ptr_eq(&attachment.delegate().unwrap(), &delegate));

  attachment.set_delegate(None);

  assert!(attachment.delegate().is_none());
}

#[test]
fn holds_its_delegate_weakly() {
  let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url("nbd://127.0.0.1")).unwrap();
  let delegate: Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate> = Arc::new(Delegate);

  attachment.set_delegate(Some(&delegate));
  drop(delegate);

  assert!(attachment.delegate().is_none());
}

#[test]
fn a_clone_is_the_same_object() {
  let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url("nbd://127.0.0.1")).unwrap();
  let delegate: Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate> = Arc::new(Delegate);

  attachment.clone().set_delegate(Some(&delegate));

  assert!(Arc::ptr_eq(&attachment.delegate().unwrap(), &delegate));
}

#[test]
fn is_the_attachment_of_the_configuration_it_was_given_to() {
  let url = url("nbd://127.0.0.1:10809/export");
  let configuration =
    VirtioBlockDeviceConfiguration::new(&NetworkBlockDeviceStorageDeviceAttachment::new(&url).unwrap()).unwrap();

  assert_eq!(
    configuration
      .attachment::<NetworkBlockDeviceStorageDeviceAttachment>()
      .unwrap()
      .url()
      .unwrap(),
    url
  );
  assert!(
    configuration
      .attachment::<DiskImageStorageDeviceAttachment>()
      .is_none()
  );
}

#[test]
fn debugs_as_its_description() {
  let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url("nbd://127.0.0.1")).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZNetworkBlockDeviceStorageDeviceAttachment: 0x"));
}

/// Sends what the attachment tells it.
struct Telling(Mutex<Sender<String>>);

impl NetworkBlockDeviceStorageDeviceAttachmentDelegate for Telling {
  fn attachment_was_connected(&self, attachment: &NetworkBlockDeviceStorageDeviceAttachment) {
    let _ = self
      .0
      .lock()
      .unwrap()
      .send(format!("connected {}", attachment.url().unwrap()));
  }

  fn attachment_did_encounter_error(&self, attachment: &NetworkBlockDeviceStorageDeviceAttachment, error: Error) {
    let Error::NSError(error) = error else {
      panic!("expected Virtualization's error, got {error:?}");
    };

    let _ = self.0.lock().unwrap().send(format!(
      "error {} {} {}",
      attachment.url().unwrap(),
      error.domain,
      error.code
    ));
  }
}

/// A megabyte starting with `contents`.
fn export(contents: &[u8]) -> Vec<u8> {
  let mut export = contents.to_vec();

  export.resize(1024 * 1024, 0);
  export
}

#[test]
fn a_guest_reads_and_writes_the_export_and_the_delegate_hears_it_connect() {
  let name = "a_guest_reads_and_writes_the_export";
  let server = nbd_server(export(b"from the host"));
  let (sender, told) = mpsc::channel();
  let delegate: Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate> = Arc::new(Telling(Mutex::new(sender)));
  let script = "read=\"$(head -c 13 /dev/vda)\"\n\
    printf 'from the guest' | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null\n\
    echo \"read:$read\"";

  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |machine| {
    let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&server.url).unwrap();

    attachment.set_delegate(Some(&delegate));
    machine.set_storage_devices(&[&VirtioBlockDeviceConfiguration::new(&attachment).unwrap()]);
  });
  start(&queue, &machine);

  assert_eq!(console.read_until("read:").trim_end(), "read:from the host");
  stop(&queue, &machine);
  assert_eq!(told.recv().unwrap(), format!("connected {}", server.url));
  assert!(server.export.lock().unwrap().starts_with(b"from the guest"));
}

#[test]
fn fails_to_start_as_virtualization_does_with_no_server() {
  let name = "fails_to_start_as_virtualization_does_with_no_server";

  let (queue, machine, _console) = booting_machine(name, None, |machine| {
    let attachment = NetworkBlockDeviceStorageDeviceAttachment::new(&url("nbd://127.0.0.1:1/")).unwrap();

    machine.set_storage_devices(&[&VirtioBlockDeviceConfiguration::new(&attachment).unwrap()]);
  });

  let error = ns_error(completed(&queue, |completion_handler| {
    machine.start(completion_handler)
  }));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

/// The client reconnects each time its timeout passes, and each failure to
/// negotiate is an error: the first is enough here. The machine keeps
/// running.
#[test]
fn tells_its_delegate_when_the_server_no_longer_negotiates() {
  let name = "tells_its_delegate_when_the_server_no_longer_negotiates";
  let server = nbd_server(export(b""));
  let (sender, told) = mpsc::channel();
  let delegate: Arc<dyn NetworkBlockDeviceStorageDeviceAttachmentDelegate> = Arc::new(Telling(Mutex::new(sender)));
  let script = "echo ready:; read line; dd if=/dev/vda of=/dev/null bs=4096 skip=128 count=1";

  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |machine| {
    let attachment =
      NetworkBlockDeviceStorageDeviceAttachment::with_timeout_and_is_forced_read_only_and_synchronization_mode(
        &server.url,
        1.0,
        false,
        DiskSynchronizationMode::Full,
      )
      .unwrap();

    attachment.set_delegate(Some(&delegate));
    machine.set_storage_devices(&[&VirtioBlockDeviceConfiguration::new(&attachment).unwrap()]);
  });
  start(&queue, &machine);
  console.read_until("ready:");
  server.stop();
  console.input.write_all(b"go\n").unwrap();

  assert_eq!(told.recv().unwrap(), format!("connected {}", server.url));
  assert_eq!(
    told.recv().unwrap(),
    format!("error {} VZErrorDomain 20001", server.url)
  );
  assert_eq!(on_queue(&queue, || machine.state()), VirtualMachineState::Running);
}
