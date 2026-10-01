# Graphics, input, and audio

Graphics devices and their displays (Mac and Virtio), keyboards, pointing devices, and Virtio sound with its host audio sources and sinks. Configurations go on the virtual machine configuration; running devices and displays come from the virtual machine built from it.

## VZGraphicsDevice

`class VZGraphicsDevice: NSObject`

In a running virtual machine, from `VZVirtualMachine.graphicsDevices`. Cannot be created directly; its type matches the configuration that produced it.

- `var displays: [VZGraphicsDisplay] { get }`: One per configured display or scanout, in configuration order.

## VZMacGraphicsDevice

`class VZMacGraphicsDevice: VZGraphicsDevice`

Running counterpart of `VZMacGraphicsDeviceConfiguration`. Its displays are `VZMacGraphicsDisplay`.

## VZVirtioGraphicsDevice

`class VZVirtioGraphicsDevice: VZGraphicsDevice`

Running counterpart of `VZVirtioGraphicsDeviceConfiguration`. Its displays are `VZVirtioGraphicsScanout`.

## VZGraphicsDeviceConfiguration

`class VZGraphicsDeviceConfiguration: NSObject, NSCopying`

Cannot be created directly.

## VZMacGraphicsDeviceConfiguration

`class VZMacGraphicsDeviceConfiguration: VZGraphicsDeviceConfiguration, NSCopying`

Its display can be shown in a `VZVirtualMachineView`.

- `init()`
- `var displays: [VZMacGraphicsDisplayConfiguration] { get set }`: At most one display is supported.

## VZVirtioGraphicsDeviceConfiguration

`class VZVirtioGraphicsDeviceConfiguration: VZGraphicsDeviceConfiguration, NSCopying`

Paravirtualized, per the Virtio GPU Device specification. Its scanout can be shown in a `VZVirtualMachineView`.

- `init()`
- `var scanouts: [VZVirtioGraphicsScanoutConfiguration] { get set }`: At most one scanout is supported.

## VZGraphicsDisplay

`class VZGraphicsDisplay: NSObject`

A display of a running graphics device. Cannot be created directly.

- `var sizeInPixels: CGSize { get }`
- `func reconfigure(sizeInPixels: CGSize) throws`: Throws if the size is invalid. The guest may ignore the new size without an error. Triggers the observer reconfiguration callbacks.
- `func reconfigure(configuration: VZGraphicsDisplayConfiguration) throws`: Must be of the type that created this display. Throws if invalid. The guest may ignore parts of it without an error. Triggers the observer reconfiguration callbacks.
- `func addObserver(_ observer: any VZGraphicsDisplayObserver)`
- `func removeObserver(_ observer: any VZGraphicsDisplayObserver)`

## VZGraphicsDisplayObserver

`protocol VZGraphicsDisplayObserver : NSObjectProtocol`

Observes display state changes. Called on the virtual machine's queue.

- `optional func displayDidBeginReconfiguration(_ display: VZGraphicsDisplay)`: A size or configuration change was issued; frames reflecting it are expected.
- `optional func displayDidEndReconfiguration(_ display: VZGraphicsDisplay)`: Frames have arrived at the most recently requested size and configuration.

## VZMacGraphicsDisplay

`class VZMacGraphicsDisplay: VZGraphicsDisplay`

Running counterpart of `VZMacGraphicsDisplayConfiguration`.

- `var pixelsPerInch: Int { get }`

## VZVirtioGraphicsScanout

`class VZVirtioGraphicsScanout: VZGraphicsDisplay`

Running counterpart of `VZVirtioGraphicsScanoutConfiguration`.

## VZGraphicsDisplayConfiguration

`class VZGraphicsDisplayConfiguration: NSObject, NSCopying`

Cannot be created directly.

## VZMacGraphicsDisplayConfiguration

`class VZMacGraphicsDisplayConfiguration: VZGraphicsDisplayConfiguration, NSCopying`

Attached to a Mac graphics device; can be shown in a `VZVirtualMachineView`.

- `init(widthInPixels: Int, heightInPixels: Int, pixelsPerInch: Int)`
- `convenience init(for screen: NSScreen, sizeInPoints: NSSize)`: Derives pixel dimensions and density from the screen and the intended logical size. The macOS guest may not offer a display mode whose backing scale factor matches the screen.
- `var widthInPixels: Int { get set }`
- `var heightInPixels: Int { get set }`
- `var pixelsPerInch: Int { get set }`

