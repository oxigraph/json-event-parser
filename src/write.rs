use crate::JsonEvent;
use std::io::{Error, ErrorKind, Result, Write};
#[cfg(feature = "async-tokio")]
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// A JSON streaming writer writing to a [`Write`] implementation.
///
/// ```
/// use json_event_parser::{JsonEvent, WriterJsonSerializer};
///
/// let mut writer = WriterJsonSerializer::new(Vec::new());
/// writer.serialize_event(JsonEvent::StartObject)?;
/// writer.serialize_event(JsonEvent::ObjectKey("foo".into()))?;
/// writer.serialize_event(JsonEvent::Number("1".into()))?;
/// writer.serialize_event(JsonEvent::EndObject)?;
///
/// assert_eq!(writer.finish()?.as_slice(), b"{\"foo\":1}");
/// # std::io::Result::Ok(())
/// ```
pub struct WriterJsonSerializer<W: Write> {
  write: W,
  writer: LowLevelJsonSerializer,
}

impl<W: Write> WriterJsonSerializer<W> {
  pub const fn new(write: W) -> Self {
    Self {
      write,
      writer: LowLevelJsonSerializer::new(),
    }
  }

  pub fn serialize_event(&mut self, event: JsonEvent<'_>) -> Result<()> {
    self.writer.serialize_event(event, &mut self.write)
  }

  #[deprecated(note = "Use serialize_event() instead")]
  pub fn write_event(&mut self, event: JsonEvent<'_>) -> Result<()> {
    self.serialize_event(event)
  }

  pub fn finish(self) -> Result<W> {
    self.writer.validate_eof()?;
    Ok(self.write)
  }
}

