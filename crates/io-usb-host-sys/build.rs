//! Builds `swift/` and links it: see `apple-bridge-build`.

fn main() {
  apple_bridge_build::build(apple_bridge_build::Bridge {
    bridge: "io-usb-host-bridge",
    package: "IOUSBHostBridge",
    framework: "IOUSBHost",
  });
}
