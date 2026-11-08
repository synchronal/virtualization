# io-usb-host

Rust bindings for Apple's [IOUSBHost] framework: `IOUSBHostDevice`, which
AccessoryAccess opens. Apple's API, nothing added.

- macOS 27+, Xcode, Rust 1.98+. Elsewhere, calls fail with
  `Error::Unsupported`.
- Re-exported by [`accessory-access`] and [`virtualization`].

Development and testing: see the [main README].

MIT licensed.

[IOUSBHost]: https://developer.apple.com/documentation/iousbhost
[`accessory-access`]: https://crates.io/crates/accessory-access
[`virtualization`]: https://crates.io/crates/virtualization
[main README]: https://github.com/synchronal/virtualization#development
