use virtualization::MacGraphicsDisplayConfiguration;

#[test]
fn has_the_size_and_density_it_was_made_with() {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  assert_eq!(display.width_in_pixels(), 1920);
  assert_eq!(display.height_in_pixels(), 1080);
  assert_eq!(display.pixels_per_inch(), 80);
}

#[test]
fn has_the_size_and_density_it_was_given() {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  display.set_width_in_pixels(2560);
  display.set_height_in_pixels(1600);
  display.set_pixels_per_inch(220);

  assert_eq!(display.width_in_pixels(), 2560);
  assert_eq!(display.height_in_pixels(), 1600);
  assert_eq!(display.pixels_per_inch(), 220);
}

#[test]
fn a_clone_is_the_same_object() {
  let display = MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap();

  display.clone().set_pixels_per_inch(220);

  assert_eq!(display.pixels_per_inch(), 220);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", MacGraphicsDisplayConfiguration::new(1920, 1080, 80).unwrap())
      .starts_with("<VZMacGraphicsDisplayConfiguration: 0x")
  );
}
