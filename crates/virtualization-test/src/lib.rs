//! What the integration suites assume, which Virtualization does not: files
//! to attach, a kernel and userland to boot, and the values chosen for them.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, PipeReader, PipeWriter, Read, Write};
use std::net::{Ipv4Addr, Shutdown, TcpListener, TcpStream};
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::io_usb_host::{Device, DeviceMatchingDictionaryArguments, PropertyListValue};
use virtualization::url::Url;
use virtualization::vmnet::{Mode, Network, NetworkConfiguration};
use virtualization::{
  DiskImageStorageDeviceAttachment, EFIVariableStore, EFIVariableStoreInitializationOptions, Error,
  FileHandleSerialPortAttachment, LinuxBootLoader, NetworkDeviceConfiguration, SerialPortConfiguration,
  USBMassStorageDevice, USBMassStorageDeviceConfiguration, VirtioConsoleDeviceSerialPortConfiguration,
  VirtioNetworkDeviceConfiguration, VirtioSocketConnection, VirtioSocketDevice, VirtioSocketDeviceConfiguration,
  VirtualMachine, VirtualMachineConfiguration, VmnetNetworkDeviceAttachment, XHCIController,
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
  raw_image(name, &[])
}

/// A raw disk image of 1 MiB starting with `contents`.
pub fn raw_image(name: &str, contents: &[u8]) -> Url {
  let mut image = contents.to_vec();

  image.resize(1024 * 1024, 0);

  file(&format!("{name}.img"), &image)
}

/// A file with `contents`.
pub fn file(name: &str, contents: &[u8]) -> Url {
  let path = scratch_path(name);

  std::fs::write(&path, contents).unwrap();

  Url::from_file_path(path).unwrap()
}

/// A self-signed X.509 certificate's DER bytes, as `/usr/bin/openssl` makes
/// one for a new 2048-bit RSA key.
pub fn certificate(name: &str) -> Vec<u8> {
  let path = scratch_path(&format!("{name}.der"));
  let made = Command::new("/usr/bin/openssl")
    .args(["req", "-x509", "-newkey", "rsa:2048", "-nodes", "-keyout", "/dev/null"])
    .args(["-outform", "DER", "-days", "1", "-subj", &format!("/CN={name}"), "-out"])
    .arg(&path)
    .output()
    .unwrap();

  assert!(made.status.success(), "{}", String::from_utf8_lossy(&made.stderr));

  std::fs::read(path).unwrap()
}

/// A new EFI variable store, in the scratch directory.
pub fn variable_store(name: &str) -> EFIVariableStore {
  EFIVariableStore::with_creating_variable_store_at_and_options(
    &new_file_url(&format!("{name}.efivars")),
    EFIVariableStoreInitializationOptions::new(0),
  )
  .unwrap()
}

/// A disk the host attached from a raw image, which whoever attached it may
/// open. Detached when dropped.
pub struct BlockDevice {
  /// Its raw device, `/dev/rdiskN`.
  pub path: PathBuf,
}

/// A block device of 1 MiB starting with `contents`, as `/usr/bin/hdiutil`
/// attaches a raw image without mounting it.
pub fn block_device(name: &str, contents: &[u8]) -> BlockDevice {
  let image = raw_image(name, contents);
  let attached = Command::new("/usr/bin/hdiutil")
    .args(["attach", "-nomount", "-imagekey", "diskimage-class=CRawDiskImage"])
    .arg(image.to_file_path().unwrap())
    .output()
    .unwrap();

  assert!(
    attached.status.success(),
    "{}",
    String::from_utf8_lossy(&attached.stderr)
  );

  let disk = String::from_utf8(attached.stdout).unwrap();
  let disk = disk.split_whitespace().next().unwrap();

  BlockDevice {
    path: PathBuf::from(disk.replacen("/dev/disk", "/dev/rdisk", 1)),
  }
}

impl BlockDevice {
  /// A descriptor for the device, for reading and writing.
  pub fn open(&self) -> OwnedFd {
    OwnedFd::from(
      File::options()
        .read(true)
        .write(true)
        .open(&self.path)
        .unwrap(),
    )
  }

  /// The device's first sector: a raw device reads only whole ones.
  pub fn first_sector(&self) -> [u8; 512] {
    let mut sector = [0; 512];

    File::from(self.open()).read_exact(&mut sector).unwrap();

    sector
  }
}

impl Drop for BlockDevice {
  fn drop(&mut self) {
    let _ = Command::new("/usr/bin/hdiutil")
      .args(["detach", "-force"])
      .arg(&self.path)
      .output();
  }
}

