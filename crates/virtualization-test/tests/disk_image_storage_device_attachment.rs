use virtualization::disk_image_kit::{
  ASIFCreationConfiguration, ASIFLayerCreationConfiguration, DiskImage, DiskImageBlockSize, DiskImageLayerType,
  OpenConfiguration, OpenConfigurationMode,
};
use virtualization::url::Url;
use virtualization::{
  DiskImageCachingMode, DiskImageStorageDeviceAttachment, DiskImageSynchronizationMode, ErrorCode,
  VirtioBlockDeviceConfiguration,
};
use virtualization_test::{
  booting_machine, disk_image, file, file_url, initial_ramdisk, new_file_url, ns_error, start, stop,
};

#[test]
fn attaches_a_disk_image() {
  let url = disk_image("attaches_a_disk_image");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(attachment.is_read_only());
}

#[test]
fn attaches_writable() {
  let url = disk_image("attaches_writable");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, false).unwrap();

  assert!(!attachment.is_read_only());
}

/// Apple documents `init(url:readOnly:)` as automatic caching and full
/// synchronization.
#[test]
fn has_virtualizations_modes_when_none_are_given() {
  let url = disk_image("has_virtualizations_modes_when_none_are_given");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert_eq!(attachment.caching_mode(), DiskImageCachingMode::Automatic);
  assert_eq!(attachment.synchronization_mode(), DiskImageSynchronizationMode::Full);
}

/// Every case crosses to Swift and back as itself.
#[test]
fn attaches_with_each_caching_and_synchronization_mode() {
  let url = disk_image("attaches_with_each_caching_and_synchronization_mode");
  let caching_modes = [
    DiskImageCachingMode::Automatic,
    DiskImageCachingMode::Uncached,
    DiskImageCachingMode::Cached,
  ];
  let synchronization_modes = [
    DiskImageSynchronizationMode::Full,
    DiskImageSynchronizationMode::Fsync,
    DiskImageSynchronizationMode::None,
  ];

  for caching_mode in caching_modes {
    for synchronization_mode in synchronization_modes {
      let attachment = DiskImageStorageDeviceAttachment::with_caching_mode_and_synchronization_mode(
        &url,
        false,
        caching_mode,
        synchronization_mode,
      )
      .unwrap();

      assert!(!attachment.is_read_only());
      assert_eq!(attachment.url().unwrap(), url);
      assert_eq!(attachment.caching_mode(), caching_mode);
      assert_eq!(attachment.synchronization_mode(), synchronization_mode);
    }
  }
}

#[test]
fn attaches_a_diskimagekit_image() {
  let url = new_file_url("attaches_a_diskimagekit_image.asif");
  let image =
    DiskImage::new(&ASIFCreationConfiguration::asif(&url, 2048, DiskImageBlockSize::Bytes512).unwrap()).unwrap();

  let attachment = DiskImageStorageDeviceAttachment::with_disk_image_and_caching_mode_and_synchronization_mode(
    &image,
    DiskImageCachingMode::Cached,
    DiskImageSynchronizationMode::Fsync,
  )
  .unwrap();

  assert_eq!(attachment.url().unwrap(), url);
  assert!(!attachment.is_read_only());
  assert_eq!(attachment.caching_mode(), DiskImageCachingMode::Cached);
  assert_eq!(attachment.synchronization_mode(), DiskImageSynchronizationMode::Fsync);
}

#[test]
fn attaches_a_read_only_diskimagekit_image_as_it_is() {
  let url = new_file_url("attaches_a_read_only_diskimagekit_image.asif");
  DiskImage::new(&ASIFCreationConfiguration::asif(&url, 2048, DiskImageBlockSize::Bytes512).unwrap()).unwrap();
  let image = DiskImage::with_opening(&OpenConfiguration::new(url, OpenConfigurationMode::ReadOnly)).unwrap();

  let attachment = DiskImageStorageDeviceAttachment::with_disk_image_and_caching_mode_and_synchronization_mode(
    &image,
    DiskImageCachingMode::Automatic,
    DiskImageSynchronizationMode::Full,
  )
  .unwrap();

  assert!(attachment.is_read_only());
}

