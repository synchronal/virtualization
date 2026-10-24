//! The build script the `-sys` crates share: generates the FFI glue, builds
//! the Swift package, and tells cargo how to link it. Each crate's `build.rs`
//! calls [`build`] with what differs.
//!
//! The package is staged into `OUT_DIR`, but SwiftPM's build directory is kept
//! at `target/<profile>/<crate>-swift`: `OUT_DIR` is keyed by the build
//! script's fingerprint, so every edit here would restart the Swift build.
//!
//! macOS only: elsewhere the build fails. Each safe crate depends on its
//! `-sys` crate only on macOS, so compiles everywhere.
//!
//! Internal to these crates; no stability promise.

use std::path::{Path, PathBuf};
use std::process::Command;

/// What differs between the `-sys` crates' builds.
pub struct Bridge {
  /// swift-bridge's name for the bridge: its generated header directory,
  /// which `bridging-header.h` imports, and glue file.
  pub bridge: &'static str,
  /// The Swift package in `swift/`, and its static library.
  pub package: &'static str,
  /// The framework the glue imports and the crate links.
  pub framework: &'static str,
}

/// The file holding the `#[swift_bridge::bridge]` module.
const BRIDGE_MODULE: &str = "src/lib.rs";

/// Never copied into the staged package: SwiftPM's build directory, and glue
/// this script regenerates.
const NOT_INPUTS: [&str; 2] = [".build", "generated"];

fn manifest_dir() -> PathBuf {
  PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"))
}

fn out_dir() -> PathBuf {
  PathBuf::from(std::env::var("OUT_DIR").expect("cargo sets OUT_DIR"))
}

/// The crate being built, which cargo names to its build script.
fn package_name() -> String {
  std::env::var("CARGO_PKG_NAME").expect("cargo sets CARGO_PKG_NAME")
}

fn source_package_dir() -> PathBuf {
  manifest_dir()
    .join("swift")
    .canonicalize()
    .expect("the swift package is committed inside this crate")
}

fn source_sources_dir(bridge: &Bridge) -> PathBuf {
  source_package_dir().join("Sources").join(bridge.package)
}

fn staged_package_dir() -> PathBuf {
  out_dir().join("swift")
}

/// SwiftPM's build directory, one per profile.
///
/// `OUT_DIR` is `<target>/<profile>/build/<crate>-<fingerprint>/out`, so the
/// profile directory is its fourth ancestor. Cargo documents no part of that
/// layout, so a path that doesn't match it falls back to `OUT_DIR`.
fn scratch_dir() -> PathBuf {
  let out = out_dir();
  let profile = out.ancestors().nth(3).filter(|_| {
    out
      .ancestors()
      .nth(2)
      .is_some_and(|dir| dir.ends_with("build"))
  });

  profile
    .unwrap_or(&out)
    .join(format!("{}-swift", package_name()))
}

fn is_release() -> bool {
  std::env::var("PROFILE").as_deref() == Ok("release")
}

/// Makes swift-bridge's generated `@_cdecl` shims public.
///
/// They are emitted internal. A release build optimises whole-module, sees no
/// caller inside the module (the callers are across the C ABI), and strips
/// them, leaving undefined `__swift_bridge__$…` symbols at link time. Debug
/// builds don't optimise, so `cargo test` never shows it.
fn publish_bridge_shims(bridge: &Bridge, generated: &Path) {
  let name = bridge.bridge;
  let path = generated.join(name).join(format!("{name}.swift"));
  let source = std::fs::read_to_string(&path).expect("swift-bridge should have just written the glue");
  let published = source.replace("\nfunc __swift_bridge__", "\npublic func __swift_bridge__");

  // A silent no-op would resurface as an unexplained release-only link failure.
  if published == source {
    panic!(
      "no `func __swift_bridge__` to publish in {}; swift-bridge's output has changed shape",
      path.display()
    );
  }

  // The glue names the framework's types, and a Swift import reaches only the
  // file it is written in.
  std::fs::write(&path, format!("import {}\n\n{published}", bridge.framework))
    .expect("the generated glue should be writable");
}

