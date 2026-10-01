# Serial ports, consoles, and sockets

Serial ports and their host attachments, Virtio console devices and ports (including the Spice agent port), and Virtio socket devices for host-guest communication without Ethernet or IP.

## VZSerialPortConfiguration

`class VZSerialPortConfiguration: NSObject, NSCopying`

Base class; use a subclass such as `VZVirtioConsoleDeviceSerialPortConfiguration`. Cannot be instantiated directly.

- `var attachment: VZSerialPortAttachment? { get set }`: how the guest's serial port interfaces with the host. Default `nil`.

## VZVirtioConsoleDeviceSerialPortConfiguration

`class VZVirtioConsoleDeviceSerialPortConfiguration: VZSerialPortConfiguration`

A single host-guest communication port on a Virtio console device.

- `init()`

## VZSerialPortAttachment

`class VZSerialPortAttachment: NSObject`

Base class defining how a guest serial port interfaces with the host; use a subclass. Cannot be instantiated directly.

## VZFileHandleSerialPortAttachment

`class VZFileHandleSerialPortAttachment: VZSerialPortAttachment`

- `init(fileHandleForReading: FileHandle?, fileHandleForWriting: FileHandle?)`: each handle must be `nil` or have a valid file descriptor.
- `var fileHandleForReading: FileHandle? { get }`: data written to it goes to the guest.
- `var fileHandleForWriting: FileHandle? { get }`: data sent from the guest appears on it.

## VZFileSerialPortAttachment

`class VZFileSerialPortAttachment: VZSerialPortAttachment`

Writes everything the guest sends to a file; sends nothing to the guest.

- `init(url: URL, append shouldAppend: Bool) throws`: `url` is a local file; `append` opens it in append mode.
- `var url: URL { get }`
- `var append: Bool { get }`

## VZConsoleDevice

`class VZConsoleDevice: NSObject`

In the machine's `consoleDevices` once created from a configuration, with the matching concrete type: a `VZVirtioConsoleDeviceConfiguration` yields a `VZVirtioConsoleDevice`. Cannot be instantiated directly.

## VZConsoleDeviceConfiguration

`class VZConsoleDeviceConfiguration: NSObject, NSCopying`

Base class, set on the machine configuration's `consoleDevices`; use a subclass such as `VZVirtioConsoleDeviceConfiguration`. Cannot be instantiated directly.

## VZConsolePortConfiguration

`class VZConsolePortConfiguration: NSObject, NSCopying`

Base class; use a subclass such as `VZVirtioConsolePortConfiguration`. Cannot be instantiated directly.

- `var attachment: VZSerialPortAttachment? { get set }`: how the console port interfaces with the host. Default `nil`. Accepts `VZFileHandleSerialPortAttachment`, `VZFileSerialPortAttachment`, and `VZSpiceAgentPortAttachment`.

## VZVirtioConsoleDeviceConfiguration

`class VZVirtioConsoleDeviceConfiguration: VZConsoleDeviceConfiguration, NSCopying`

A Virtio console device with one or more ports, each a `VZVirtioConsolePortConfiguration`.

- `init()`
- `var ports: VZVirtioConsolePortConfigurationArray { get }`

## VZVirtioConsolePortConfiguration

`class VZVirtioConsolePortConfiguration: VZConsolePortConfiguration, NSCopying`

A two-way channel between a host `VZSerialPortAttachment` and a guest console port.

- `init()`
- `var name: String? { get set }`: default no name.
- `var isConsole: Bool { get set }`: marks the port as the system console. Default `false`.

## VZVirtioConsolePortConfigurationArray

`class VZVirtioConsolePortConfigurationArray: NSObject, NSCopying`

A Virtio console device's port configurations; an entry's index is its port index in the virtual machine.

- `var maximumPortCount: UInt32 { get set }`: ports the device allocates. Default the number of ports attached; if unset, the highest indexed port is used. A set value must exceed the highest indexed port.
- `subscript(portIndex: Int) -> VZVirtioConsolePortConfiguration? { get set }`

## VZVirtioConsoleDevice

`class VZVirtioConsoleDevice: VZConsoleDevice`

Cannot be instantiated directly.

- `weak var delegate: (any VZVirtioConsoleDeviceDelegate)? { get set }`
- `var ports: VZVirtioConsolePortArray { get }`: the ports currently in use.

## VZVirtioConsoleDeviceDelegate

`protocol VZVirtioConsoleDeviceDelegate : NSObjectProtocol`

