//! Fetching asks Apple's catalog, over the network.

use virtualization::{ErrorCode, MacOSRestoreImage};
use virtualization_test::{file, file_url, ns_error, restore_image};

#[test]
fn fetches_the_latest_supported_from_the_network() {
  let (scheme, path) = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    let url = image.unwrap().url().unwrap();

    (url.scheme().to_owned(), url.path().to_owned())
  });

  assert_eq!(scheme, "https");
  assert!(path.ends_with(".ipsw"), "{path}");
}

#[test]
fn the_latest_supported_is_supported() {
  assert!(restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    image.unwrap().is_supported()
  }));
}

#[test]
fn the_latest_supported_has_a_build_version() {
  let build_version = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    image.unwrap().build_version()
  });

  assert!(!build_version.is_empty());
}

/// The host runs macOS 27 or later, and the latest image is no older.
#[test]
fn the_latest_supported_has_an_operating_system_version_no_older_than_the_host_minimum() {
  let version = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    image.unwrap().operating_system_version()
  });

  assert!(version.major_version >= 27, "{version:?}");
}

#[test]
fn the_latest_supported_has_a_most_featureful_supported_configuration() {
  assert!(restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    image
      .unwrap()
      .most_featureful_supported_configuration()
      .is_some()
  }));
}

#[test]
fn fails_as_virtualization_does_to_load_a_file_that_is_not_a_restore_image() {
  let url = file("not-a-restore-image.ipsw", b"not a restore image");

  let error = ns_error(restore_image(
    |completion_handler| MacOSRestoreImage::load(&url, completion_handler),
    |image| image.map(|_| ()),
  ));

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::RestoreImageLoadFailed,
    "{error:#?}"
  );
}

#[test]
fn fails_as_virtualization_does_to_load_a_missing_file() {
  let url = file_url("/nonexistent/restore-image.ipsw");

  let error = ns_error(restore_image(
    |completion_handler| MacOSRestoreImage::load(&url, completion_handler),
    |image| image.map(|_| ()),
  ));

  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidRestoreImage,
    "{error:#?}"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let (clone, original) = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    let image = image.unwrap();

    (format!("{:?}", image.clone()), format!("{image:?}"))
  });

  assert_eq!(clone, original);
}

#[test]
fn debugs_as_its_description() {
  let description = restore_image(MacOSRestoreImage::fetch_latest_supported, |image| {
    format!("{:?}", image.unwrap())
  });

  assert!(description.starts_with("<VZMacOSRestoreImage: 0x"), "{description}");
}
