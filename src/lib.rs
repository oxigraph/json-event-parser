#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
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
pub use crate::read::TokioAsyncReaderJsonParser;
pub use crate::read::{
    JsonParseError, JsonSyntaxError, LowLevelJsonParser, LowLevelJsonParserResult,
    ReaderJsonParser, SliceJsonParser, TextPosition,
};
#[cfg(feature = "async-tokio")]
pub use crate::write::TokioAsyncWriterJsonSerializer;
pub use crate::write::{LowLevelJsonSerializer, WriterJsonSerializer};
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
    ArrayIndex,
    StartObject,
    EndObject,
    ObjectKey(Cow<'a, str>),
    Eof,
}

#[cfg(feature = "async-tokio")]
#[deprecated(note = "Use TokioAsyncReaderJsonParser")]
pub type FromTokioAsyncReadJsonReader<R> = TokioAsyncReaderJsonParser<R>;
#[deprecated(note = "Use SliceJsonParser")]
pub type FromBufferJsonReader<'a> = SliceJsonParser<'a>;
#[deprecated(note = "Use ReaderJsonParser")]
pub type FromReadJsonReader<R> = ReaderJsonParser<R>;
#[deprecated(note = "Use LowLevelJsonParser")]
pub type LowLevelJsonReader = LowLevelJsonParser;
#[deprecated(note = "Use LowLevelJsonParserResult")]
pub type LowLevelJsonReaderResult<'a> = LowLevelJsonParserResult<'a>;
#[deprecated(note = "Use JsonParseError")]
pub type ParseError = JsonParseError;
#[deprecated(note = "Use JsonSyntaxError")]
pub type SyntaxError = JsonSyntaxError;
#[cfg(feature = "async-tokio")]
#[deprecated(note = "Use TokioAsyncWriterJsonSerializer")]
pub type ToTokioAsyncWriteJsonWriter<W> = TokioAsyncWriterJsonSerializer<W>;
#[deprecated(note = "Use WriterJsonSerializer")]
pub type ToWriteJsonWriter<W> = WriterJsonSerializer<W>;
#[deprecated(note = "Use LowLevelJsonSerializer")]
pub type LowLevelJsonWriter = LowLevelJsonSerializer;
