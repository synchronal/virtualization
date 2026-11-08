# vmnet-framework

Rust bindings for Apple's [vmnet] framework: the networks a virtual machine
attaches to. Apple's API, nothing added. (`vmnet` on crates.io is another
project.)

- macOS 27+, Xcode, Rust 1.98+. Elsewhere, calls fail with
  `Error::Unsupported`.
- Re-exported by [`virtualization`].

Development and testing: see the [main README].

MIT licensed.

[vmnet]: https://developer.apple.com/documentation/vmnet
[`virtualization`]: https://crates.io/crates/virtualization
[main README]: https://github.com/synchronal/virtualization#development
