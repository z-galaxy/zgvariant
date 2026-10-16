#![allow(clippy::unusual_byte_groupings)]
#![deny(rust_2018_idioms)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zgvariant`.
    allow(unused_extern_crates),
)))]
#![cfg_attr(test, recursion_limit = "256")]

#[macro_use]
mod utils;
pub use utils::*;

mod array;
pub use array::*;

mod basic;
pub use basic::*;

mod dict;
pub use dict::*;

pub mod serialized;

mod object_path;
pub use crate::object_path::*;

mod file_path;
pub use crate::file_path::*;

mod ser;
pub use ser::*;

mod de;

pub mod signature;
pub use signature::Signature;

mod str;
pub use crate::str::*;

mod structure;
pub use crate::structure::*;

mod maybe;
pub use crate::maybe::*;

mod variant;
pub use variant::*;

mod error;
pub use error::*;

#[macro_use]
mod r#type;
pub use r#type::*;

mod tuple;
pub use tuple::*;

mod from_variant;

mod into_variant;

mod owned_variant;
pub use owned_variant::*;

mod framing_offset_size;
mod framing_offsets;

mod container_depths;

pub mod as_variant;

/// Deprecated alias of [`as_variant`], kept for code written against zgvariant 1.x.
#[deprecated(since = "2.0.0", note = "renamed to `as_variant`")]
pub mod as_value {
    pub use crate::as_variant::*;
}

pub use zgvariant_derive::{
    DeserializeDict, OwnedVariant, SerializeDict, Type, Variant, signature,
};

// Kept for code written against zgvariant 1.x, where these derives were named `Value` and
// `OwnedValue`. A re-export can't carry a deprecation, so these are not marked as deprecated.
pub use zgvariant_derive::{OwnedVariant as OwnedValue, Variant as Value};

// Required for the macros to function within this crate.
extern crate self as zgvariant;

// Macro support module, not part of the public API.
#[doc(hidden)]
pub mod export {
    pub use serde;
}

// Re-export all of the `endi` API for ease of use.
pub use endi::*;
