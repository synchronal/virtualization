//! What the integration suites assume, which Virtualization does not: files
//! to attach, a kernel and userland to boot, and the values chosen for them.

use std::io::{BufRead, BufReader, Lines, PipeReader, PipeWriter};
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::sync::mpsc;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{
  Error, FileHandleSerialPortAttachment, LinuxBootLoader, SerialPortConfiguration,
  VirtioConsoleDeviceSerialPortConfiguration, VirtualMachine, VirtualMachineConfiguration,
};

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

/// A fresh scratch directory holding each file with its contents.
pub fn directory(name: &str, files: &[(&str, &str)]) -> PathBuf {
  let path = scratch_path(name);
  let _ = std::fs::remove_dir_all(&path);

  std::fs::create_dir(&path).unwrap();
  for (file, contents) in files {
    std::fs::write(path.join(file), contents).unwrap();
  }

  path
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

/// A machine booting the kernel `bin/dev/prepare-integration` downloaded,
/// made on its own queue, with `initial_ramdisk` and no root filesystem: 1
/// CPU, 512 MiB, and a virtio console, `hvc0`. `configure` adds to the
/// configuration before the machine is made from it.
pub fn booting_machine(
  name: &str,
  initial_ramdisk: Option<&Url>,
  configure: impl FnOnce(&VirtualMachineConfiguration) + Send,
) -> (DispatchRetained<DispatchQueue>, VirtualMachine, Console) {
  let queue = serial_queue(name);
  let (machine, console) = on_queue(&queue, || {
    let kernel = std::env::var("VZ_TEST_KERNEL").expect("VZ_TEST_KERNEL: run under nextest, which sets it");
    let boot_loader = LinuxBootLoader::new(&Url::from_file_path(kernel).unwrap()).unwrap();
    let (attachment, console) = console_attachment();
    let serial_port = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();
    let configuration = VirtualMachineConfiguration::new().unwrap();

    boot_loader.set_command_line("console=hvc0 quiet");
    boot_loader
      .set_initial_ramdisk_url(initial_ramdisk)
      .unwrap();
    serial_port.set_attachment(Some(&attachment));
    configuration.set_boot_loader(Some(&boot_loader));
    configuration.set_cpu_count(1);
    configuration.set_memory_size(512 * 1024 * 1024);
    configuration.set_serial_ports(&[&serial_port]);
    configure(&configuration);

    (VirtualMachine::with_queue(&configuration, &queue).unwrap(), console)
  });

  (queue, machine, console)
}

/// An attachment of a pipe each way, and the host's ends of them.
pub fn console_attachment() -> (FileHandleSerialPortAttachment, Console) {
  let (output, guest_output) = std::io::pipe().unwrap();
  let (guest_input, input) = std::io::pipe().unwrap();

  (
    FileHandleSerialPortAttachment::new(Some(OwnedFd::from(guest_input)), Some(OwnedFd::from(guest_output))).unwrap(),
    Console {
      output: BufReader::new(output).lines(),
      input,
    },
  )
}

/// Starts `machine` on `queue`, and waits for it to have started.
pub fn start(queue: &DispatchQueue, machine: &VirtualMachine) {
  completed(queue, |completion_handler| machine.start(completion_handler)).unwrap();
}

/// Stops `machine` on `queue`, and waits for it to have stopped.
pub fn stop(queue: &DispatchQueue, machine: &VirtualMachine) {
  completed(queue, |completion_handler| machine.stop(completion_handler)).unwrap();
}

/// The host's end of a guest's console: `hvc0`, or a port given
/// `console_attachment`.
pub struct Console {
  output: Lines<BufReader<PipeReader>>,
  /// What the guest reads.
  pub input: PipeWriter,
}

impl Console {
  /// Reads what the guest writes until a line containing `text`, and returns
  /// that line. Panics if the console closes first.
  pub fn read_until(&mut self, text: &str) -> String {
    self
      .output
      .by_ref()
      .map_while(Result::ok)
      .find(|line| line.contains(text))
      .unwrap_or_else(|| panic!("the console closed before {text:?}"))
  }
}

/// An initial ramdisk whose `/init` is busybox's shell running `script`,
/// after installing busybox's applets in `/bin` and mounting `/proc`, `/sys`,
/// and `/dev`. Its stdin and stdout are the console. When `script` ends, init
/// exits and the kernel panics: end with `poweroff -f` to stop the machine.
pub fn initial_ramdisk(name: &str, script: &str) -> Url {
  let busybox = std::env::var("VZ_TEST_BUSYBOX").expect("VZ_TEST_BUSYBOX: run under nextest, which sets it");
  let init = format!(
    "#!/bin/busybox sh\n/bin/busybox --install -s /bin\nexport PATH=/bin\nmount -t proc proc /proc\nmount -t sysfs \
     sysfs /sys\nmount -t devtmpfs devtmpfs /dev\n{script}\n"
  );
  let directory = 0o040_755;
  let executable = 0o100_755;

  file(
    &format!("{name}.cpio"),
    &cpio(&[
      CpioEntry::new("bin", directory, &[]),
      CpioEntry::new("bin/busybox", executable, &std::fs::read(busybox).unwrap()),
      CpioEntry::new("dev", directory, &[]),
      CpioEntry {
        device: (5, 1),
        ..CpioEntry::new("dev/console", 0o020_600, &[])
      },
      CpioEntry::new("init", executable, init.as_bytes()),
      CpioEntry::new("proc", directory, &[]),
      CpioEntry::new("sys", directory, &[]),
      CpioEntry::new("TRAILER!!!", 0, &[]),
    ]),
  )
}

/// A file in a cpio archive.
struct CpioEntry<'a> {
  name: &'a str,
  mode: u32,
  /// The major and minor number of the device a device file is.
  device: (u32, u32),
  contents: &'a [u8],
}

impl<'a> CpioEntry<'a> {
  fn new(name: &'a str, mode: u32, contents: &'a [u8]) -> Self {
    Self {
      name,
      mode,
      device: (0, 0),
      contents,
    }
  }
}

/// A newc cpio archive of `entries`.
fn cpio(entries: &[CpioEntry<'_>]) -> Vec<u8> {
  let mut archive = Vec::new();

  for (
    inode,
    CpioEntry {
      name,
      mode,
      device,
      contents,
    },
  ) in entries.iter().enumerate()
  {
    let fields = [
      inode as u32 + 1,
      *mode,
      0,
      0,
      1,
      0,
      contents.len() as u32,
      0,
      0,
      device.0,
      device.1,
      name.len() as u32 + 1,
      0,
    ];

    archive.extend_from_slice(b"070701");
    for field in fields {
      archive.extend_from_slice(format!("{field:08x}").as_bytes());
    }
    archive.extend_from_slice(name.as_bytes());
    archive.push(0);
    archive.resize(archive.len().next_multiple_of(4), 0);
    archive.extend_from_slice(contents);
    archive.resize(archive.len().next_multiple_of(4), 0);
  }

  archive
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
