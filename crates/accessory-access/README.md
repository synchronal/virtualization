# accessory-access

Rust bindings for Apple's [AccessoryAccess] framework: finding USB accessories
to pass through to a virtual machine. Apple's API, nothing added.

- macOS 27+, Xcode, Rust 1.98+. Elsewhere, calls fail with
  `Error::Unsupported`.
- Needs the `com.apple.developer.accessory-access.usb` entitlement and an app
  with a UI.
- An opened accessory is an [`io-usb-host`] device.

Development and testing: see the [main README].

MIT licensed.

[AccessoryAccess]: https://developer.apple.com/documentation/accessoryaccess
[`io-usb-host`]: https://crates.io/crates/io-usb-host
[main README]: https://github.com/synchronal/virtualization#development
