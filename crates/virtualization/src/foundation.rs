//! Foundation's types across the bridge: `URL` and thrown errors.

use url::Url;

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

/// What a throwing call threw.
pub(crate) fn thrown(_thrown: ffi::BridgedThrown) -> Error {
  Error::NSError(Box::new((&*ffi::bridge_take_thrown_error()).into()))
}
