//! Foundation's types across the bridge: `URL` and `UUID`.

use url::Url;
use uuid::Uuid;

use crate::Error;
use crate::platform::ffi;

/// `NSURL(string:)`.
pub(crate) fn ns_url(url: &Url) -> Result<ffi::Retained<ffi::NSURL>, Error> {
  ffi::ns_url_init_string(url.as_str()).ok_or_else(|| Error::UnrepresentableUrl(url.clone()))
}

/// `absoluteString`, parsed.
pub(crate) fn url(ns_url: &ffi::NSURL) -> Result<Url, Error> {
  Ok(Url::parse(&ffi::ns_url_absolute_string(ns_url))?)
}

/// `uuidString`, parsed.
pub(crate) fn uuid(ns_uuid: &ffi::NSUUID) -> Uuid {
  Uuid::parse_str(&ffi::ns_uuid_uuid_string(ns_uuid)).expect("Foundation's uuidString is a UUID")
}
