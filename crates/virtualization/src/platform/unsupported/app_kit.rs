use super::{GUARDED, Never};

#[derive(Clone, Copy)]
pub(crate) struct MainThreadMarker {
  never: Never,
}

impl MainThreadMarker {
  pub(crate) fn new() -> Option<MainThreadMarker> {
    unreachable!("{GUARDED}")
  }
}

pub(crate) struct NSScreen {
  never: Never,
}

pub(crate) fn ns_screen_screens(mtm: MainThreadMarker) -> Vec<NSScreen> {
  match mtm.never {}
}

pub(crate) fn ns_screen_main(mtm: MainThreadMarker) -> Option<NSScreen> {
  match mtm.never {}
}

impl NSScreen {
  pub(crate) fn bridge_retained(&self) -> NSScreen {
    match self.never {}
  }

  pub(crate) fn ns_description(&self) -> String {
    match self.never {}
  }
}
