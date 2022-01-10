#![doc = include_str!("../README.md")]
#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]

mod read;
mod write;
pub use crate::read::JsonReader;
pub use crate::write::JsonWriter;

/// Possible events during JSON parsing
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum JsonEvent<'a> {
    String(&'a str),
    Number(&'a str),
    Boolean(bool),
    Null,
    StartArray,
    EndArray,
    StartObject,
    EndObject,
    ObjectKey(&'a str),
    Eof,
}