## VZVirtioGraphicsScanoutConfiguration

`class VZVirtioGraphicsScanoutConfiguration: VZGraphicsDisplayConfiguration, NSCopying`

Attached to a Virtio graphics device; can be shown in a `VZVirtualMachineView`.

- `init(widthInPixels: Int, heightInPixels: Int)`
- `var widthInPixels: Int { get set }`
- `var heightInPixels: Int { get set }`

## VZKeyboardConfiguration

`class VZKeyboardConfiguration: NSObject, NSCopying`

Cannot be created directly.

## VZUSBKeyboardConfiguration

`class VZUSBKeyboardConfiguration: VZKeyboardConfiguration, NSCopying`

`VZVirtualMachineView` sends key events through it.

- `init()`

## VZMacKeyboardConfiguration

`class VZMacKeyboardConfiguration: VZKeyboardConfiguration, NSCopying`

Has Apple-specific features such as the globe key. `VZVirtualMachineView` sends keyboard events through it. Apple silicon only.

Only guests running macOS 13 or later recognize it. To support earlier guests too, put a `VZMacKeyboardConfiguration` and a `VZUSBKeyboardConfiguration` in `VZVirtualMachineConfiguration.keyboards`.

- `init()`

## VZPointingDeviceConfiguration

`class VZPointingDeviceConfiguration: NSObject, NSCopying`

Cannot be created directly.

## VZUSBScreenCoordinatePointingDeviceConfiguration

`class VZUSBScreenCoordinatePointingDeviceConfiguration: VZPointingDeviceConfiguration, NSCopying`

Reports absolute coordinates. `VZVirtualMachineView` sends pointer events through it.

- `init()`

## VZMacTrackpadConfiguration

`class VZMacTrackpadConfiguration: VZPointingDeviceConfiguration, NSCopying`

`VZVirtualMachineView` sends pointer events and multi-touch trackpad gestures through it. Apple silicon only.

Only guests running macOS 13 or later recognize it. To support earlier guests too, put a `VZMacTrackpadConfiguration` and a `VZUSBScreenCoordinatePointingDeviceConfiguration` in `VZVirtualMachineConfiguration.pointingDevices`.

- `init()`

## VZAudioDeviceConfiguration

`class VZAudioDeviceConfiguration: NSObject, NSCopying`

Cannot be created directly; use `VZVirtioSoundDeviceConfiguration`.

## VZVirtioSoundDeviceConfiguration

`class VZVirtioSoundDeviceConfiguration: VZAudioDeviceConfiguration, NSCopying`

Exposes a source or destination of sound.

- `init()`
- `var streams: [VZVirtioSoundDeviceStreamConfiguration] { get set }`: Empty by default.

## VZVirtioSoundDeviceStreamConfiguration

`class VZVirtioSoundDeviceStreamConfiguration: NSObject, NSCopying`

A PCM audio stream. Cannot be created directly; use the input or output subclass.

## VZVirtioSoundDeviceInputStreamConfiguration

`class VZVirtioSoundDeviceInputStreamConfiguration: VZVirtioSoundDeviceStreamConfiguration, NSCopying`

Input audio, such as from a microphone.

- `init()`
- `var source: VZAudioInputStreamSource? { get set }`: Supplies the audio on the host. Default `nil` produces silence.

## VZVirtioSoundDeviceOutputStreamConfiguration

`class VZVirtioSoundDeviceOutputStreamConfiguration: VZVirtioSoundDeviceStreamConfiguration, NSCopying`

Output audio, such as to a speaker.

- `init()`
- `var sink: VZAudioOutputStreamSink? { get set }`: Handles the guest's audio on the host. Default `nil` discards it.

## VZAudioInputStreamSource

`class VZAudioInputStreamSource: NSObject`

How the host produces a guest's audio input. Cannot be created directly; use `VZHostAudioInputStreamSource`.

## VZAudioOutputStreamSink

`class VZAudioOutputStreamSink: NSObject`

How the host consumes a guest's audio. Cannot be created directly; use `VZHostAudioOutputStreamSink`.

## VZHostAudioInputStreamSource

`class VZHostAudioInputStreamSource: VZAudioInputStreamSource`

Provides audio from the host's default input device, the same one `AudioQueueNewInput` uses.

- `init()`

## VZHostAudioOutputStreamSink

`class VZHostAudioOutputStreamSink: VZAudioOutputStreamSink`

Plays audio to the host's default output device, the same one `AudioQueueNewOutput` uses.

- `init()`