/// Apple documents a stack's attachment URL as its top layer's.
#[test]
fn attaches_a_stacked_image_by_its_top_layer() {
  let base_url = new_file_url("attaches_a_stacked_image_base.asif");
  let overlay_url = new_file_url("attaches_a_stacked_image_overlay.asif");
  DiskImage::new(&ASIFCreationConfiguration::asif(&base_url, 2048, DiskImageBlockSize::Bytes512).unwrap()).unwrap();
  let base = DiskImage::with_opening(&OpenConfiguration::new(base_url, OpenConfigurationMode::ReadOnly)).unwrap();
  let overlay =
    ASIFLayerCreationConfiguration::asif_layer(&overlay_url, &DiskImageLayerType::overlay().unwrap()).unwrap();
  let stacked = base.appending_configuration(&overlay).unwrap();

  let attachment = DiskImageStorageDeviceAttachment::with_disk_image_and_caching_mode_and_synchronization_mode(
    &stacked,
    DiskImageCachingMode::Automatic,
    DiskImageSynchronizationMode::Full,
  )
  .unwrap();

  assert_eq!(attachment.url().unwrap(), overlay_url);
  assert!(!attachment.is_read_only());
}

/// Boots a guest running `script` with `image()` as `vda`, until its console
/// shows `text`; returns that line.
fn run_guest(name: &str, script: &str, image: impl FnOnce() -> DiskImage + Send, text: &str) -> String {
  let (queue, machine, mut console) = booting_machine(name, Some(&initial_ramdisk(name, script)), |configuration| {
    let attachment = DiskImageStorageDeviceAttachment::with_disk_image_and_caching_mode_and_synchronization_mode(
      &image(),
      DiskImageCachingMode::Automatic,
      DiskImageSynchronizationMode::Full,
    )
    .unwrap();

    configuration.set_storage_devices(&[&VirtioBlockDeviceConfiguration::new(&attachment).unwrap()]);
  });

  start(&queue, &machine);
  let line = console.read_until(text);
  stop(&queue, &machine);

  line.trim_end().to_owned()
}

/// Opens the image at `url` in `mode`, when called.
fn opening(url: &Url, mode: OpenConfigurationMode) -> impl FnOnce() -> DiskImage + Send + '_ {
  move || DiskImage::with_opening(&OpenConfiguration::new(url.clone(), mode)).unwrap()
}

const WRITE: &str = "printf %s \"$TEXT\" | dd of=/dev/vda conv=notrunc,fsync 2>/dev/null\necho written";
const READ: &str = "echo \"read:$(head -c 9 /dev/vda)\"";

#[test]
fn a_guest_writes_and_reads_back_a_diskimagekit_image() {
  let url = new_file_url("a_guest_writes_and_reads_back_a_diskimagekit_image.asif");
  DiskImage::new(&ASIFCreationConfiguration::asif(&url, 2048, DiskImageBlockSize::Bytes512).unwrap()).unwrap();

  run_guest(
    "a_guest_writes_a_diskimagekit_image",
    &format!("TEXT=persisted\n{WRITE}"),
    opening(&url, OpenConfigurationMode::ReadWrite),
    "written",
  );

  assert_eq!(
    run_guest(
      "a_guest_reads_back_a_diskimagekit_image",
      READ,
      opening(&url, OpenConfigurationMode::ReadOnly),
      "read:"
    ),
    "read:persisted"
  );
}

