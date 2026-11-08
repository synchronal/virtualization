# virtualization

Rust bindings for Apple's [Virtualization] framework, for creating and running
Linux and macOS virtual machines on a Mac.

This project provides safe access to Apple's APIs, as close to the upstream
API as possible. Types and methods mirror Virtualization, without providing
extra functionality.

It also binds other frameworks that virtual machines need: [DiskImageKit] for
disk images, [vmnet] for networks, [AccessoryAccess] for USB accessories, and
[IOUSBHost] for the USB devices AccessoryAccess opens.

## Examples

[`examples/`](examples) has programs using the crates.
[`boot_linux.rs`](examples/boot_linux.rs) starts a Linux virtual machine from a
kernel image:

```sh
cargo build -p virtualization-test --example boot_linux
bin/dev/sign target/debug/examples/boot_linux
target/debug/examples/boot_linux /path/to/vmlinux
```

Virtualization won't start a machine without the
`com.apple.security.virtualization` entitlement, so binaries must be signed
after each build:

```sh
codesign --force --entitlements virtualization.entitlements \
  --sign - target/debug/your-binary
```

## Requirements

- macOS 27 or later. Every Virtualization API is available, without
  availability checks.
- Xcode, for the Swift toolchain used by the build (found via `xcrun`).
- Rust 1.98 or later.

Crates compile on other platforms, but stub each API call to fail with
`Error::Unsupported`.

## How it works

Much of Apple's API is Swift-only (DiskImageKit has no Objective-C at all), so
bindings go through Swift rather than through the Objective-C runtime. Each
framework has a small Swift package that forwards calls one-for-one, connected
to Rust via [swift-bridge]. Foundation types come from [objc2] and dispatch
queues from [dispatch2].

Each framework is a pair of crates: a `-sys` crate that holds the Swift
package, the bridge, and all the `unsafe` code, and a safe crate on top that
forbids `unsafe`.

## Crates

| Crate | What it is |
| --- | --- |
| `virtualization` | Virtualization: configurations, virtual machines, devices, boot loaders. |
| `disk-image-kit` | DiskImageKit: creating and opening disk images. |
| `vmnet-framework` | vmnet's networks, which a virtual machine can attach to. (`vmnet` on crates.io is a different project.) |
| `accessory-access` | AccessoryAccess: finding USB accessories and passing them through to a virtual machine. |
| `io-usb-host` | IOUSBHost's `IOUSBHostDevice`, which AccessoryAccess opens. |
| `*-sys` | The Swift package and swift-bridge FFI behind each of the above. Use the safe crate instead. |
| `apple-bridge-foundation` | The Foundation glue the `-sys` crates share: objc2 objects, and `NSError`, `NSMutableData`, and property list values as plain Rust. |
| `apple-bridge-build` | Build scripts shared by `-sys` crates: generates swift-bridge's glue, builds the Swift package, and links it. |
| `virtualization-test` | Integration test suites, and the owner of `examples/`. Not published. |

Depend on `virtualization`, plus the safe crates whose types you use with it.

## Development

### Setup

Development tasks run through [medic], which checks that your machine has what
the project needs and specifies all checks required to ship changes. It is
available via a custom Homebrew tap:

```sh
brew bundle
```

Then let medic check your setup:

```sh
medic doctor
medic test
medic audit
medic update
medic shipit
```

Check out its config in `.config/medic.toml`.

You'll also need Xcode, and `swift-format` on your `PATH` for formatting and
linting the Swift.

### Integration tests

Integration suites in `crates/virtualization-test` boot real virtual
machines, so require a Mac and some setup, which `medic test` handles:

- **Signing.** Cargo builds test binaries unsigned, and Virtualization refuses
  to run without its entitlement. `bin/dev/test-integration` builds the suites,
  signs each binary with `virtualization.entitlements`, and runs them with
  nextest. Extra arguments go to nextest, so one test runs with
  `bin/dev/test-integration -E 'test(name)'`. Signing is ad hoc unless you set
  `DEVELOPMENT_TEAM` to your team ID. Set `DEVELOPMENT_TEAM` to keep code identity
  stable across builds.
- **A kernel and busybox.** The first run downloads a Linux kernel and a static
  busybox into `~/.cache/virtualization-tests`, checked against pinned
  hashes. Guests boot from them.
- **Network access.** Some suites fetch Apple's catalog of macOS restore images
  to learn the latest hardware model. No restore image is downloaded.
- **A screen.** A few tests use AppKit and need a logged-in Mac with a display.
  They're named `screen::…`, skipped by default, and run with
  `bin/dev/test-integration --profile screen`.

Suites are signed only with `com.apple.security.virtualization`. So bridged
networking, which needs a provisioning profile from Apple, is tested only
failing validation. USB passthrough, which needs a real accessory and an
app entitlement, is automatically tested only as far as possible without
entitlement.

### Other scripts

- `bin/dev/swift-interface` prints how Swift imports a framework from the
  macOS SDK, for APIs Apple documents only in Objective-C.
- `bin/dev/format-swift --all` formats the Swift sources; `bin/dev/lint-swift`
  lints them.
- `bin/dev/sign <binary>` signs any binary with the virtualization
  entitlement.

`docs/` holds a copy of Apple's documentation for each framework, which is the
reference for what each type has.

## License

MIT. See [LICENSE.md](LICENSE.md).

[AccessoryAccess]: https://developer.apple.com/documentation/accessoryaccess
[DiskImageKit]: https://developer.apple.com/documentation/diskimagekit
[IOUSBHost]: https://developer.apple.com/documentation/iousbhost
[Virtualization]: https://developer.apple.com/documentation/virtualization
[dispatch2]: https://crates.io/crates/dispatch2
[medic]: https://github.com/synchronal/medic-rs
[objc2]: https://github.com/madsmtm/objc2
[swift-bridge]: https://github.com/chinedufn/swift-bridge
[vmnet]: https://developer.apple.com/documentation/vmnet
