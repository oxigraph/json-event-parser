use crate::JsonEvent;
use std::io::{Error, ErrorKind, Result, Write};

/// A JSON streaming writer.
///
/// ```
/// use json_event_parser::{JsonWriter, JsonEvent};
///
/// let mut buffer = Vec::new();
/// let mut writer = JsonWriter::from_writer(&mut buffer);
/// writer.write_event(JsonEvent::StartObject)?;
/// writer.write_event(JsonEvent::ObjectKey("foo".into()))?;
/// writer.write_event(JsonEvent::Number("1".into()))?;
/// writer.write_event(JsonEvent::EndObject)?;
///
/// assert_eq!(buffer.as_slice(), b"{\"foo\":1}");
///
/// # std::io::Result::Ok(())
/// ```
pub struct JsonWriter<W: Write> {
    writer: W,
    state_stack: Vec<JsonState>,
    element_written: bool,
}

impl<W: Write> JsonWriter<W> {
    pub fn from_writer(writer: W) -> Self {
        Self {
            writer,
            state_stack: Vec::new(),
            element_written: false,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn inner(&mut self) -> &mut W {
        &mut self.writer
    }

    pub fn write_event(&mut self, event: JsonEvent<'_>) -> Result<()> {
        match event {
            JsonEvent::String(s) => {
                self.before_value()?;
                write_escaped_json_string(&s, &mut self.writer)
            }
            JsonEvent::Number(number) => {
                self.before_value()?;
                self.writer.write_all(number.as_bytes())
            }
            JsonEvent::Boolean(b) => {
                self.before_value()?;
                self.writer.write_all(if b { b"true" } else { b"false" })
            }
            JsonEvent::Null => {
                self.before_value()?;
                self.writer.write_all(b"null")
            }
            JsonEvent::StartArray => {
                self.before_value()?;
                self.state_stack.push(JsonState::OpenArray);
                self.writer.write_all(b"[")
            }
            JsonEvent::EndArray => match self.state_stack.pop() {
                Some(JsonState::OpenArray) | Some(JsonState::ContinuationArray) => {
                    self.writer.write_all(b"]")
                }
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
                self.before_value()?;
                self.state_stack.push(JsonState::OpenObject);
                self.writer.write_all(b"{")
            }
            JsonEvent::EndObject => match self.state_stack.pop() {
                Some(JsonState::OpenObject) | Some(JsonState::ContinuationObject) => {
                    self.writer.write_all(b"}")
                }
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
            JsonEvent::ObjectKey(key) => {
                match self.state_stack.pop() {
                    Some(JsonState::OpenObject) => (),
                    Some(JsonState::ContinuationObject) => self.writer.write_all(b",")?,
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Trying to write an object key in an not object",
                        ))
                    }
                }
                self.state_stack.push(JsonState::ContinuationObject);
                self.state_stack.push(JsonState::ObjectValue);
                write_escaped_json_string(&key, &mut self.writer)?;
                self.writer.write_all(b":")
            }
            JsonEvent::Eof => Err(Error::new(
                ErrorKind::InvalidInput,
                "EOF is not allowed in JSON writer",
            )),
        }
    }

    fn before_value(&mut self) -> Result<()> {
        match self.state_stack.pop() {
            Some(JsonState::OpenArray) => {
                self.state_stack.push(JsonState::ContinuationArray);
                Ok(())
            }
            Some(JsonState::ContinuationArray) => {
                self.state_stack.push(JsonState::ContinuationArray);
                self.writer.write_all(b",")?;
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
}

enum JsonState {
    OpenArray,
    ContinuationArray,
    OpenObject,
    ContinuationObject,
    ObjectValue,
}

fn write_escaped_json_string(s: &str, sink: &mut impl Write) -> Result<()> {
    sink.write_all(b"\"")?;
    let mut buffer = [b'\\', b'u', 0, 0, 0, 0];
    for c in s.chars() {
        match c {
            '\\' => sink.write_all(b"\\\\"),
            '"' => sink.write_all(b"\\\""),
            c => {
                if c < char::from(32) {
                    match c {
                        '\u{08}' => sink.write_all(b"\\b"),
                        '\u{0C}' => sink.write_all(b"\\f"),
                        '\n' => sink.write_all(b"\\n"),
                        '\r' => sink.write_all(b"\\r"),
                        '\t' => sink.write_all(b"\\t"),
                        c => {
                            let mut c = c as u8;
                            for i in (2..6).rev() {
                                let ch = c % 16;
                                buffer[i] = if ch < 10 { b'0' + ch } else { b'A' + ch - 10 };
                                c /= 16;
                            }
                            sink.write_all(&buffer)
                        }
                    }
                } else {
                    sink.write_all(c.encode_utf8(&mut buffer[2..]).as_bytes())
                }
            }
        }?;
    }
    sink.write_all(b"\"")?;
    Ok(())
}