/// A JSON streaming writer writing to an [`AsyncWrite`] implementation.
///
/// ```
/// use json_event_parser::{JsonEvent, TokioAsyncWriterJsonSerializer};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> ::std::io::Result<()> {
/// let mut writer = TokioAsyncWriterJsonSerializer::new(Vec::new());
/// writer.serialize_event(JsonEvent::StartObject).await?;
/// writer
///     .serialize_event(JsonEvent::ObjectKey("foo".into()))
///     .await?;
/// writer
///     .serialize_event(JsonEvent::Number("1".into()))
///     .await?;
/// writer.serialize_event(JsonEvent::EndObject).await?;
/// assert_eq!(writer.finish()?.as_slice(), b"{\"foo\":1}");
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub struct TokioAsyncWriterJsonSerializer<W: AsyncWrite + Unpin> {
  write: W,
  writer: LowLevelJsonSerializer,
  buffer: Vec<u8>,
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin> TokioAsyncWriterJsonSerializer<W> {
  pub const fn new(write: W) -> Self {
    Self {
      write,
      writer: LowLevelJsonSerializer::new(),
      buffer: Vec::new(),
    }
  }

  pub async fn serialize_event(&mut self, event: JsonEvent<'_>) -> Result<()> {
    self.writer.serialize_event(event, &mut self.buffer)?;
    self.write.write_all(&self.buffer).await?;
    self.buffer.clear();
    Ok(())
  }

  #[deprecated(note = "Use serialize_event() instead")]
  pub async fn write_event(&mut self, event: JsonEvent<'_>) -> Result<()> {
    self.serialize_event(event).await
  }

  pub fn finish(self) -> Result<W> {
    self.writer.validate_eof()?;
    Ok(self.write)
  }
}

/// A low-level JSON streaming writer writing to a [`Write`] implementation.
///
/// YOu probably want to use [`WriterJsonSerializer`] instead.
///
/// ```
/// use json_event_parser::{JsonEvent, LowLevelJsonSerializer};
///
/// let mut writer = LowLevelJsonSerializer::new();
/// let mut output = Vec::new();
/// writer.serialize_event(JsonEvent::StartObject, &mut output)?;
/// writer.serialize_event(JsonEvent::ObjectKey("foo".into()), &mut output)?;
/// writer.serialize_event(JsonEvent::Number("1".into()), &mut output)?;
/// writer.serialize_event(JsonEvent::EndObject, &mut output)?;
///
/// assert_eq!(output.as_slice(), b"{\"foo\":1}");
/// # std::io::Result::Ok(())
/// ```

#[derive(Default)]
pub struct LowLevelJsonSerializer {
  state_stack: Vec<JsonState>,
  element_written: bool,
}

impl LowLevelJsonSerializer {
  pub const fn new() -> Self {
    Self {
      state_stack: Vec::new(),
      element_written: false,
    }
  }

  pub fn serialize_event(&mut self, event: JsonEvent<'_>, mut write: impl Write) -> Result<()> {
    match event {
      JsonEvent::String(s) => {
        self.before_value(&mut write)?;
        write_escaped_json_string(&s, write)
      }
      JsonEvent::Number(number) => {
        self.before_value(&mut write)?;
        write.write_all(number.as_bytes())
      }
      JsonEvent::Boolean(b) => {
        self.before_value(&mut write)?;
        write.write_all(if b { b"true" } else { b"false" })
      }
      JsonEvent::Null => {
        self.before_value(&mut write)?;
        write.write_all(b"null")
      }
      JsonEvent::StartArray => {
        self.before_value(&mut write)?;
        self.state_stack.push(JsonState::OpenArray);
        write.write_all(b"[")
      }
      JsonEvent::EndArray => match self.state_stack.pop() {
        Some(JsonState::OpenArray) | Some(JsonState::ContinuationArray) => write.write_all(b"]"),
        Some(s) => {
          self.state_stack.push(s);
          Err(Error::new(
            ErrorKind::InvalidInput,
            "Closing a not opened array",
          ))
        }
        None => Err(Error::new(
          ErrorKind::InvalidInput,
          "Closing a not opened array",
        )),
      },
      JsonEvent::StartObject => {
        self.before_value(&mut write)?;
        self.state_stack.push(JsonState::OpenObject);
        write.write_all(b"{")
      }
      JsonEvent::EndObject => match self.state_stack.pop() {
        Some(JsonState::OpenObject) | Some(JsonState::ContinuationObject) => write.write_all(b"}"),
        Some(s) => {
          self.state_stack.push(s);
          Err(Error::new(
            ErrorKind::InvalidInput,
            "Closing a not opened object",
          ))
        }
        None => Err(Error::new(
          ErrorKind::InvalidInput,
          "Closing a not opened object",
        )),
      },
      JsonEvent::ArrayIndex => Ok(()),
      JsonEvent::ObjectKey(key) => {
        match self.state_stack.pop() {
          Some(JsonState::OpenObject) => (),
          Some(JsonState::ContinuationObject) => write.write_all(b",")?,
          _ => {
            return Err(Error::new(
              ErrorKind::InvalidInput,
              "Trying to write an object key in an not object",
            ))
          }
        }
        self.state_stack.push(JsonState::ContinuationObject);
        self.state_stack.push(JsonState::ObjectValue);
        write_escaped_json_string(&key, &mut write)?;
        write.write_all(b":")
      }
      JsonEvent::Eof => Err(Error::new(
        ErrorKind::InvalidInput,
        "EOF is not allowed in JSON writer",
      )),
    }
  }

  #[deprecated(note = "Use serialize_event() instead")]
  pub fn write_event(&mut self, event: JsonEvent<'_>, write: impl Write) -> Result<()> {
    self.serialize_event(event, write)
  }

  fn before_value(&mut self, mut write: impl Write) -> Result<()> {
    match self.state_stack.pop() {
      Some(JsonState::OpenArray) => {
        self.state_stack.push(JsonState::ContinuationArray);
        Ok(())
      }
      Some(JsonState::ContinuationArray) => {
        self.state_stack.push(JsonState::ContinuationArray);
        write.write_all(b",")?;
        Ok(())
      }
      Some(last_state @ JsonState::OpenObject)
      | Some(last_state @ JsonState::ContinuationObject) => {
        self.state_stack.push(last_state);
        Err(Error::new(
          ErrorKind::InvalidInput,
          "Object key expected, string found",
        ))
      }
      Some(JsonState::ObjectValue) => Ok(()),
      None => {
        if self.element_written {
          Err(Error::new(
            ErrorKind::InvalidInput,
            "A root JSON value has already been written",
          ))
        } else {
          self.element_written = true;
          Ok(())
        }
      }
    }
  }

  fn validate_eof(&self) -> Result<()> {
    if !self.state_stack.is_empty() {
      return Err(Error::new(
        ErrorKind::InvalidInput,
        "The written JSON is not balanced: an object or an array has not been closed",
      ));
    }
    if !self.element_written {
      return Err(Error::new(
        ErrorKind::InvalidInput,
        "A JSON file can't be empty",
      ));
    }
    Ok(())
  }
}

enum JsonState {
  OpenArray,
  ContinuationArray,
  OpenObject,
  ContinuationObject,
  ObjectValue,
}

fn write_escaped_json_string(s: &str, mut write: impl Write) -> Result<()> {
  write.write_all(b"\"")?;
  let mut buffer = [b'\\', b'u', 0, 0, 0, 0];
  for c in s.chars() {
    match c {
      '\\' => write.write_all(b"\\\\"),
      '"' => write.write_all(b"\\\""),
      c => {
        if c < char::from(32) {
          match c {
            '\u{08}' => write.write_all(b"\\b"),
            '\u{0C}' => write.write_all(b"\\f"),
            '\n' => write.write_all(b"\\n"),
            '\r' => write.write_all(b"\\r"),
            '\t' => write.write_all(b"\\t"),
            c => {
              let mut c = c as u8;
              for i in (2..6).rev() {
                let ch = c % 16;
                buffer[i] = if ch < 10 { b'0' + ch } else { b'A' + ch - 10 };
                c /= 16;
              }
              write.write_all(&buffer)
            }
          }
        } else {
          write.write_all(c.encode_utf8(&mut buffer[2..]).as_bytes())
        }
      }
    }?;
  }
  write.write_all(b"\"")?;
  Ok(())
}
