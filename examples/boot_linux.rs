//! Starts a Linux virtual machine from a kernel image.
//!
//!   cargo build -p virtualization-test --example boot_linux
//!   bin/dev/sign target/debug/examples/boot_linux
//!   target/debug/examples/boot_linux /path/to/vmlinux

use std::sync::mpsc;

use dispatch2::DispatchQueue;
use url::Url;
use virtualization::{Error, LinuxBootLoader, VirtualMachine, VirtualMachineConfiguration};

fn main() -> Result<(), Error> {
  let kernel = std::env::args().nth(1).expect("usage: boot_linux <kernel>");
  let kernel = Url::from_file_path(std::path::absolute(kernel).unwrap()).unwrap();

  // A virtual machine is used from one dispatch queue, as in Swift.
  let queue = DispatchQueue::new("example.vm", None);

  let mut created = None;
  queue.exec_sync(|| created = Some(create_machine(&kernel, &queue)));
  let machine = created.unwrap()?;

  let (sender, receiver) = mpsc::channel();
  queue.exec_async(move || machine.start(move |result| sender.send(result).unwrap()));

  receiver.recv().unwrap()?;
  println!("started");

  Ok(())
}

fn create_machine(kernel: &Url, queue: &DispatchQueue) -> Result<VirtualMachine, Error> {
  let boot_loader = LinuxBootLoader::new(kernel)?;
  boot_loader.set_command_line("console=hvc0");

  let configuration = VirtualMachineConfiguration::new()?;
  configuration.set_boot_loader(Some(&boot_loader));
  configuration.set_cpu_count(2);
  configuration.set_memory_size(1024 * 1024 * 1024);
  configuration.validate()?;

  VirtualMachine::with_queue(&configuration, queue)
}