/// A network block device server on a loopback port, serving one export to
/// any client under any export name, until it is stopped or the process
/// exits.
pub struct NbdServer {
  /// Its `nbd` URL.
  pub url: Url,
  /// What it serves, shared, so a test reads what a guest wrote.
  pub export: Arc<Mutex<Vec<u8>>>,
  /// Its clients, until it is stopped.
  clients: Arc<Mutex<Option<Vec<TcpStream>>>>,
}

impl NbdServer {
  /// Hangs up on every client, and on any that connects from now on.
  pub fn stop(&self) {
    for client in self.clients.lock().unwrap().take().unwrap() {
      client.shutdown(Shutdown::Both).unwrap();
    }
  }
}

/// A server of `export`.
pub fn nbd_server(export: Vec<u8>) -> NbdServer {
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  let server = NbdServer {
    url: Url::parse(&format!("nbd://127.0.0.1:{}/", listener.local_addr().unwrap().port())).unwrap(),
    export: Arc::new(Mutex::new(export)),
    clients: Arc::new(Mutex::new(Some(Vec::new()))),
  };
  let export = Arc::clone(&server.export);
  let clients = Arc::clone(&server.clients);

  std::thread::spawn(move || {
    for client in listener.incoming() {
      let client = client.unwrap();
      let export = Arc::clone(&export);

      // Stopped: dropping the client hangs up on it.
      let Some(clients) = &mut *clients.lock().unwrap() else {
        continue;
      };

      clients.push(client.try_clone().unwrap());
      // A client that hangs up ends its own thread.
      std::thread::spawn(move || nbd_serve(client, &export));
    }
  });

  server
}

/// Replies to an NBD option.
fn nbd_reply(client: &mut TcpStream, option: u32, reply: u32, data: &[u8]) -> std::io::Result<()> {
  client.write_all(&0x0003_e889_0455_65a9_u64.to_be_bytes())?;
  client.write_all(&option.to_be_bytes())?;
  client.write_all(&reply.to_be_bytes())?;
  client.write_all(&u32::try_from(data.len()).unwrap().to_be_bytes())?;
  client.write_all(data)
}

/// The NBD protocol's fixed newstyle handshake, then its simple replies to
/// reads, writes, and flushes, until the client disconnects.
fn nbd_serve(mut client: TcpStream, export: &Mutex<Vec<u8>>) -> std::io::Result<()> {
  // What the export sends: flags, and flush.
  let transmission_flags = 0b101_u16.to_be_bytes();
  let size = (export.lock().unwrap().len() as u64).to_be_bytes();
  let mut word = [0; 4];

  // Fixed newstyle, and no zeroes after an export's name.
  client.write_all(b"NBDMAGICIHAVEOPT")?;
  client.write_all(&0b11_u16.to_be_bytes())?;
  client.read_exact(&mut word)?;

  let no_zeroes = u32::from_be_bytes(word) & 0b10 != 0;

  loop {
    let mut header = [0; 16];

    client.read_exact(&mut header)?;

    let option = u32::from_be_bytes(header[8..12].try_into().unwrap());
    let mut data = vec![0; u32::from_be_bytes(header[12..16].try_into().unwrap()) as usize];

    client.read_exact(&mut data)?;

    match option {
      // NBD_OPT_EXPORT_NAME.
      1 => {
        client.write_all(&size)?;
        client.write_all(&transmission_flags)?;
        if !no_zeroes {
          client.write_all(&[0; 124])?;
        }

        break;
      }
      // NBD_OPT_INFO, and NBD_OPT_GO, which starts transmission: NBD_REP_INFO
      // with NBD_INFO_EXPORT, another with NBD_INFO_BLOCK_SIZE (512 bytes at
      // least, 4 KiB preferred, 1 MiB at most), then NBD_REP_ACK.
      6 | 7 => {
        let block_sizes = [512_u32, 4096, 1024 * 1024].map(u32::to_be_bytes).concat();

        nbd_reply(
          &mut client,
          option,
          3,
          &[&[0, 0], &size[..], &transmission_flags].concat(),
        )?;
        nbd_reply(&mut client, option, 3, &[&[0, 3], &block_sizes[..]].concat())?;
        nbd_reply(&mut client, option, 1, &[])?;
        if option == 7 {
          break;
        }
      }
      // NBD_REP_ERR_UNSUP.
      _ => nbd_reply(&mut client, option, 0x8000_0001, &[])?,
    }
  }

  loop {
    let mut request = [0; 28];

    client.read_exact(&mut request)?;

    let command = u16::from_be_bytes(request[6..8].try_into().unwrap());
    let offset = u64::from_be_bytes(request[16..24].try_into().unwrap()) as usize;
    let length = u32::from_be_bytes(request[24..28].try_into().unwrap()) as usize;
    let mut reply = [&0x6744_6698_u32.to_be_bytes(), &[0; 4], &request[8..16]].concat();

    match command {
      // NBD_CMD_READ.
      0 => reply.extend_from_slice(&export.lock().unwrap()[offset..offset + length]),
      // NBD_CMD_WRITE.
      1 => client.read_exact(&mut export.lock().unwrap()[offset..offset + length])?,
      // NBD_CMD_DISC.
      2 => return Ok(()),
      // NBD_CMD_FLUSH.
      3 => {}
      // EINVAL.
      _ => reply[4..8].copy_from_slice(&22_u32.to_be_bytes()),
    }

    client.write_all(&reply)?;
  }
}