Notifies you when a guest process opens or closes a console port.

- `optional func consoleDevice(_ consoleDevice: VZVirtioConsoleDevice, didOpen consolePort: VZVirtioConsolePort)`: process or flush pending data from the port's attachment before communicating with the new guest process.
- `optional func consoleDevice(_ consoleDevice: VZVirtioConsoleDevice, didClose consolePort: VZVirtioConsolePort)`: finish processing or flushing remaining attachment data.

## VZVirtioConsolePort

`class VZVirtioConsolePort: NSObject`

Retrieved from `VZVirtioConsoleDevice.ports`. Cannot be instantiated directly.

- `var name: String? { get }`: fixed while the machine runs; `nil` if unnamed.
- `var attachment: VZSerialPortAttachment? { get set }`: can change any time while the machine runs.

## VZVirtioConsolePortArray

`class VZVirtioConsolePortArray: NSObject`

A `VZVirtioConsoleDevice`'s ports, by index.

- `var maximumPortCount: UInt32 { get }`: ports the device allocates.
- `subscript(portIndex: Int) -> VZVirtioConsolePort? { get }`

## VZSpiceAgentPortAttachment

`class VZSpiceAgentPortAttachment: VZSerialPortAttachment`

Connects a console port to a Spice guest agent, which shares the clipboard between host and guest.

- `init()`
- `var sharesClipboard: Bool { get set }`: advertises the clipboard capability to the agent so copy and paste events are shared. Enabled by default.
- `class var spiceAgentPortName: String { get }`: a console port with this name spawns a Spice guest agent if the guest supports it. Its `attachment` must be a `VZSpiceAgentPortAttachment` and `isConsole` must stay `false`.

## VZSocketDevice

`class VZSocketDevice: NSObject`

Base class; in the machine's `socketDevices` once created from a configuration, with the matching concrete type: a `VZVirtioSocketDeviceConfiguration` yields a `VZVirtioSocketDevice`. Cannot be instantiated directly.

## VZSocketDeviceConfiguration

`class VZSocketDeviceConfiguration: NSObject, NSCopying`

Base class; use a subclass such as `VZVirtioSocketDeviceConfiguration`. Cannot be instantiated directly.

## VZVirtioSocketDeviceConfiguration

`class VZVirtioSocketDeviceConfiguration: VZSocketDeviceConfiguration, NSCopying`

At most one Virtio socket device per virtual machine.

- `init()`

## VZVirtioSocketDevice

`class VZVirtioSocketDevice: VZSocketDevice`

Paravirtualized; moves data between guest and host without Ethernet or IP. Cannot be instantiated directly.

- `func setSocketListener(_ listener: VZVirtioSocketListener, forPort port: UInt32)`: one listener per port, replacing any existing one; a listener can serve several ports. Its delegate is called whenever the guest connects to the port.
- `func removeSocketListener(forPort port: UInt32)`: does nothing if the port has no listener.
- `func connect(toPort port: UInt32, completionHandler: @escaping (Result<VZVirtioSocketConnection, any Error>) -> Void)`: does nothing if the guest is not listening on the port. The handler is called once the connection is established or fails.
- `func connect(toPort port: UInt32) async throws -> VZVirtioSocketConnection`: `async` form of the above.

## VZVirtioSocketListener

`class VZVirtioSocketListener: NSObject`

Listens on one or more ports of a `VZVirtioSocketDevice`; its delegate is used when a guest connects.

- `weak var delegate: (any VZVirtioSocketListenerDelegate)? { get set }`

## VZVirtioSocketListenerDelegate

`protocol VZVirtioSocketListenerDelegate : NSObjectProtocol`

Decides whether to accept new guest connections.

- `optional func listener(_ listener: VZVirtioSocketListener, shouldAcceptNewConnection connection: VZVirtioSocketConnection, from socketDevice: VZVirtioSocketDevice) -> Bool`: `true` establishes the connection. The connection owns a file descriptor for sending and receiving data; keep a reference to an accepted connection to use it.

## VZVirtioSocketConnection

`class VZVirtioSocketConnection: NSObject`

Cannot be instantiated directly.

- `var destinationPort: UInt32 { get }`
- `var sourcePort: UInt32 { get }`
- `var fileDescriptor: Int32 { get }`: write to send, read to receive; `-1` when closed. The connection owns it and closes it when destroyed.
- `func close()`: closes the file descriptor.
