use crate::JsonEvent;
use std::io::{BufRead, Error, ErrorKind, Result};
use std::str;

/// A simple JSON streaming parser.
///
/// Does not allocate except a stack to check if array and object opening and closing are properly nested.
/// This stack size might be limited using the method [`max_stack_size`](JsonReader::max_stack_size).
///
/// Example:
/// ```rust
/// use json_event_parser::{JsonReader, JsonEvent};
/// use std::io::Cursor;
///
/// let json = b"{\"foo\": 1}";
/// let mut reader = JsonReader::from_reader(Cursor::new(json));
///
/// let mut buffer = Vec::new();
/// assert_eq!(JsonEvent::StartObject, reader.read_event(&mut buffer)?);
/// assert_eq!(JsonEvent::ObjectKey("foo"), reader.read_event(&mut buffer)?);
/// assert_eq!(JsonEvent::Number("1"), reader.read_event(&mut buffer)?);
/// assert_eq!(JsonEvent::EndObject, reader.read_event(&mut buffer)?);
/// assert_eq!(JsonEvent::Eof, reader.read_event(&mut buffer)?);
///
/// # std::io::Result::Ok(())
/// ```
pub struct JsonReader<R: BufRead> {
    reader: R,
    state_stack: Vec<JsonState>,
    element_read: bool,
    max_stack_size: Option<usize>,
}

