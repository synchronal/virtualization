//! On the main thread, where AppKit's members run (`main_thread_tests`), for
//! the screen a display is made for. Assumes a Mac with a screen.

use virtualization::{CGSize, MacGraphicsDisplayConfiguration, MainThreadMarker, NSScreen};
use virtualization_test::main_thread_tests;

fn has_the_size_and_density_it_was_made_with(_mtm: MainThreadMarker) {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  assert_eq!(display.width_in_pixels(), 1920);
  assert_eq!(display.height_in_pixels(), 1080);
  assert_eq!(display.pixels_per_inch(), 80);
}

/// Its pixels are the size in points at the screen's backing scale.
fn is_made_for_a_size_in_points_on_a_screen(mtm: MainThreadMarker) {
  let display = MacGraphicsDisplayConfiguration::with_for_and_size_in_points(
    &NSScreen::main(mtm).unwrap(),
    CGSize {
      width: 1440.0,
      height: 900.0,
    },
  )
  .unwrap();
  let scale = display.width_in_pixels() / 1440;

  assert!(scale >= 1, "{display:?}");
  assert_eq!(display.width_in_pixels(), 1440 * scale);
  assert_eq!(display.height_in_pixels(), 900 * scale);
  assert!(display.pixels_per_inch() > 0);
}

fn has_the_size_and_density_it_was_given(_mtm: MainThreadMarker) {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  display.set_width_in_pixels(2560);
  display.set_height_in_pixels(1600);
  display.set_pixels_per_inch(220);

  assert_eq!(display.width_in_pixels(), 2560);
  assert_eq!(display.height_in_pixels(), 1600);
  assert_eq!(display.pixels_per_inch(), 220);
}

fn a_clone_is_the_same_object(_mtm: MainThreadMarker) {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  display.clone().set_pixels_per_inch(220);

  assert_eq!(display.pixels_per_inch(), 220);
}

fn debugs_as_its_description(_mtm: MainThreadMarker) {
  assert!(
    format!("{:?}", MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap())
      .starts_with("<VZMacGraphicsDisplayConfiguration: 0x")
  );
}

fn main() {
  main_thread_tests(&[
    (
      "has_the_size_and_density_it_was_made_with",
      has_the_size_and_density_it_was_made_with,
    ),
    (
      "is_made_for_a_size_in_points_on_a_screen",
      is_made_for_a_size_in_points_on_a_screen,
    ),
    (
      "has_the_size_and_density_it_was_given",
      has_the_size_and_density_it_was_given,
    ),
    ("a_clone_is_the_same_object", a_clone_is_the_same_object),
    ("debugs_as_its_description", debugs_as_its_description),
  ]);
}
