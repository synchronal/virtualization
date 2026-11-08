# disk-image-kit

Rust bindings for Apple's [DiskImageKit] framework: creating, opening, and
stacking disk images. Apple's API, nothing added.

- macOS 27+, Xcode, Rust 1.98+. Elsewhere, calls fail with
  `Error::Unsupported`.
- Re-exported by [`virtualization`].

Development and testing: see the [main README].

MIT licensed.

[DiskImageKit]: https://developer.apple.com/documentation/diskimagekit
[`virtualization`]: https://crates.io/crates/virtualization
[main README]: https://github.com/synchronal/virtualization#development