/// Mirrors `from` onto `to`, skipping unchanged files and removing anything
/// this run didn't write. SwiftPM keys off mtimes.
fn mirror(from: &Path, to: &Path, skip: &[&str]) {
  std::fs::create_dir_all(to).expect("the destination should be creatable");

  for entry in std::fs::read_dir(to).expect("the destination was just created") {
    let entry = entry.expect("a readable directory entry");

    if from.join(entry.file_name()).exists() {
      continue;
    }

    let stale = entry.path();
    let removed = if entry.file_type().expect("a stat-able entry").is_dir() {
      std::fs::remove_dir_all(&stale)
    } else {
      std::fs::remove_file(&stale)
    };

    removed.expect("a stale staged file should be removable");
  }

  for entry in std::fs::read_dir(from).expect("the source directory should be readable") {
    let entry = entry.expect("a readable directory entry");
    let name = entry.file_name();

    if skip.iter().any(|skipped| name == *skipped) {
      continue;
    }

    let destination = to.join(&name);

    if entry.file_type().expect("a stat-able entry").is_dir() {
      mirror(&entry.path(), &destination, skip);
      continue;
    }

    let fresh = std::fs::read(entry.path()).expect("a readable source file");

    if std::fs::read(&destination).is_ok_and(|current| current == fresh) {
      continue;
    }

    std::fs::write(&destination, fresh).expect("the staged file should be writable");
  }
}

/// Compiles the staged package to a static library.
///
/// The bridging header is set in the package's `swiftSettings`, which resolves
/// it against `#filePath`, so it follows the package to `OUT_DIR`.
fn compile_swift() {
  let mut command = Command::new("swift");

  command
    .arg("build")
    .arg("--package-path")
    .arg(staged_package_dir())
    .arg("--scratch-path")
    .arg(scratch_dir());

  if is_release() {
    command.args(["-c", "release"]);
  }

  let output = command.output().expect("swift should be on PATH on macOS");

  if !output.status.success() {
    panic!(
      "swift build failed\n--- stdout ---\n{}\n--- stderr ---\n{}",
      String::from_utf8_lossy(&output.stdout),
      String::from_utf8_lossy(&output.stderr),
    );
  }
}

/// Where `swift build` leaves the static library.
fn swift_build_dir() -> PathBuf {
  scratch_dir().join(if is_release() { "release" } else { "debug" })
}

/// Where the dynamic Swift runtime lives.
const SWIFT_RUNTIME_DIR: &str = "/usr/lib/swift";

/// The Swift runtime the static library depends on but does not carry, and
/// the framework it wraps.
fn link_swift_runtime(bridge: &Bridge) {
  let developer = Command::new("xcode-select")
    .arg("--print-path")
    .output()
    .ok()
    .filter(|output| output.status.success())
    .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    .unwrap_or_else(|| "/Applications/Xcode.app/Contents/Developer".to_string());

  println!("cargo:rustc-link-search={developer}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx/");
  println!("cargo:rustc-link-search={SWIFT_RUNTIME_DIR}");

  println!("cargo:rustc-link-lib=framework={}", bridge.framework);
}

/// Builds the calling `-sys` crate's Swift package and tells cargo how to link
/// it. Panics off macOS: the framework exists nowhere else.
pub fn build(bridge: Bridge) {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed={BRIDGE_MODULE}");

  if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
    panic!(
      "{} builds only for macOS: {}.framework exists nowhere else",
      package_name(),
      bridge.framework
    );
  }

  println!("cargo:rerun-if-changed={}", source_sources_dir(&bridge).display());
  println!(
    "cargo:rerun-if-changed={}",
    source_package_dir().join("Package.swift").display()
  );

  let staged = staged_package_dir();

  mirror(&source_package_dir(), &staged, &NOT_INPUTS);

  let glue = out_dir().join("swift-bridge");

  // `OUT_DIR` survives between builds.
  let _ = std::fs::remove_dir_all(&glue);

  swift_bridge_build::parse_bridges(vec![manifest_dir().join(BRIDGE_MODULE)])
    .write_all_concatenated(&glue, bridge.bridge);
  publish_bridge_shims(&bridge, &glue);
  mirror(
    &glue,
    &staged
      .join("Sources")
      .join(bridge.package)
      .join("generated"),
    &[],
  );

  compile_swift();

  println!("cargo:rustc-link-lib=static={}", bridge.package);
  println!("cargo:rustc-link-search={}", swift_build_dir().display());

  link_swift_runtime(&bridge);
}
