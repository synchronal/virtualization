use super::{GUARDED, Local, Never};

#[derive(Clone, Copy)]
pub(crate) struct MainThreadMarker {
  never: Never,
  local: Local,
}

impl MainThreadMarker {
  pub(crate) fn new() -> Option<MainThreadMarker> {
    unreachable!("{GUARDED}")
  }

  pub(crate) fn never(&self) -> ! {
    match self.never {}
  }
}

pub(crate) struct NSScreen {
  never: Never,
  local: Local,
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
