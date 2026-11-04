//! Builds `swift/` and links it: see `apple-bridge-build`.

fn main() {
  apple_bridge_build::build(apple_bridge_build::Bridge {
    bridge: "accessory-access-bridge",
    package: "AccessoryAccessBridge",
    framework: "AccessoryAccess",
  });
}