#[test]
fn a_guest_writes_a_stacked_image_to_its_top_layer() {
  let base_url = new_file_url("a_guest_writes_a_stacked_image_base.asif");
  let overlay_url = new_file_url("a_guest_writes_a_stacked_image_overlay.asif");
  DiskImage::new(&ASIFCreationConfiguration::asif(&base_url, 2048, DiskImageBlockSize::Bytes512).unwrap()).unwrap();

  run_guest(
    "a_guest_writes_a_stacked_image_base",
    &format!("TEXT=the+base!\n{WRITE}"),
    opening(&base_url, OpenConfigurationMode::ReadWrite),
    "written",
  );

  assert_eq!(
    run_guest(
      "a_guest_writes_a_stacked_image",
      &format!("{READ}\nTEXT=overlaid\n{WRITE}"),
      || {
        let overlay =
          ASIFLayerCreationConfiguration::asif_layer(&overlay_url, &DiskImageLayerType::overlay().unwrap()).unwrap();

        opening(&base_url, OpenConfigurationMode::ReadOnly)()
          .appending_configuration(&overlay)
          .unwrap()
          .into()
      },
      "read:"
    ),
    "read:the+base!"
  );

  assert_eq!(
    run_guest(
      "a_guest_reads_a_stacked_image_base",
      READ,
      opening(&base_url, OpenConfigurationMode::ReadOnly),
      "read:"
    ),
    "read:the+base!"
  );
}

#[test]
fn a_clone_is_the_same_object() {
  let url = disk_image("disk_image_storage_device_attachment_a_clone_is_the_same_object");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();
  let clone = attachment.clone();

  drop(attachment);

  assert_eq!(clone.url().unwrap(), url);
  assert!(clone.is_read_only());
}

#[test]
fn fails_as_virtualization_does_with_modes_and_no_image() {
  let url = file_url("/nowhere/disk.img");

  let error = ns_error(
    DiskImageStorageDeviceAttachment::with_caching_mode_and_synchronization_mode(
      &url,
      true,
      DiskImageCachingMode::Cached,
      DiskImageSynchronizationMode::Full,
    ),
  );

  // What macOS 27 throws: ENOTSUP, not ENOENT.
  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 45, "{error:#?}");
}

#[test]
fn fails_as_virtualization_does_when_there_is_no_image() {
  let url = file_url("/nowhere/disk.img");

  let error = ns_error(DiskImageStorageDeviceAttachment::new(&url, true));

  // What macOS 27 throws: ENOTSUP, not ENOENT.
  assert_eq!(error.domain, "NSPOSIXErrorDomain", "{error:#?}");
  assert_eq!(error.code, 45, "{error:#?}");
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("Operation not supported")
  );
  assert!(!error.localized_description.is_empty());
}

#[test]
fn fails_as_virtualization_does_on_an_unrecognized_image() {
  let url = file("fails_as_virtualization_does_on_an_unrecognized_image.img", b"abc");

  let error = ns_error(DiskImageStorageDeviceAttachment::new(&url, true));

  assert_eq!(error.domain, "VZErrorDomain", "{error:#?}");
  assert_eq!(
    ErrorCode::new(error.code).unwrap(),
    ErrorCode::InvalidDiskImage,
    "{error:#?}"
  );
  assert_eq!(
    error
      .user_info
      .get("NSLocalizedFailureReason")
      .map(String::as_str),
    Some("The disk image format is not recognized.")
  );
  assert_eq!(
    error.localized_failure_reason.as_deref(),
    Some("The disk image format is not recognized.")
  );
}

/// The thrown error is taken from Swift after the call; a second one must not
/// be confused with the first.
#[test]
fn reports_each_thrown_error_as_its_own() {
  let missing = file_url("/nowhere/disk.img");
  let unrecognized = file("reports_each_thrown_error_as_its_own.img", b"abc");

  let first = ns_error(DiskImageStorageDeviceAttachment::new(&missing, true));
  let second = ns_error(DiskImageStorageDeviceAttachment::new(&unrecognized, true));

  assert_eq!(first.domain, "NSPOSIXErrorDomain");
  assert_eq!(second.domain, "VZErrorDomain");
}

#[test]
fn debugs_as_its_description() {
  let url = disk_image("disk_image_storage_device_attachment_debugs_as_its_description");
  let attachment = DiskImageStorageDeviceAttachment::new(&url, true).unwrap();

  assert!(format!("{attachment:?}").starts_with("<VZDiskImageStorageDeviceAttachment: 0x"));
}
