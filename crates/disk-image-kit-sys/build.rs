//! Builds `swift/` and links it: see `apple-bridge-build`.

fn main() {
  apple_bridge_build::build(apple_bridge_build::Bridge {
    bridge: "disk-image-kit-bridge",
    package: "DiskImageKitBridge",
    framework: "DiskImageKit",
  });
}
