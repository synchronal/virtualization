use virtualization::VirtioGraphicsScanoutConfiguration;

#[test]
fn has_the_size_it_was_made_with() {
  let scanout = VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap();

  assert_eq!(scanout.width_in_pixels(), 1920);
  assert_eq!(scanout.height_in_pixels(), 1080);
}

#[test]
fn has_the_size_it_was_given() {
  let scanout = VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap();

  scanout.set_width_in_pixels(800);
  scanout.set_height_in_pixels(600);

  assert_eq!(scanout.width_in_pixels(), 800);
  assert_eq!(scanout.height_in_pixels(), 600);
}

#[test]
fn a_clone_is_the_same_object() {
  let scanout = VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap();

  scanout.clone().set_width_in_pixels(800);

  assert_eq!(scanout.width_in_pixels(), 800);
}

#[test]
fn debugs_as_its_description() {
  assert!(
    format!("{:?}", VirtioGraphicsScanoutConfiguration::new(1920, 1080).unwrap())
      .starts_with("<VZVirtioGraphicsScanoutConfiguration: 0x")
  );
}
