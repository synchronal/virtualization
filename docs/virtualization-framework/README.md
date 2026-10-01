# Virtualization framework

Apple's Virtualization framework as of the macOS 27 SDK, in Swift.

- [Essentials](essentials.md): virtual machines, configurations, delegates, start options, views, errors, guest memory
- [Boot loaders and platforms](boot-loaders-and-platforms.md): Linux and EFI boot loaders, EFI variable stores and signatures, the generic platform
- [macOS guests](macos-guests.md): macOS boot loader, Mac platform, hardware models, installer, restore images
- [Storage](storage.md): Virtio block, NVMe, and USB mass storage devices; disk image, block device, and NBD attachments
- [Network](network.md): network devices; NAT, bridged, file handle, and vmnet attachments; MAC addresses
- [Serial ports, consoles, and sockets](serial-console-and-sockets.md): serial ports, Virtio consoles and ports, the Spice agent, Virtio sockets
- [Graphics, input, and audio](graphics-input-and-audio.md): Mac and Virtio graphics, displays, keyboards, pointing devices, Virtio sound
- [Directory sharing](directory-sharing.md): Virtio file system devices, shared directories, Rosetta
- [Entropy, memory balloon, and USB](other-devices.md): entropy and memory balloon devices, USB controllers and devices
- [Custom Virtio devices](custom-virtio-devices.md): custom Virtio devices, queues, feature sets, shared memory
