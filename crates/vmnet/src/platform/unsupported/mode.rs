pub(crate) enum VmnetMode {
  Host,
  Shared,
  Bridged,
  Unknown(u32),
}