/// A USB mass storage device's configuration, of a read-only disk image.
pub fn usb_mass_storage_device_configuration(name: &str) -> USBMassStorageDeviceConfiguration {
  USBMassStorageDeviceConfiguration::new(&DiskImageStorageDeviceAttachment::new(&disk_image(name), true).unwrap())
    .unwrap()
}

thread_local! {
  /// A device can't leave its thread, and a test needs the same one from one
  /// call on its machine's queue to the next. Dispatch runs a block on the
  /// thread that waits for it (`on_queue`), so the test's thread keeps it.
  static USB_MASS_STORAGE_DEVICE: USBMassStorageDevice =
    USBMassStorageDevice::new(&usb_mass_storage_device_configuration("usb_mass_storage_device")).unwrap();
}

/// Runs `work` with this thread's USB mass storage device, which no
/// configuration made: one to attach to a running machine's controller.
pub fn with_usb_mass_storage_device<T>(work: impl FnOnce(&USBMassStorageDevice) -> T) -> T {
  USB_MASS_STORAGE_DEVICE.with(work)
}

/// IOUSBHost's matching dictionary for an Apple device (vendor 0x05ac) of
/// product 0x1234, which no Mac need have.
pub fn apple_device_matching_dictionary() -> BTreeMap<String, PropertyListValue> {
  Device::create_matching_dictionary(&DeviceMatchingDictionaryArguments {
    vendor_id: Some(0x05ac),
    product_id: Some(0x1234),
    bcd_device: None,
    device_class: None,
    device_subclass: None,
    device_protocol: None,
    speed: None,
    product_id_array: None,
  })
  .unwrap()
}

/// The XHCI controller of a machine configured with one. Call it on the
/// machine's queue.
pub fn xhci_controller(machine: &VirtualMachine) -> XHCIController {
  machine
    .usb_controllers::<XHCIController>()
    .remove(0)
    .unwrap()
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

/// What a guest found on a vmnet network: the address a DHCP lease gave
/// `eth0`, if one came within five seconds, and `eth0`'s MTU.
pub struct GuestNetwork {
  pub lease: Option<Ipv4Addr>,
  pub mtu: u32,
  /// The network's `subnet` and `mask`.
  pub subnet: (Ipv4Addr, Ipv4Addr),
}

/// Boots a guest with a Virtio network device on a host-mode vmnet network,
/// and reports what it found. `configure` adds to the network's configuration
/// and the device's.
pub fn guest_network(
  name: &str,
  configure: impl FnOnce(&NetworkConfiguration, &VirtioNetworkDeviceConfiguration) + Send,
) -> GuestNetwork {
  let script = "printf '#!/bin/sh\\n[ \"$1\" = bound ] && echo \"lease:$ip\"\\n' > /bin/lease\nchmod +x \
                /bin/lease\nip link set eth0 up\nudhcpc -i eth0 -n -q -t 5 -T 1 -s /bin/lease || echo \
                lease:none\necho \"mtu:$(cat /sys/class/net/eth0/mtu)\"\npoweroff -f";
  let mut subnet = (Ipv4Addr::UNSPECIFIED, Ipv4Addr::UNSPECIFIED);
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |configuration| {
    let network_configuration = NetworkConfiguration::new(Mode::Host).unwrap();
    let device = VirtioNetworkDeviceConfiguration::new().unwrap();

    configure(&network_configuration, &device);
    let network = Network::new(&network_configuration).unwrap();

    subnet = network.ipv4_subnet();
    device.set_attachment(Some(&VmnetNetworkDeviceAttachment::new(&network).unwrap()));
    configuration.set_network_devices(&[&device]);
  });

  start(&queue, &machine);
  let lease = console.read_until("lease:");
  let mtu = console.read_until("mtu:");

  GuestNetwork {
    lease: lease
      .trim_end()
      .strip_prefix("lease:")
      .unwrap()
      .parse()
      .ok(),
    mtu: mtu
      .trim_end()
      .strip_prefix("mtu:")
      .unwrap()
      .parse()
      .unwrap(),
    subnet,
  }
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
