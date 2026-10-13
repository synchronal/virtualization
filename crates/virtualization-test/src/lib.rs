//! What the integration suites assume, which Virtualization does not: files
//! to attach, a kernel and userland to boot, and the values chosen for them.

use std::fs::File;
use std::io::{BufRead, BufReader, Lines, PipeReader, PipeWriter};
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{
  Error, FileHandleSerialPortAttachment, LinuxBootLoader, SerialPortConfiguration,
  VirtioConsoleDeviceSerialPortConfiguration, VirtioSocketConnection, VirtioSocketDevice,
  VirtioSocketDeviceConfiguration, VirtualMachine, VirtualMachineConfiguration,
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
  initial_ramdisk_with_programs(name, script, &[])
}

/// As `initial_ramdisk`, with each program, by name, in `/bin`.
pub fn initial_ramdisk_with_programs(name: &str, script: &str, programs: &[(&str, &[u8])]) -> Url {
  let programs: Vec<(String, &[u8])> = programs
    .iter()
    .map(|(program, contents)| (format!("bin/{program}"), *contents))
    .collect();
  let busybox = std::env::var("VZ_TEST_BUSYBOX").expect("VZ_TEST_BUSYBOX: run under nextest, which sets it");
  let init = format!(
    "#!/bin/busybox sh\n/bin/busybox --install -s /bin\nexport PATH=/bin\nmount -t proc proc /proc\nmount -t sysfs \
     sysfs /sys\nmount -t devtmpfs devtmpfs /dev\n{script}\n"
  );
  let directory = 0o040_755;
  let executable = 0o100_755;

  let busybox = std::fs::read(busybox).unwrap();
  let mut entries = vec![
    CpioEntry::new("bin", directory, &[]),
    CpioEntry::new("bin/busybox", executable, &busybox),
  ];

  entries.extend(
    programs
      .iter()
      .map(|(path, contents)| CpioEntry::new(path, executable, contents)),
  );
  entries.extend([
    CpioEntry::new("dev", directory, &[]),
    CpioEntry {
      device: (5, 1),
      ..CpioEntry::new("dev/console", 0o020_600, &[])
    },
    CpioEntry::new("init", executable, init.as_bytes()),
    CpioEntry::new("proc", directory, &[]),
    CpioEntry::new("sys", directory, &[]),
    CpioEntry::new("TRAILER!!!", 0, &[]),
  ]);

  file(&format!("{name}.cpio"), &cpio(&entries))
}

/// A guest's program that connects to the host's `port`, then runs the
/// command it is given with the connection as its stdin and stdout. It exits
/// with 1 if it can't connect: busybox has nothing that speaks `AF_VSOCK`.
pub fn vsock_connecting(port: u32) -> Vec<u8> {
  vsock_program(port, false)
}

/// A guest's program that listens on its `port`, then runs the command it is
/// given with the first connection as its stdin and stdout. It exits with 1
/// if it can't accept one.
pub fn vsock_listening(port: u32) -> Vec<u8> {
  vsock_program(port, true)
}

