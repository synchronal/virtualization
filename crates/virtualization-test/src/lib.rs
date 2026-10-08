//! What the integration suites assume, which Virtualization does not: files
//! to attach, a kernel to boot, and the values chosen for them.

use std::io::PipeReader;
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::sync::mpsc;

use virtualization::dispatch2::{DispatchQueue, DispatchRetained};
use virtualization::url::Url;
use virtualization::{
  Error, FileHandleSerialPortAttachment, LinuxBootLoader, SerialPortConfiguration,
  VirtioConsoleDeviceSerialPortConfiguration, VirtualMachineConfiguration,
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

/// A Linux machine's configuration that boots the kernel
/// `bin/dev/prepare-integration` downloaded, with `initial_ramdisk` and no
/// root filesystem: 1 CPU, 512 MiB, and a virtio console, `hvc0`, writing to
/// the pipe returned.
pub fn booting_configuration(initial_ramdisk: Option<&Url>) -> (VirtualMachineConfiguration, PipeReader) {
  let kernel = std::env::var("VZ_TEST_KERNEL").expect("VZ_TEST_KERNEL: run under nextest, which sets it");
  let boot_loader = LinuxBootLoader::new(&Url::from_file_path(kernel).unwrap()).unwrap();
  let (reader, writer) = std::io::pipe().unwrap();
  let console = VirtioConsoleDeviceSerialPortConfiguration::new().unwrap();
  let configuration = VirtualMachineConfiguration::new().unwrap();

  boot_loader.set_command_line("console=hvc0");
  boot_loader
    .set_initial_ramdisk_url(initial_ramdisk)
    .unwrap();
  console.set_attachment(Some(
    &FileHandleSerialPortAttachment::new(None, Some(OwnedFd::from(writer))).unwrap(),
  ));
  configuration.set_boot_loader(Some(&boot_loader));
  configuration.set_cpu_count(1);
  configuration.set_memory_size(512 * 1024 * 1024);
  configuration.set_serial_ports(&[&console]);

  (configuration, reader)
}

/// An initial ramdisk whose `/init` powers the machine off: a newc cpio
/// archive holding one static aarch64 executable, assembled here so the
/// suites need no cross toolchain.
pub fn power_off_initial_ramdisk(name: &str) -> Url {
  file(
    &format!("{name}.cpio"),
    &cpio(&[("init", 0o100_755, &power_off_executable()), ("TRAILER!!!", 0, &[])]),
  )
}

/// An ELF executable that calls `reboot(LINUX_REBOOT_CMD_POWER_OFF)`.
fn power_off_executable() -> Vec<u8> {
  const BASE: u64 = 0x40_0000;
  const HEADERS: u64 = 64 + 56;
  let code: [u32; 9] = [
    0xd2bf_dc20, // movz x0, #0xfee1, lsl #16
    0xf29b_d5a0, // movk x0, #0xdead: LINUX_REBOOT_MAGIC1
    0xd2a5_0241, // movz x1, #0x2812, lsl #16
    0xf283_2d21, // movk x1, #0x1969: LINUX_REBOOT_MAGIC2
    0xd2a8_6422, // movz x2, #0x4321, lsl #16
    0xf29f_db82, // movk x2, #0xfedc: LINUX_REBOOT_CMD_POWER_OFF
    0xd280_11c8, // movz x8, #142: reboot
    0xd400_0001, // svc #0
    0x1400_0000, // b .
  ];
  let size = HEADERS + 4 * code.len() as u64;
  let mut elf = Vec::new();

  // The ELF header: 64-bit, little-endian, an executable for AArch64.
  elf.extend_from_slice(b"\x7fELF\x02\x01\x01\0\0\0\0\0\0\0\0\0");
  elf.extend_from_slice(&2u16.to_le_bytes());
  elf.extend_from_slice(&183u16.to_le_bytes());
  elf.extend_from_slice(&1u32.to_le_bytes());
  elf.extend_from_slice(&(BASE + HEADERS).to_le_bytes());
  elf.extend_from_slice(&64u64.to_le_bytes());
  elf.extend_from_slice(&0u64.to_le_bytes());
  elf.extend_from_slice(&0u32.to_le_bytes());
  for half in [64u16, 56, 1, 0, 0, 0] {
    elf.extend_from_slice(&half.to_le_bytes());
  }

  // One program header, loading the whole file readable and executable.
  elf.extend_from_slice(&1u32.to_le_bytes());
  elf.extend_from_slice(&5u32.to_le_bytes());
  for word in [0, BASE, BASE, size, size, 0x1000] {
    elf.extend_from_slice(&word.to_le_bytes());
  }

  for instruction in code {
    elf.extend_from_slice(&instruction.to_le_bytes());
  }

  elf
}

/// A newc cpio archive of `(name, mode, contents)` entries.
fn cpio(entries: &[(&str, u32, &[u8])]) -> Vec<u8> {
  let mut archive = Vec::new();

  for (inode, (name, mode, contents)) in entries.iter().enumerate() {
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
      0,
      0,
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
