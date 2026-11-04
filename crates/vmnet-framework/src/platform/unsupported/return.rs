pub(crate) enum VmnetReturn {
  Success,
  Failure,
  MemFailure,
  InvalidArgument,
  SetupIncomplete,
  InvalidAccess,
  PacketTooBig,
  BufferExhausted,
  TooManyPackets,
  SharingServiceBusy,
  NotAuthorized,
  Unknown(u32),
}
