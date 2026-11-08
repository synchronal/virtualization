use disk_image_kit::{
  ASIFCreationConfiguration, ASIFLayerCreationConfiguration, DiskImage, DiskImageBlockSize, DiskImageFormat,
  DiskImageLayerType, Error, OpenConfiguration, OpenConfigurationMode, RAWCreationConfiguration,
};
use url::Url;
use virtualization_test::{file, file_url, new_file_url};

/// 1 MiB, in 512-byte blocks.
const BLOCK_COUNT: isize = 2048;

fn asif(name: &str) -> (Url, DiskImage) {
  let url = new_file_url(name);
  let configuration = ASIFCreationConfiguration::asif(&url, BLOCK_COUNT, DiskImageBlockSize::Bytes512).unwrap();

  (url, DiskImage::new(&configuration).unwrap())
}

fn open(url: &Url, mode: OpenConfigurationMode) -> DiskImage {
  DiskImage::with_opening(&OpenConfiguration::new(url.clone(), mode)).unwrap()
}

#[test]
fn creates_a_raw_image() {
  let url = new_file_url("creates_a_raw_image.img");
  let image = DiskImage::new(&RAWCreationConfiguration::raw(&url, BLOCK_COUNT).unwrap()).unwrap();

  assert_eq!(image.url().unwrap(), url);
  assert_eq!(image.format(), DiskImageFormat::Raw);
  assert_eq!(image.block_count(), BLOCK_COUNT);
  assert_eq!(image.block_size(), DiskImageBlockSize::Bytes512);
  assert_eq!(image.size(), 1024 * 1024);
  assert_eq!(image.open_mode(), OpenConfigurationMode::ReadWrite);
  assert_eq!(image.layer_type(), None);
  assert_eq!(image.layer_uuid(), None, "RAW images have no layer UUID");
  assert_eq!(image.parent_uuid(), None);
}

#[test]
fn creates_an_asif_image_with_each_block_size() {
  for (block_size, size) in [
    (DiskImageBlockSize::Bytes512, 1024 * 1024),
    (DiskImageBlockSize::Bytes4096, 8 * 1024 * 1024),
  ] {
    let url = new_file_url(&format!("creates_an_asif_image_{block_size:?}.asif"));
    let image = DiskImage::new(&ASIFCreationConfiguration::asif(&url, BLOCK_COUNT, block_size).unwrap()).unwrap();

    assert_eq!(image.url().unwrap(), url);
    assert_eq!(image.format(), DiskImageFormat::Asif);
    assert_eq!(image.block_size(), block_size);
    assert_eq!(image.size(), size);
    assert!(image.layer_uuid().is_some());
    assert_eq!(image.parent_uuid(), None);
  }
}

#[test]
fn rejects_a_block_count_that_is_not_positive() {
  let zero = RAWCreationConfiguration::raw(&new_file_url("zero.img"), 0).unwrap();
  let negative =
    ASIFCreationConfiguration::asif(&new_file_url("negative.asif"), -1, DiskImageBlockSize::Bytes512).unwrap();

  for result in [DiskImage::new(&zero), DiskImage::new(&negative)] {
    let Err(Error::InvalidBlockCount(error)) = result else {
      panic!("created an image without blocks: {result:?}");
    };

    assert_eq!(
      error.error_description.as_deref(),
      Some("Block count must be greater than zero")
    );
  }
}

#[test]
fn opens_an_image_in_each_mode() {
  let (url, created) = asif("opens_an_image_in_each_mode.asif");

  for (mode, open_mode) in [
    (OpenConfigurationMode::Automatic, OpenConfigurationMode::ReadWrite),
    (OpenConfigurationMode::ReadOnly, OpenConfigurationMode::ReadOnly),
    (OpenConfigurationMode::ReadWrite, OpenConfigurationMode::ReadWrite),
  ] {
    let image = open(&url, mode);

    assert_eq!(image.url().unwrap(), url);
    assert_eq!(image.format(), DiskImageFormat::Asif);
    assert_eq!(image.block_count(), BLOCK_COUNT);
    assert_eq!(image.open_mode(), open_mode);
    assert_eq!(image.layer_uuid(), created.layer_uuid());
  }
}

