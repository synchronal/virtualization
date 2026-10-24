//! The safe crates' check that their types are `Send` and `Sync` off macOS
//! exactly as on it, where the sys crates decide. For their tests only.

/// Compiles only if each type in `send_sync` is `Send + Sync`, and each in
/// `local` is neither.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_auto_traits {
  (send_sync: [$($send_sync:ty),* $(,)?], local: [$($local:ty),* $(,)?] $(,)?) => {
    const _: () = {
      // Naming `item` is ambiguous, so fails, where both impls apply.
      trait AmbiguousIfSend<A> {
        fn item() {}
      }
      impl<T: ?Sized> AmbiguousIfSend<()> for T {}
      impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

      trait AmbiguousIfSync<A> {
        fn item() {}
      }
      impl<T: ?Sized> AmbiguousIfSync<()> for T {}
      impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

      fn send_sync<T: ?Sized + Send + Sync>() {}

      $(let _ = send_sync::<$send_sync>;)*
      $(
        let _ = <$local as AmbiguousIfSend<_>>::item;
        let _ = <$local as AmbiguousIfSync<_>>::item;
      )*
    };
  };
}