/// A static arm64 Linux executable: an ELF header, one segment's program
/// header, the code, and the `sockaddr_vm` it connects or binds to.
fn vsock_program(port: u32, listens: bool) -> Vec<u8> {
  /// Where the file is loaded.
  const BASE: u64 = 0x40_0000;
  /// The ELF header's size, then the program header's.
  const HEADERS: u64 = 64 + 56;
  /// `svc #0`
  const SYSTEM_CALL: u32 = 0xd400_0001;
  /// `mov x19, x0`
  const KEEP_SOCKET: u32 = 0xaa00_03f3;
  /// `mov x0, x19`
  const SOCKET: u32 = 0xaa13_03e0;

  // `mov x<register>, #<value>`
  let set = |register: u32, value: u32| 0xd280_0000 | value << 5 | register;
  // A system call by number, its arguments already in `x0` to `x2`.
  let call = |number: u32| [set(8, number), SYSTEM_CALL];

  // socket(AF_VSOCK, SOCK_STREAM, 0), which stays in `x0` for the next call.
  let mut code = vec![set(0, 40), set(1, 1), set(2, 0)];
  code.extend(call(198));
  code.push(KEEP_SOCKET);

  // `adr x1, address`, once its offset is known; the address is 16 bytes.
  let takes_address = code.len();
  code.extend([0, set(2, 16)]);

  if listens {
    // bind(socket, address, 16), listen(socket, 1), accept(socket, NULL, NULL)
    code.extend(call(200));
    code.extend([SOCKET, set(1, 1)]);
    code.extend(call(201));
    code.extend([SOCKET, set(1, 0), set(2, 0)]);
    code.extend(call(202));
    code.push(KEEP_SOCKET);
  } else {
    // connect(socket, address, 16)
    code.extend(call(203));
  }

  // `tbnz x0, #63, failed`, once its offset is known: a negative result.
  let checks_result = code.len();
  code.push(0);

  // dup3(socket, 0, 0), dup3(socket, 1, 0)
  for descriptor in [0, 1] {
    code.extend([SOCKET, set(1, descriptor), set(2, 0)]);
    code.extend(call(24));
  }

  // execve(argv[1], &argv[1], envp): the stack still holds `argc`, `argv`, a
  // null, then `envp`.
  code.extend([
    0xf940_03e9, // ldr x9, [sp]
    0xf940_0be0, // ldr x0, [sp, #16]
    0x9100_43e1, // add x1, sp, #16
    0x8b09_0c22, // add x2, x1, x9, lsl #3
  ]);
  code.extend(call(221));

  // exit(1), also where a failed execve lands.
  let failed = code.len();
  code.push(set(0, 1));
  code.extend(call(93));

  // Offsets count instructions.
  code[takes_address] = 0x1000_0001 | ((code.len() - takes_address) as u32) << 5;
  code[checks_result] = 0xb7f8_0000 | ((failed - checks_result) as u32) << 5;

  // VMADDR_CID_HOST to connect to, VMADDR_CID_ANY to bind to.
  let cid: u32 = if listens { u32::MAX } else { 2 };
  let size = HEADERS + code.len() as u64 * 4 + 16;
  let mut program = Vec::new();

  // The ELF header: 64-bit, little-endian, an executable for AArch64, entered
  // at the code, with one program header, right after this one.
  program.extend_from_slice(b"\x7fELF\x02\x01\x01\0\0\0\0\0\0\0\0\0");
  program.extend_from_slice(&2u16.to_le_bytes());
  program.extend_from_slice(&183u16.to_le_bytes());
  program.extend_from_slice(&1u32.to_le_bytes());
  program.extend_from_slice(&(BASE + HEADERS).to_le_bytes());
  program.extend_from_slice(&64u64.to_le_bytes());
  program.extend_from_slice(&0u64.to_le_bytes());
  program.extend_from_slice(&0u32.to_le_bytes());
  for field in [64u16, 56, 1, 0, 0, 0] {
    program.extend_from_slice(&field.to_le_bytes());
  }

  // The program header: load the whole file at `BASE`, to read and execute.
  program.extend_from_slice(&1u32.to_le_bytes());
  program.extend_from_slice(&5u32.to_le_bytes());
  for field in [0, BASE, BASE, size, size, 0x1_0000] {
    program.extend_from_slice(&field.to_le_bytes());
  }

  for instruction in code {
    program.extend_from_slice(&instruction.to_le_bytes());
  }

  // The `sockaddr_vm`: AF_VSOCK, the port, the CID.
  program.extend_from_slice(&40u32.to_le_bytes());
  program.extend_from_slice(&port.to_le_bytes());
  program.extend_from_slice(&cid.to_le_bytes());
  program.extend_from_slice(&0u32.to_le_bytes());

  program
}

/// Gives `configuration` a Virtio socket device.
pub fn add_socket_device(configuration: &VirtualMachineConfiguration) {
  configuration.set_socket_devices(&[&VirtioSocketDeviceConfiguration::new().unwrap()]);
}

/// The socket device of a machine configured with `add_socket_device`. Call
/// it on the machine's queue.
pub fn socket_device(machine: &VirtualMachine) -> VirtioSocketDevice {
  machine
    .socket_devices::<VirtioSocketDevice>()
    .remove(0)
    .unwrap()
}

/// Another descriptor for `connection`'s, which can leave the machine's queue
/// as the connection can't, and stays open without it.
pub fn stream(connection: &VirtioSocketConnection) -> File {
  File::options()
    .read(true)
    .write(true)
    .open(format!("/dev/fd/{}", connection.file_descriptor()))
    .unwrap()
}

/// Connects `machine`'s socket device to the guest's `port`, on `queue`, and
/// waits for what `inspect` makes of the connection, or what it failed with.
pub fn connect<T: Send + 'static>(
  queue: &DispatchQueue,
  machine: &VirtualMachine,
  port: u32,
  inspect: fn(&VirtioSocketConnection) -> T,
) -> Result<T, Error> {
  let (sender, receiver) = mpsc::channel();

  on_queue(queue, || {
    socket_device(machine).connect(port, move |result| {
      sender
        .send(result.map(|connection| inspect(&connection)))
        .unwrap();
    });
  });

  receiver.recv().unwrap()
}

/// As `connect`, to a port the guest is about to listen on: it does a moment
/// after its last word on the console.
pub fn connect_once_listening<T: Send + 'static>(
  queue: &DispatchQueue,
  machine: &VirtualMachine,
  port: u32,
  inspect: fn(&VirtioSocketConnection) -> T,
) -> T {
  let mut failed = None;

  for _ in 0..50 {
    match connect(queue, machine, port, inspect) {
      Ok(inspected) => return inspected,
      Err(error) => failed = Some(error),
    }
    std::thread::sleep(Duration::from_millis(100));
  }

  panic!("the guest never listened on {port}: {failed:?}");
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