#[test]
fn fails_as_diskimagekit_does_when_there_is_no_image() {
  let result = DiskImage::with_opening(&OpenConfiguration::new(
    file_url("/nowhere/disk.asif"),
    OpenConfigurationMode::Automatic,
  ));

  let Err(Error::NSError(error)) = result else {
    panic!("opened an image that does not exist: {result:?}");
  };

  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 2, "{error:#?}");
}

#[test]
fn fails_as_diskimagekit_does_on_an_unrecognized_image() {
  let result = DiskImage::with_opening(&OpenConfiguration::new(
    file("fails_as_diskimagekit_does_on_an_unrecognized_image.asif", b"abc"),
    OpenConfigurationMode::Automatic,
  ));

  let Err(Error::UnsupportedFormat(error)) = result else {
    panic!("opened an image DiskImageKit can't read: {result:?}");
  };

  assert_eq!(
    error.error_description.as_deref(),
    Some("The disk image format isn't supported")
  );
  assert_eq!(error.underlying_error.domain, "com.apple.DiskImages2.ErrorDomain");
  assert_eq!(error.underlying_error.code, 161);
}

#[test]
fn stacks_a_cache_layer_on_a_base_image() {
  let (base_url, base) = asif("stacks_a_cache_layer_base.asif");
  let base_uuid = base.layer_uuid();
  let cache_url = new_file_url("stacks_a_cache_layer_cache.asif");
  let cache = ASIFLayerCreationConfiguration::asif_layer(&cache_url, &DiskImageLayerType::cache().unwrap()).unwrap();

  let stacked = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_configuration(&cache)
    .unwrap();
  let layers = stacked.layers();

  assert_eq!(stacked.format(), DiskImageFormat::Stack);
  assert_eq!(stacked.url().unwrap(), cache_url, "a stack's URL is its top layer's");
  assert_eq!(stacked.block_count(), BLOCK_COUNT);
  assert_eq!(stacked.parent_uuid(), base_uuid);
  assert_eq!(layers.len(), 2);
  assert_eq!(layers[0].url().unwrap(), base_url);
  assert_eq!(layers[0].layer_type(), None);
  assert_eq!(layers[1].url().unwrap(), cache_url);
  assert_eq!(layers[1].layer_type(), Some(DiskImageLayerType::cache().unwrap()));
  assert_eq!(layers[1].parent_uuid(), base_uuid);
}

#[test]
fn sizes_a_stack_by_an_overlay_with_its_own_block_count() {
  let (base_url, _base) = asif("sizes_a_stack_base.asif");
  let overlay_url = new_file_url("sizes_a_stack_overlay.asif");
  let overlay =
    ASIFCreationConfiguration::layer(&overlay_url, &DiskImageLayerType::overlay_block_count(8).unwrap()).unwrap();

  let stacked = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_configuration(&overlay)
    .unwrap();

  assert_eq!(stacked.block_count(), 8);
  assert_eq!(stacked.size(), 8 * 512);
  assert_eq!(stacked.open_mode(), OpenConfigurationMode::ReadWrite);
}

#[test]
fn appends_to_a_stack() {
  let (base_url, _base) = asif("appends_to_a_stack_base.asif");
  let cache = ASIFLayerCreationConfiguration::asif_layer(
    &new_file_url("appends_to_a_stack_cache.asif"),
    &DiskImageLayerType::cache().unwrap(),
  )
  .unwrap();
  let overlay = ASIFLayerCreationConfiguration::asif_layer(
    &new_file_url("appends_to_a_stack_overlay.asif"),
    &DiskImageLayerType::overlay().unwrap(),
  )
  .unwrap();

  let stacked = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_configuration(&cache)
    .unwrap();
  let stacked = DiskImage::from(stacked)
    .appending_configuration(&overlay)
    .unwrap();

  assert_eq!(stacked.layers().len(), 3);
  assert_eq!(
    stacked.layers()[2].layer_type(),
    Some(DiskImageLayerType::overlay().unwrap())
  );
}

