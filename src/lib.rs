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

#[cfg(feature = "async-tokio")]
pub use crate::read::FromTokioAsyncReadJsonReader;
pub use crate::read::{
    FromBufferJsonReader, FromReadJsonReader, LowLevelJsonReader, LowLevelJsonReaderResult,
    ParseError, SyntaxError, TextPosition,
};
pub use crate::write::JsonWriter;
use std::borrow::Cow;

/// Possible events during JSON parsing.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum JsonEvent<'a> {
    String(Cow<'a, str>),
    Number(Cow<'a, str>),
    Boolean(bool),
    Null,
    StartArray,
    EndArray,
    StartObject,
    EndObject,
    ObjectKey(Cow<'a, str>),
    Eof,
}