impl<R: BufRead> JsonReader<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            state_stack: Vec::new(),
            element_read: false,
            max_stack_size: None,
        }
    }

    /// Maximal allowed number of nested object and array openings. Infinite by default.
    pub fn max_stack_size(&mut self, size: usize) -> &mut Self {
        self.max_stack_size = Some(size);
        self
    }

    pub fn read_event<'a>(&mut self, buffer: &'a mut Vec<u8>) -> Result<JsonEvent<'a>> {
        let front = if let Some(b) = self.lookup_front_skipping_whitespaces()? {
            b
        } else {
            return if self.state_stack.is_empty() && self.element_read {
                Ok(JsonEvent::Eof)
            } else {
                Err(Error::from(ErrorKind::UnexpectedEof))
            };
        };
        match front {
            b'{' => {
                self.reader.consume(1);
                self.check_stack_size()?;
                self.state_stack.push(JsonState::FirstObjectKey);
                Ok(JsonEvent::StartObject)
            }
            b'}' => {
                self.reader.consume(1);
                if matches!(
                    self.state_stack.pop(),
                    Some(JsonState::FirstObjectKey) | Some(JsonState::LastObjectKey)
                ) {
                    self.read_after_value(JsonEvent::EndObject)
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Closing a not opened object",
                    ))
                }
            }
            b'[' => {
                self.reader.consume(1);
                self.check_stack_size()?;
                self.state_stack.push(JsonState::FirstArray);
                Ok(JsonEvent::StartArray)
            }
            b']' => {
                self.reader.consume(1);
                if matches!(
                    self.state_stack.pop(),
                    Some(JsonState::FirstArray) | Some(JsonState::LastArray)
                ) {
                    self.read_after_value(JsonEvent::EndArray)
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Closing a not opened array",
                    ))
                }
            }
            b'"' => self.parse_string(buffer),
            b't' => self.parse_constant::<4>("true", JsonEvent::Boolean(true)),
            b'f' => self.parse_constant::<5>("false", JsonEvent::Boolean(false)),
            b'n' => self.parse_constant::<4>("null", JsonEvent::Null),
            b'-' | b'0'..=b'9' => self.parse_number(front, buffer),
            c => {
                self.reader.consume(1);
                Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected char: {}", char::from(c)),
                ))
            }
        }
    }

    fn parse_string<'a>(&mut self, output: &'a mut Vec<u8>) -> Result<JsonEvent<'a>> {
        output.clear();
        self.reader.consume(1);

        #[derive(Eq, PartialEq, Copy, Clone)]
        enum StringState {
            Default,
            Escape,
        }

        let mut state = StringState::Default;
        loop {
            match state {
                StringState::Default => {
                    let buffer = match self.reader.fill_buf() {
                        Ok(buf) => {
                            if buf.is_empty() {
                                return Err(Error::from(ErrorKind::UnexpectedEof));
                            } else {
                                buf
                            }
                        }
                        Err(e) => {
                            if e.kind() == ErrorKind::Interrupted {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    };
                    let mut i = 0;
                    for c in buffer {
                        i += 1;
                        match *c {
                            b'"' => {
                                self.reader.consume(i);
                                return self.read_after_value(JsonEvent::String(
                                    str::from_utf8(output.as_slice())
                                        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
                                ));
                            }
                            b'\\' => {
                                state = StringState::Escape;
                                break;
                            }
                            0..=0x1F => {
                                self.reader.consume(i);
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "Control characters are not allowed in JSON",
                                ));
                            }
                            c => output.push(c),
                        }
                    }
                    self.reader.consume(i);
                }
                StringState::Escape => {
                    let c = self.lookup_mandatory_front()?;
                    self.reader.consume(1);
                    match c {
                        b'"' => {
                            output.push(b'"');
                        }
                        b'\\' => {
                            output.push(b'\\');
                        }
                        b'/' => {
                            output.push(b'/');
                        }
                        b'b' => {
                            output.push(8);
                        }
                        b'f' => {
                            output.push(12);
                        }
                        b'n' => {
                            output.push(b'\n');
                        }
                        b'r' => {
                            output.push(b'\r');
                        }
                        b't' => {
                            output.push(b'\t');
                        }
                        b'u' => {
                            let mut buf = [0u8; 4];
                            self.reader.read_exact(&mut buf)?;
                            let code_point = read_hexa_char(&buf)?;
                            if let Some(c) = char::from_u32(code_point) {
                                output.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                            } else {
                                let high_surrogate = code_point;
                                let mut buf = [0u8; 6];
                                self.reader.read_exact(&mut buf)?;
                                if !buf.starts_with(b"\\u") {
                                    return Err(Error::new(
                                            ErrorKind::InvalidData,
                                            format!(
                                                "\\u{:X} is a surrogate should be followed by an other surrogate",
                                                high_surrogate
                                            ),
                                        ));
                                }
                                let low_surrogate = read_hexa_char(&buf[2..])?;
                                let code_point = 0x10000
                                    + ((high_surrogate & 0x03FF) << 10)
                                    + (low_surrogate & 0x03FF);
                                if let Some(c) = char::from_u32(code_point) {
                                    output.extend_from_slice(c.encode_utf8(&mut buf).as_bytes())
                                } else {
                                    return Err(Error::new(
                                        ErrorKind::InvalidData,
                                        format!(
                                            "\\u{:X}\\u{:X} is an invalid surrogate pair",
                                            high_surrogate, low_surrogate
                                        ),
                                    ));
                                }
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Invalid string escape",
                            ));
                        }
                    }
                    state = StringState::Default;
                }
            }
        }
    }

    fn parse_constant<'a, const SIZE: usize>(
        &mut self,
        expected: &str,
        value: JsonEvent<'a>,
    ) -> Result<JsonEvent<'a>> {
        debug_assert_eq!(expected.len(), SIZE);
        let mut buf = [0u8; SIZE];
        self.reader.read_exact(&mut buf)?;
        if buf == expected.as_bytes() {
            self.read_after_value(value)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "{} expected, found {}",
                    expected,
                    str::from_utf8(&buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                ),
            ))
        }
    }

    fn parse_number<'a>(
        &mut self,
        first_byte: u8,
        output: &'a mut Vec<u8>,
    ) -> Result<JsonEvent<'a>> {
        output.clear();
        if first_byte == b'-' {
            output.push(b'-');
            self.reader.consume(1);
        }
        // integer starting with first bytes
        // TODO: avoid too many fill_buf
        let c = self.lookup_mandatory_front()?;
        match c {
            b'0' => {
                output.push(b'0');
                self.reader.consume(1);
            }
            b'1'..=b'9' => {
                output.push(c);
                self.reader.consume(1);
                self.read_digits(output)?;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid number")),
        }

        // Dot
        if self.lookup_front()? == Some(b'.') {
            output.push(b'.');
            self.reader.consume(1);
            self.read_char(|c| c.is_ascii_digit(), output)?;
            self.read_digits(output)?;
        }

        // Exp
        if let Some(c) = self.lookup_front()? {
            if c == b'e' || c == b'E' {
                output.push(c);
                self.reader.consume(1);
                let c = self.lookup_mandatory_front()?;
                match c {
                    b'-' | b'+' => {
                        output.push(c);
                        self.reader.consume(1);
                        self.read_char(|c| c.is_ascii_digit(), output)?;
                    }
                    b'0'..=b'9' => {
                        output.push(c);
                        self.reader.consume(1);
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Invalid number. Found char {}", char::from(c)),
                        ))
                    }
                }
                self.read_digits(output)?;
            }
        }

        self.read_after_value(JsonEvent::Number(
            str::from_utf8(output.as_slice()).map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        ))
    }

    fn read_char(&mut self, valid: impl Fn(u8) -> bool, output: &mut Vec<u8>) -> Result<()> {
        let c = self.lookup_mandatory_front()?;
        if valid(c) {
            output.push(c);
            self.reader.consume(1);
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid number. Found char {}", char::from(c)),
            ))
        }
    }

    fn read_digits(&mut self, output: &mut Vec<u8>) -> Result<()> {
        while let Some(c) = self.lookup_front()? {
            if c.is_ascii_digit() {
                output.push(c);
                self.reader.consume(1);
            } else {
                break;
            }
        }
        Ok(())
    }

    fn read_after_value<'a>(&mut self, value: JsonEvent<'a>) -> Result<JsonEvent<'a>> {
        match self.state_stack.pop() {
            Some(JsonState::FirstObjectKey) | Some(JsonState::NextObjectKey) => {
                if self.lookup_front_skipping_whitespaces()? == Some(b':') {
                    self.reader.consume(1);
                    self.state_stack.push(JsonState::ObjectValue);
                    if let JsonEvent::String(value) = value {
                        Ok(JsonEvent::ObjectKey(value))
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidData,
                            "Object keys should strings",
                        ))
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Object keys should be followed by ':'",
                    ))
                }
            }
            Some(JsonState::ObjectValue) => match self.lookup_front_skipping_whitespaces()? {
                Some(b',') => {
                    self.reader.consume(1);
                    self.state_stack.push(JsonState::NextObjectKey);
                    Ok(value)
                }
                Some(b'}') => {
                    self.state_stack.push(JsonState::LastObjectKey);
                    Ok(value)
                }
                _ => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Object values should be followed by a comma or the object end",
                )),
            },
            Some(JsonState::FirstArray) | Some(JsonState::NextArray) => {
                match self.lookup_front_skipping_whitespaces()? {
                    Some(b',') => {
                        self.reader.consume(1);
                        self.state_stack.push(JsonState::NextArray);
                        Ok(value)
                    }
                    Some(b']') => {
                        self.state_stack.push(JsonState::LastArray);
                        Ok(value)
                    }
                    _ => Err(Error::new(
                        ErrorKind::InvalidData,
                        "Array values should be followed by a comma or the array end",
                    )),
                }
            }
            None => {
                if self.element_read {
                    Err(Error::new(ErrorKind::InvalidData, "JSON trailing content"))
                } else {
                    self.element_read = true;
                    Ok(value)
                }
            }
            Some(JsonState::LastObjectKey) => Err(Error::new(
                ErrorKind::InvalidData,
                "JSON object elements should be separated by commas",
            )),
            Some(JsonState::LastArray) => Err(Error::new(
                ErrorKind::InvalidData,
                "JSON array elements should be separated by commas",
            )),
        }
    }

    fn lookup_front_skipping_whitespaces(&mut self) -> Result<Option<u8>> {
        loop {
            match self.reader.fill_buf() {
                Ok(buf) => {
                    if buf.is_empty() {
                        return Ok(None);
                    }
                    let skipped = skip_whitespaces(buf);
                    if skipped == buf.len() {
                        self.reader.consume(skipped);
                    } else {
                        let result = Some(buf[skipped]);
                        self.reader.consume(skipped);
                        return Ok(result);
                    }
                }
                Err(error) => {
                    if error.kind() != ErrorKind::Interrupted {
                        return Err(error);
                    }
                }
            }
        }
    }

    fn lookup_mandatory_front(&mut self) -> Result<u8> {
        if let Some(v) = self.lookup_front()? {
            Ok(v)
        } else {
            Err(Error::from(ErrorKind::UnexpectedEof))
        }
    }

    fn lookup_front(&mut self) -> Result<Option<u8>> {
        loop {
            match self.reader.fill_buf() {
                Ok(buf) => return Ok(if buf.is_empty() { None } else { Some(buf[0]) }),
                Err(error) => {
                    if error.kind() != ErrorKind::Interrupted {
                        return Err(error);
                    }
                }
            }
        }
    }

    fn check_stack_size(&self) -> Result<()> {
        if let Some(max_stack_size) = self.max_stack_size {
            if self.state_stack.len() > max_stack_size {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Max stack size of {} reached on an object opening",
                        max_stack_size
                    ),
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum JsonState {
    FirstArray,
    NextArray,
    LastArray,
    FirstObjectKey,
    NextObjectKey,
    LastObjectKey,
    ObjectValue,
}

fn skip_whitespaces(buf: &[u8]) -> usize {
    for (i, c) in buf.iter().enumerate() {
        if !matches!(c, b' ' | b'\t' | b'\n' | b'\r') {
            return i;
        }
    }
    buf.len()
}

fn read_hexa_char(input: &[u8]) -> Result<u32> {
    let mut value = 0;
    for c in input.iter().copied() {
        value = value * 16
            + match c {
                b'0'..=b'9' => u32::from(c) - u32::from(b'0'),
                b'a'..=b'f' => u32::from(c) - u32::from(b'a') + 10,
                b'A'..=b'F' => u32::from(c) - u32::from(b'A') + 10,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Unexpected character in a unicode escape",
                    ))
                }
            }
    }
    Ok(value)
}
