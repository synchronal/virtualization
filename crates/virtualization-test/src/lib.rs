//! What the integration suites assume, which Virtualization does not: files
//! to attach, and the values chosen for them.

use std::path::PathBuf;

use virtualization::url::Url;

/// A path in the suites' scratch directory, named for the test that uses it.
pub fn scratch_path(name: &str) -> PathBuf {
  let directory = std::env::temp_dir().join("virtualization-test");

  std::fs::create_dir_all(&directory).unwrap();

  directory.join(name)
}

/// A raw disk image: 1 MiB of zeroes.
pub fn disk_image(name: &str) -> Url {
  file(&format!("{name}.img"), &vec![0u8; 1024 * 1024])
}

/// A file with `contents`.
pub fn file(name: &str, contents: &[u8]) -> Url {
  let path = scratch_path(name);

  std::fs::write(&path, contents).unwrap();

  Url::from_file_path(path).unwrap()
}

/// A file URL for `path`, which need not exist.
pub fn file_url(path: &str) -> Url {
  Url::from_file_path(path).unwrap()
}
