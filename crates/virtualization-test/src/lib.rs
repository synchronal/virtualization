//! What the integration suites assume, which Virtualization does not: files
//! to attach, and the values chosen for them.

use std::path::PathBuf;
use std::sync::mpsc;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{Error, LinuxBootLoader, VirtualMachineConfiguration};

/// A path in the suites' scratch directory, named for the test that uses it.
pub fn scratch_path(name: &str) -> PathBuf {
  let directory = std::env::temp_dir().join("virtualization-test");

  std::fs::create_dir_all(&directory).unwrap();

  directory.join(name)
}

/// A file URL in the scratch directory with nothing at it yet, to create
/// something at.
pub fn new_file_url(name: &str) -> Url {
  let path = scratch_path(name);
  let _ = std::fs::remove_file(&path);

  Url::from_file_path(path).unwrap()
}

/// A raw disk image: 1 MiB of zeroes.
pub fn disk_image(name: &str) -> Url {
  file(&format!("{name}.img"), &vec![0u8; 1024 * 1024])
}

/// A file with `contents`.
pub fn file(name: &str, contents: &[u8]) -> Url {
  let path = scratch_path(name);

  std::fs::write(&path, contents).unwrap();

  Url::from_file_path(path).unwrap()
}

/// A file URL for `path`, which need not exist.
pub fn file_url(path: &str) -> Url {
  Url::from_file_path(path).unwrap()
}

/// A Linux machine's configuration that validates: 1 CPU, 512 MiB, and a
/// placeholder kernel, which doesn't boot.
pub fn linux_configuration(name: &str) -> VirtualMachineConfiguration {
  let configuration = VirtualMachineConfiguration::new().unwrap();

  configuration.set_boot_loader(Some(
    &LinuxBootLoader::new(&file(&format!("{name}.kernel"), b"kernel")).unwrap(),
  ));
  configuration.set_cpu_count(1);
  configuration.set_memory_size(512 * 1024 * 1024);

  configuration
}

/// A serial queue for one machine.
pub fn serial_queue(name: &str) -> DispatchRetained<DispatchQueue> {
  DispatchQueue::new(name, None)
}

/// Runs `work` on `queue`, and returns what it did.
pub fn on_queue<T: Send>(queue: &DispatchQueue, work: impl FnOnce() -> T + Send) -> T {
  let mut result = None;

  queue.exec_sync(|| result = Some(work()));

  result.unwrap()
}

/// Calls `operation` on `queue` with a completion handler, and waits for
/// what it is called with.
pub fn completed(
  queue: &DispatchQueue,
  operation: impl FnOnce(Box<dyn FnOnce(Result<(), Error>) + Send>) + Send,
) -> Result<(), Error> {
  let (sender, receiver) = mpsc::channel();

  on_queue(queue, || {
    operation(Box::new(move |result| sender.send(result).unwrap()));
  });

  receiver.recv().unwrap()
}
