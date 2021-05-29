//! JSON event parser is a simple streaming JSON parser and serializer implementation.
//!
//! It does not aims to be the fastest or the more versatile JSON parser possible but to just be an as simple as possible implementation.
//!
//! Reader example:
//!
//! ```rust
//! use json_event_parser::{JsonReader, JsonEvent};
//! use std::io::Cursor;
//!
//! let json = b"{\"foo\": 1}";
//! let mut reader = JsonReader::from_reader(Cursor::new(json));
//!
//! let mut buffer = Vec::new();
//! assert_eq!(JsonEvent::StartObject, reader.read_event(&mut buffer)?);
//! assert_eq!(JsonEvent::ObjectKey("foo"), reader.read_event(&mut buffer)?);
//! assert_eq!(JsonEvent::Number("1"), reader.read_event(&mut buffer)?);
//! assert_eq!(JsonEvent::EndObject, reader.read_event(&mut buffer)?);
//! assert_eq!(JsonEvent::Eof, reader.read_event(&mut buffer)?);
//!
//! # std::io::Result::Ok(())
//! ```
//!
//! Writer example:
//!
//! ```rust
//! use json_event_parser::{JsonWriter, JsonEvent};
//!
//! let mut buffer = Vec::new();
//! let mut writer = JsonWriter::from_writer(&mut buffer);
//! writer.write_event(JsonEvent::StartObject)?;
//! writer.write_event(JsonEvent::ObjectKey("foo"))?;
//! writer.write_event(JsonEvent::Number("1"))?;
//! writer.write_event(JsonEvent::EndObject)?;
//!
//! assert_eq!(buffer.as_slice(), b"{\"foo\":1}");
//!
//! # std::io::Result::Ok(())
//! ```

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
