//! Builds `swift/` and links it: see `virtualization-build`.

fn main() {
  virtualization_build::build(virtualization_build::Bridge {
    bridge: "io-usb-host-bridge",
    package: "IOUSBHostBridge",
    framework: "IOUSBHost",
  });
}