/// The overlay was made over this base, so its parent UUID is the base's.
#[test]
fn appends_an_existing_layer() {
  let (base_url, _base) = asif("appends_an_existing_layer_base.asif");
  let overlay_url = new_file_url("appends_an_existing_layer_overlay.asif");
  let overlay =
    ASIFLayerCreationConfiguration::asif_layer(&overlay_url, &DiskImageLayerType::overlay().unwrap()).unwrap();
  drop(
    open(&base_url, OpenConfigurationMode::ReadOnly)
      .appending_configuration(&overlay)
      .unwrap(),
  );

  let stacked = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_layer(open(&overlay_url, OpenConfigurationMode::ReadWrite))
    .unwrap();

  assert_eq!(stacked.format(), DiskImageFormat::Stack);
  assert_eq!(stacked.layers().len(), 2);
  assert_eq!(stacked.url().unwrap(), overlay_url);
}

#[test]
fn fails_as_diskimagekit_does_on_an_unrelated_layer() {
  let (base_url, _base) = asif("an_unrelated_layer_base.asif");
  let (layer_url, _layer) = asif("an_unrelated_layer.asif");

  let result = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_layer(open(&layer_url, OpenConfigurationMode::ReadWrite));

  let Err(Error::IncompatibleStacking(error)) = result else {
    panic!("stacked an unrelated image: {result:?}");
  };

  assert_eq!(
    error.reason,
    "Cannot add a layer without a parent UUID above a layer with a layer UUID"
  );
  assert_eq!(
    error.error_description.as_deref(),
    Some("The appended layer isn't compatible with the existing stack")
  );
}

#[test]
fn fails_as_diskimagekit_does_on_a_second_cache_layer() {
  let (base_url, _base) = asif("a_second_cache_layer_base.asif");
  let cache = DiskImageLayerType::cache().unwrap();
  let first = ASIFLayerCreationConfiguration::asif_layer(&new_file_url("a_second_cache_layer_1.asif"), &cache).unwrap();
  let second =
    ASIFLayerCreationConfiguration::asif_layer(&new_file_url("a_second_cache_layer_2.asif"), &cache).unwrap();

  let stacked = open(&base_url, OpenConfigurationMode::ReadOnly)
    .appending_configuration(&first)
    .unwrap();
  let result = DiskImage::from(stacked).appending_configuration(&second);

  let Err(Error::IncompatibleStacking(error)) = result else {
    panic!("stacked a second cache layer: {result:?}");
  };

  assert_eq!(error.reason, "Too many cache layers");
}

#[test]
fn truncates() {
  let url = new_file_url("truncates.img");
  let image = DiskImage::new(&RAWCreationConfiguration::raw(&url, BLOCK_COUNT).unwrap()).unwrap();

  image.truncate(2 * BLOCK_COUNT).unwrap();

  assert_eq!(image.block_count(), 2 * BLOCK_COUNT);
  assert_eq!(image.size(), 2 * 1024 * 1024);
  assert!(matches!(image.truncate(0), Err(Error::InvalidBlockCount(_))));
}

/// As DiskImageKit compares them: an overlay's block count doesn't count.
#[test]
fn compares_layer_types_as_diskimagekit_does() {
  let cache = DiskImageLayerType::cache().unwrap();
  let overlay = DiskImageLayerType::overlay().unwrap();

  assert_eq!(cache, DiskImageLayerType::cache().unwrap());
  assert_ne!(cache, overlay);
  assert_eq!(overlay, DiskImageLayerType::overlay_block_count(8).unwrap());
}

#[test]
fn a_clone_is_the_same_object() {
  let (url, image) = asif("disk_image_a_clone_is_the_same_object.asif");
  let clone = image.clone();

  drop(image);

  assert_eq!(clone.url().unwrap(), url);
}

#[test]
fn debugs_as_its_description() {
  let (_url, image) = asif("disk_image_debugs_as_its_description.asif");
  let configuration = ASIFCreationConfiguration::asif(
    &new_file_url("disk_image_debugs_as_its_description_2.asif"),
    BLOCK_COUNT,
    DiskImageBlockSize::Bytes512,
  )
  .unwrap();

  assert_eq!(format!("{image:?}"), "DiskImageKit.DiskImage");
  assert!(format!("{configuration:?}").starts_with("ASIFCreationConfiguration("));
}
