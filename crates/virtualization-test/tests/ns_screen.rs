//! On the main thread, where AppKit's members run (`main_thread_tests`).
//! Each needs a screen, so is named `screen::…`.

use virtualization::{MainThreadMarker, NSScreen};
use virtualization_test::main_thread_tests;

fn has_its_main_screen_among_its_screens(mtm: MainThreadMarker) {
  let main = format!("{:?}", NSScreen::main(mtm).unwrap());
  let screens: Vec<_> = NSScreen::screens(mtm)
    .iter()
    .map(|screen| format!("{screen:?}"))
    .collect();

  assert!(screens.contains(&main), "{main} in {screens:?}");
}

fn a_clone_is_the_same_object(mtm: MainThreadMarker) {
  let screen = NSScreen::main(mtm).unwrap();

  assert_eq!(format!("{:?}", screen.clone()), format!("{screen:?}"));
}

fn debugs_as_its_description(mtm: MainThreadMarker) {
  assert!(format!("{:?}", NSScreen::main(mtm).unwrap()).starts_with("<NSScreen: 0x"));
}

fn main() {
  main_thread_tests(&[
    (
      "screen::has_its_main_screen_among_its_screens",
      has_its_main_screen_among_its_screens,
    ),
    ("screen::a_clone_is_the_same_object", a_clone_is_the_same_object),
    ("screen::debugs_as_its_description", debugs_as_its_description),
  ]);
}
