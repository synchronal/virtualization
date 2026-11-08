# virtualization

Rust bindings for Apple's [Virtualization] framework, for creating and running
Linux and macOS virtual machines on a Mac.

This project provides safe access to Apple's APIs, as close to the upstream
API as possible. Types and methods mirror Virtualization, without providing
extra functionality.

Re-exports [`disk-image-kit`], [`vmnet-framework`], [`accessory-access`], and
[`io-usb-host`].

- macOS 27+, Xcode, Rust 1.98+. Elsewhere, calls fail with
  `Error::Unsupported`.
- Binaries need the `com.apple.security.virtualization` entitlement.

Development and testing: see the [main README].

MIT licensed.

[Virtualization]: https://developer.apple.com/documentation/virtualization
[`disk-image-kit`]: https://crates.io/crates/disk-image-kit
[`vmnet-framework`]: https://crates.io/crates/vmnet-framework
[`accessory-access`]: https://crates.io/crates/accessory-access
[`io-usb-host`]: https://crates.io/crates/io-usb-host
[main README]: https://github.com/synchronal/virtualization#development
