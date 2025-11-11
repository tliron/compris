// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

mod format;

/// Annotate any type.
pub mod annotate;

/// General-purpose serde deserialization using normal types as the intermediary.
#[cfg(feature = "serde")]
pub mod de;

/// Depiction.
pub mod depict;

/// Errors.
pub mod errors;

/// Hints for extending representation formats (such as XJSON).
pub mod hints;

/// Iterate key-value pairs.
pub mod kv;

/// Normal types.
pub mod normal;

/// Parse various formats into normal types.
pub mod parse;

/// Path.
pub mod path;

/// Python API.
#[cfg(feature = "pyo3")]
pub mod pyo3;

/// Resolve normal types into other types.
pub mod resolve;

/// General-purpose serde serialization with enhanced support for normal types.
#[cfg(feature = "serde")]
pub mod ser;

#[allow(unused_imports)]
pub use format::*;
