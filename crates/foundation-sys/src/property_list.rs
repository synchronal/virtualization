//! A property list, as plain Rust.

use std::collections::BTreeMap;

/// A value of a property list, such as an IOKit matching dictionary: what
/// Swift's `any Sendable` holds in a `[String: any Sendable]` that Apple
/// reads as one. IOKit's property lists have no dates, so neither has this.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyListValue {
  /// A `Bool`, which Foundation holds as an `NSNumber`.
  Boolean(bool),
  /// An integer `NSNumber`.
  Integer(i64),
  /// A floating-point `NSNumber`.
  Real(f64),
  /// A `String`.
  String(String),
  /// A `Data`, as its bytes.
  Data(Vec<u8>),
  /// An `Array`.
  Array(Vec<PropertyListValue>),
  /// A `[String: Any]`.
  Dictionary(BTreeMap<String, PropertyListValue>),
}
