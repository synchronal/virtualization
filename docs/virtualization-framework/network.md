# Network

Network devices of a virtual machine: device configurations, the attachments that connect a device to the host, host interfaces available for bridging, and MAC addresses.

## VZNetworkDevice

`class VZNetworkDevice: NSObject`: a network device in a running virtual machine. Not created directly: configure one through a `VZNetworkDeviceConfiguration` subclass; it appears in the machine's `networkDevices` once the machine is created.

- `var attachment: VZNetworkDeviceAttachment? { get set }`: the currently connected attachment. Setting it attempts to change the attachment; on failure the virtual machine delegate's `virtualMachine(_:networkDevice:attachmentWasDisconnectedWithError:)` is called and the value becomes `nil`. Can also change at any time while the machine runs, depending on the host network's state.

## VZNetworkDeviceConfiguration

`class VZNetworkDeviceConfiguration: NSObject, NSCopying`: base class for a network adapter configuration. Not created directly; use a subclass such as `VZVirtioNetworkDeviceConfiguration`.

- `@NSCopying var macAddress: VZMACAddress { get set }`: defaults to a random, locally administered, unicast address.
- `var attachment: VZNetworkDeviceAttachment? { get set }`: how the virtual device interfaces with the host. Defaults to `nil`.

## VZNetworkDeviceAttachment

`class VZNetworkDeviceAttachment: NSObject`: base class for how a virtual network device interfaces with the host. Not created directly; use a subclass.

## VZBridgedNetworkDeviceAttachment

`class VZBridgedNetworkDeviceAttachment: VZNetworkDeviceAttachment`: bridges a host physical interface to a virtual network device; host and virtual machine share the interface but have distinct network layers. Requires the `com.apple.vm.networking` entitlement.

- `init(interface: VZBridgedNetworkInterface)`
- `var interface: VZBridgedNetworkInterface { get }`

## VZBridgedNetworkInterface

`class VZBridgedNetworkInterface: NSObject`: a host network interface that can be bridged. Not created directly; get instances from `networkInterfaces`.

- `class var networkInterfaces: [VZBridgedNetworkInterface] { get }`
- `var identifier: String { get }`: BSD name, such as "en0".
- `var localizedDisplayName: String? { get }`: display name such as "Ethernet", if available.

## VZFileHandleNetworkDeviceAttachment

`class VZFileHandleNetworkDeviceAttachment: VZNetworkDeviceAttachment`: sends raw data-link-layer frames between the virtual network interface and a file handle.

- `init(fileHandle: FileHandle)`: the handle must hold a connected datagram socket.
- `var fileHandle: FileHandle { get }`
- `var maximumTransmissionUnit: Int { get set }`: defaults to 1500; must be between 1500 and 65535, otherwise the virtual machine configuration is invalid. The socket's client side must set `SO_SNDBUF` and `SO_RCVBUF` with `setsockopt`: `SO_RCVBUF` at least twice `SO_SNDBUF`, ideally four times.

## VZNATNetworkDeviceAttachment

`class VZNATNetworkDeviceAttachment: VZNetworkDeviceAttachment`: connects to outside networks using network address translation, with the host acting as router.

- `init()`

## VZVmnetNetworkDeviceAttachment

`class VZVmnetNetworkDeviceAttachment: VZNetworkDeviceAttachment`: backed by a vmnet logical network, allowing custom topologies and letting several virtual machines appear on the same network.

- `init(network: vmnet_network_ref)`: the network must be created by the same application process; otherwise starting the interface fails. Creating or configuring a vmnet network requires a vmnet entitlement.
- `var network: vmnet_network_ref { get }`

## VZVirtioNetworkDeviceConfiguration

`class VZVirtioNetworkDeviceConfiguration: VZNetworkDeviceConfiguration, NSCopying`: configuration of a paravirtualized Virtio network device. The host communication channel is set through `attachment`; valid only with a valid `macAddress` and `attachment`. Add it to the machine configuration's `networkDevices`.

- `init()`

## VZMACAddress

`class VZMACAddress: NSObject, NSCopying`: a 48-bit Ethernet (MAC) address.

- `init(ethernetAddress: ether_addr_t)`
- `convenience init?(string: String)`: six hexadecimal bytes separated by colons, such as "01:23:45:ab:cd:ef", in either case. `nil` if malformed.
- `class func randomLocallyAdministered() -> Self`: a valid, random, unicast, locally administered address. Not guaranteed to be unique.
- `var ethernetAddress: ether_addr_t { get }`
- `var string: String { get }`: lowercase hexadecimal bytes separated by colons; accepted by `init?(string:)`.
- `var isBroadcastAddress: Bool { get }`
- `var isMulticastAddress: Bool { get }`
- `var isUnicastAddress: Bool { get }`
- `var isLocallyAdministeredAddress: Bool { get }`
- `var isUniversallyAdministeredAddress: Bool { get }`
