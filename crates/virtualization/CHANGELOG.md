# Changelog

## Unreleased

- Breaking: stop re-exporting `disk_image_kit`, `vmnet_framework`,
  `accessory_access`, `io_usb_host`, `dispatch2`, `url`, and `uuid`. Depend
  on them directly.

## v0.1.1

- Add a README and changelog.

## v0.1.0

- Virtual machines: configuration, validation, start/stop/pause/resume,
  save/restore, delegate.
- Boot loaders: Linux, EFI (variable stores, signature databases), macOS.
- Platforms: generic and Mac (hardware models, machine identifiers, auxiliary
  storage).
- macOS guests: restore images, configuration requirements, installer,
  provisioning options.
- Storage: Virtio block, NVMe, USB mass storage; disk image, block device, and
  NBD attachments.
- Networking: Virtio network; NAT, bridged, file handle, and vmnet
  attachments.
- Serial ports and consoles: Virtio console ports, file and file handle
  attachments, SPICE agent.
- Sockets: Virtio vsock connections and listeners.
- Directory sharing: Virtio file system, single and multiple shares.
- Graphics: Mac and Virtio displays, display observers, `VirtualMachineView`.
- Input: USB and Mac keyboards, USB pointing device, Mac trackpad.
- Audio: Virtio sound, host input and output streams.
- Other devices: entropy, memory balloon, XHCI controller, USB passthrough.
- Custom Virtio devices: queues, elements, shared memory, guest memory
  mapping.
