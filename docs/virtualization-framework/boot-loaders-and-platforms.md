# Boot Loaders and Platforms

Boot loaders start a virtual machine from a Linux kernel or an EFI ROM; platform configurations describe its hardware identity. Also: the EFI variable store and Secure Boot signatures.

## VZBootLoader

`class VZBootLoader: NSObject, NSCopying`: Abstract base class; a virtual machine configuration requires one. Use `VZEFIBootLoader`, `VZLinuxBootLoader`, or `VZMacOSBootLoader`.

## VZLinuxBootLoader

`class VZLinuxBootLoader: VZBootLoader, NSCopying`: Boots a Linux kernel. Use only with `VZGenericPlatformConfiguration`.

- `init(kernelURL: URL)`: Kernel on the local file system.
- `var kernelURL: URL { get set }`
- `var commandLine: String { get set }`: Parameters passed to the kernel on boot.
- `var initialRamdiskURL: URL? { get set }`: Initial RAM disk, mapped into memory before the kernel boots.

## VZEFIBootLoader

`class VZEFIBootLoader: VZBootLoader, NSCopying`: For guests that expect an EFI ROM. Use only with `VZGenericPlatformConfiguration`.

- `init()`
- `var variableStore: VZEFIVariableStore? { get set }`

## VZEFIVariableStore

`class VZEFIVariableStore: NSObject`: The NVRAM variables exposed by the EFI ROM.

- `init(url URL: URL)`: Opens an existing store file on the local file system.
- `init(creatingVariableStoreAt URL: URL, options: VZEFIVariableStore.InitializationOptions = []) throws`: Writes a new store to the URL.
- `var url: URL { get }`: Location on the local file system.
- `func enableSecureBootUsingDefaultPlatformKey() throws`: macOS 27. Replaces the Platform Key with an Apple-managed one, sets `SetupMode` to `0`, and enables Secure Boot.
- `func enableSecureBoot(platformKey: SecCertificate) throws`: macOS 27. Replaces the Platform Key with the given X.509 certificate, sets `SetupMode` to `0`, and enables Secure Boot.
- `func disableSecureBoot() throws`: macOS 27. Disables Secure Boot and leaves the applied configuration unchanged.
- `func resetSecureBoot() throws`: macOS 27. Clears any applied Secure Boot configuration, sets `SetupMode` to `1`, and disables Secure Boot.
- `var isSecureBootEnabled: Bool { get throws }`: macOS 27.
- `func enrollDefaultSecureBootSignatures() throws`: macOS 27. Adds the Microsoft Key Exchange Keys to KEK, the UEFI CA signatures to db, and the latest UEFI revocation list to dbx. Works before or after enrolling a Platform Key, which is preserved. Lets Microsoft-signed Linux distributions boot with Secure Boot enabled.
- `func enrollSecureBootSignatures(_ signatures: VZEFISignatureDatabaseConfiguration) throws`: macOS 27. Appends to KEK, db, and dbx, skipping signatures already present; preserves the Platform Key. Can be called repeatedly. Invalid signatures can leave the guest unbootable.
- `var enrolledSecureBootSignatures: VZEFISignatureDatabaseConfiguration { get throws }`: macOS 27. Current KEK, db, and dbx contents.

## VZEFIVariableStore.InitializationOptions

`struct InitializationOptions`: Options for creating a variable store.

- `init(rawValue: UInt)`
- `static var allowOverwrite: VZEFIVariableStore.InitializationOptions { get }`: Raw value `1 << 0`.

## VZEFISignature

`enum VZEFISignature: Sendable`: macOS 27. A UEFI signature.

- `case sha256(Data)`: Exactly 32 bytes, or an exception is raised. dbx uses hashes to block revoked or vulnerable UEFI binaries.
- `case x509Certificate(SecCertificate)`: DER-encoded X.509. Commonly used by the Platform Key, KEK, and db.

## VZEFISignatureList

`class VZEFISignatureList: NSObject, NSCopying, Sendable`: macOS 27. X.509 certificates or SHA-256 hashes, as stored in KEK, db, and dbx.

- `init(contentsOf url: URL) throws`: Auto-detects the format: a DER-encoded X.509 certificate (one entry), binary UEFI signature lists (all signatures extracted), or a SHA-256 hash (one entry).
- `convenience init(signatures: [VZEFISignature])`: May mix certificates and hashes.
- `var signatures: [VZEFISignature] { get }`

## VZEFISignatureDatabaseConfiguration

`class VZEFISignatureDatabaseConfiguration: NSObject, NSCopying, Sendable`: macOS 27. Signature lists for the three Secure Boot databases.

- `init(keyExchangeKeys: [VZEFISignatureList], dbSignatures: [VZEFISignatureList], dbxSignatures: [VZEFISignatureList])`: Arrays may be empty. `keyExchangeKeys` takes only X.509 certificates; a SHA-256 hash raises an exception. Malformed data can corrupt the variable store and leave the guest unbootable.
- `var keyExchangeKeys: [VZEFISignatureList] { get }`: KEK. Keys authorized to update db and dbx.
- `var dbSignatures: [VZEFISignatureList] { get }`: db. Allow list of trusted UEFI applications, boot loaders, and drivers.
- `var dbxSignatures: [VZEFISignatureList] { get }`: dbx. Deny list that blocks matching code even if db allows it.

## VZPlatformConfiguration

`class VZPlatformConfiguration: NSObject, NSCopying`: Abstract base class. Use `VZGenericPlatformConfiguration` or `VZMacPlatformConfiguration`.

## VZGenericPlatformConfiguration

`class VZGenericPlatformConfiguration: VZPlatformConfiguration, NSCopying`: Platform for a generic Intel or ARM virtual machine.

- `init()`
- `@NSCopying var machineIdentifier: VZGenericMachineIdentifier { get set }`: When loading a saved machine, must match its original identifier. Give each machine from the same configuration a unique one; running two with the same identifier at once gives undefined guest behavior.
- `class var isNestedVirtualizationSupported: Bool { get }`: Only some hardware and software configurations support it, and policy can disable it.
- `var isNestedVirtualizationEnabled: Bool { get set }`: Default `false`. Check `isNestedVirtualizationSupported` first.

## VZGenericMachineIdentifier

`class VZGenericMachineIdentifier: NSObject, NSCopying`: Lets guests tell virtual hardware apart. Equal when contents match.

- `init()`: New unique identifier.
- `init?(dataRepresentation: Data)`: `nil` if the data is invalid.
- `var dataRepresentation: Data { get }`: Opaque data for saving to disk.
