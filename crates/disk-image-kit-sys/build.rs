//! Builds `swift/` and links it: see `virtualization-build`.

fn main() {
  virtualization_build::build(virtualization_build::Bridge {
    bridge: "disk-image-kit-bridge",
    package: "DiskImageKitBridge",
    framework: "DiskImageKit",
  });
}
