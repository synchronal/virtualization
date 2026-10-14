//===----------------------------------------------------------------------===//
// `vmnet_return_t`.
//===----------------------------------------------------------------------===//

import vmnet

/// What a call returning `nil` reports through its `status`.
extension BridgedVmnetReturn: @unchecked Sendable, Error {}

extension BridgedVmnetReturn {
  init(_ value: vmnet_return_t) {
    switch value {
    case .VMNET_SUCCESS: self = .Success
    case .VMNET_FAILURE: self = .Failure
    case .VMNET_MEM_FAILURE: self = .MemFailure
    case .VMNET_INVALID_ARGUMENT: self = .InvalidArgument
    case .VMNET_SETUP_INCOMPLETE: self = .SetupIncomplete
    case .VMNET_INVALID_ACCESS: self = .InvalidAccess
    case .VMNET_PACKET_TOO_BIG: self = .PacketTooBig
    case .VMNET_BUFFER_EXHAUSTED: self = .BufferExhausted
    case .VMNET_TOO_MANY_PACKETS: self = .TooManyPackets
    case .VMNET_SHARING_SERVICE_BUSY: self = .SharingServiceBusy
    case .VMNET_NOT_AUTHORIZED: self = .NotAuthorized
    @unknown default: self = .Unknown(value.rawValue)
    }
  }
}
