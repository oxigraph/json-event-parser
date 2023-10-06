use crate::JsonEvent;
use std::borrow::Cow;
use std::cmp::{max, min};
use std::error::Error;
use std::io::{self, Read};
use std::ops::Range;
use std::{fmt, str};
#[cfg(feature = "async-tokio")]
use tokio::io::{AsyncRead, AsyncReadExt};

const MAX_STATE_STACK_SIZE: usize = 65_536;
const MIN_BUFFER_SIZE: usize = 4096;
const MAX_BUFFER_SIZE: usize = 4096 * 4096;

/// Parses a JSON file from a [`Read`] implementation.
///
///
/// ```
/// use json_event_parser::{FromReadJsonReader, JsonEvent};
///
/// let mut reader = FromReadJsonReader::new(b"{\"foo\": 1}".as_slice());
/// assert_eq!(reader.read_next_event()?, JsonEvent::StartObject);
/// assert_eq!(reader.read_next_event()?, JsonEvent::ObjectKey("foo".into()));
/// assert_eq!(reader.read_next_event()?, JsonEvent::Number("1".into()));
/// assert_eq!(reader.read_next_event()?, JsonEvent::EndObject);
/// assert_eq!(reader.read_next_event()?, JsonEvent::Eof);
/// # std::io::Result::Ok(())
/// ```
pub struct FromReadJsonReader<R: Read> {
    input_buffer: Vec<u8>,
    input_buffer_start: usize,
    input_buffer_end: usize,
    max_buffer_size: usize,
    is_ending: bool,
    read: R,
    parser: LowLevelJsonReader,
}

impl<R: Read> FromReadJsonReader<R> {
    pub const fn new(read: R) -> Self {
        Self {
            input_buffer: Vec::new(),
            input_buffer_start: 0,
            input_buffer_end: 0,
            max_buffer_size: MAX_BUFFER_SIZE,
            is_ending: false,
            read,
            parser: LowLevelJsonReader::new(),
        }
    }

    /// Sets the max size of the internal buffer in bytes
    pub fn with_max_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }

    pub fn read_next_event(&mut self) -> Result<JsonEvent<'_>, ParseError> {
        loop {
            {
                let LowLevelJsonReaderResult {
                    event,
                    consumed_bytes,
                } = self.parser.read_next_event(
                    #[allow(unsafe_code)]
                    unsafe {
                        let input_buffer_ptr: *const [u8] =
                            &self.input_buffer[self.input_buffer_start..self.input_buffer_end];
                        &*input_buffer_ptr
                    }, // SAFETY: Borrow checker workaround https://github.com/rust-lang/rust/issues/70255
                    self.is_ending,
                );
                self.input_buffer_start += consumed_bytes;
                if let Some(event) = event {
                    return Ok(event?);
                }
            }
            if self.input_buffer_start > 0 {
                self.input_buffer
                    .copy_within(self.input_buffer_start..self.input_buffer_end, 0);
                self.input_buffer_end -= self.input_buffer_start;
                self.input_buffer_start = 0;
            }
            if self.input_buffer.len() == self.max_buffer_size {
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    format!(
                        "Reached the buffer maximal size of {}",
                        self.max_buffer_size
                    ),
                )
                .into());
            }
            let min_end = min(
                self.input_buffer_end + MIN_BUFFER_SIZE,
                self.max_buffer_size,
            );
            if self.input_buffer.len() < min_end {
                self.input_buffer.resize(min_end, 0);
            }
            if self.input_buffer.len() < self.input_buffer.capacity() {
                // We keep extending to have as much space as available without reallocation
                self.input_buffer.resize(self.input_buffer.capacity(), 0);
            }
            let read = self
                .read
                .read(&mut self.input_buffer[self.input_buffer_end..])?;
            self.input_buffer_end += read;
            self.is_ending = read == 0;
        }
    }
}

/// Parses a JSON file from a [`Read`] implementation.
///
/// ```
/// use json_event_parser::{FromTokioAsyncReadJsonReader, JsonEvent};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> ::std::io::Result<()> {
/// let mut reader = FromTokioAsyncReadJsonReader::new(b"{\"foo\": 1}".as_slice());
/// assert_eq!(reader.read_next_event().await?, JsonEvent::StartObject);
/// assert_eq!(reader.read_next_event().await?, JsonEvent::ObjectKey("foo".into()));
/// assert_eq!(reader.read_next_event().await?, JsonEvent::Number("1".into()));
/// assert_eq!(reader.read_next_event().await?, JsonEvent::EndObject);
/// assert_eq!(reader.read_next_event().await?, JsonEvent::Eof);
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub struct FromTokioAsyncReadJsonReader<R: AsyncRead + Unpin> {
    input_buffer: Vec<u8>,
    input_buffer_start: usize,
    input_buffer_end: usize,
    max_buffer_size: usize,
    is_ending: bool,
    read: R,
    parser: LowLevelJsonReader,
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin> FromTokioAsyncReadJsonReader<R> {
    pub const fn new(read: R) -> Self {
        Self {
            input_buffer: Vec::new(),
            input_buffer_start: 0,
            input_buffer_end: 0,
            max_buffer_size: MAX_BUFFER_SIZE,
            is_ending: false,
            read,
            parser: LowLevelJsonReader::new(),
        }
    }

    /// Sets the max size of the internal buffer in bytes
    pub fn with_max_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }

    pub async fn read_next_event(&mut self) -> Result<JsonEvent<'_>, ParseError> {
        loop {
            {
                let LowLevelJsonReaderResult {
                    event,
                    consumed_bytes,
                } = self.parser.read_next_event(
                    #[allow(unsafe_code)]
                    unsafe {
                        let input_buffer_ptr: *const [u8] =
                            &self.input_buffer[self.input_buffer_start..self.input_buffer_end];
                        &*input_buffer_ptr
                    }, // Borrow checker workaround https://github.com/rust-lang/rust/issues/70255
                    self.is_ending,
                );
                self.input_buffer_start += consumed_bytes;
                if let Some(event) = event {
                    return Ok(event?);
                }
            }
            if self.input_buffer_start > 0 {
                self.input_buffer
                    .copy_within(self.input_buffer_start..self.input_buffer_end, 0);
                self.input_buffer_end -= self.input_buffer_start;
                self.input_buffer_start = 0;
            }
            if self.input_buffer.len() == self.max_buffer_size {
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    format!(
                        "Reached the buffer maximal size of {}",
                        self.max_buffer_size
                    ),
                )
                .into());
            }
            let min_end = min(
                self.input_buffer_end + MIN_BUFFER_SIZE,
                self.max_buffer_size,
            );
            if self.input_buffer.len() < min_end {
                self.input_buffer.resize(min_end, 0);
            }
            if self.input_buffer.len() < self.input_buffer.capacity() {
                // We keep extending to have as much space as available without reallocation
                self.input_buffer.resize(self.input_buffer.capacity(), 0);
            }
            let read = self
                .read
                .read(&mut self.input_buffer[self.input_buffer_end..])
                .await?;
            self.input_buffer_end += read;
            self.is_ending = read == 0;
        }
    }
}

/// Parses a JSON file from a `&[u8]`.
///
/// ```
/// use json_event_parser::{FromBufferJsonReader, JsonEvent};
///
/// let mut reader = FromBufferJsonReader::new(b"{\"foo\": 1}");
/// assert_eq!(reader.read_next_event()?, JsonEvent::StartObject);
/// assert_eq!(reader.read_next_event()?, JsonEvent::ObjectKey("foo".into()));
/// assert_eq!(reader.read_next_event()?, JsonEvent::Number("1".into()));
/// assert_eq!(reader.read_next_event()?, JsonEvent::EndObject);
/// assert_eq!(reader.read_next_event()?, JsonEvent::Eof);
/// # std::io::Result::Ok(())
/// ```
pub struct FromBufferJsonReader<'a> {
    input_buffer: &'a [u8],
    parser: LowLevelJsonReader,
}

impl<'a> FromBufferJsonReader<'a> {
    pub const fn new(buffer: &'a [u8]) -> Self {
        Self {
            input_buffer: buffer,
            parser: LowLevelJsonReader::new(),
        }
    }

    pub fn read_next_event(&mut self) -> Result<JsonEvent<'_>, SyntaxError> {
        loop {
            let LowLevelJsonReaderResult {
                event,
                consumed_bytes,
            } = self.parser.read_next_event(self.input_buffer, true);
            self.input_buffer = &self.input_buffer[consumed_bytes..];
            if let Some(event) = event {
                return event;
            }
        }
    }
}

/// A low-level JSON parser acting on a provided buffer.
///
/// Does not allocate except a stack to check if array and object opening and closing are properly nested.
/// This stack size might be limited using the method [`with_max_stack_size`](LowLevelJsonReader::with_max_stack_size).
///
/// ```
/// # use std::borrow::Cow;
/// use json_event_parser::{LowLevelJsonReader, JsonEvent, LowLevelJsonReaderResult};
///
/// let mut reader = LowLevelJsonReader::new();
/// assert!(matches!(
///     reader.read_next_event(b"{\"foo".as_slice(), false),
///     LowLevelJsonReaderResult { consumed_bytes: 1, event: Some(Ok(JsonEvent::StartObject))}
/// ));
/// assert!(matches!(
///     reader.read_next_event(b"\"foo".as_slice(), false),
///     LowLevelJsonReaderResult { consumed_bytes: 0, event: None }
/// ));
/// assert!(matches!(
///     reader.read_next_event(b"\"foo\": 1}".as_slice(), false),
///     LowLevelJsonReaderResult { consumed_bytes: 5, event: Some(Ok(JsonEvent::ObjectKey(Cow::Borrowed("foo")))) }
/// ));
/// assert!(matches!(
///     reader.read_next_event(b": 1}".as_slice(), false),
///     LowLevelJsonReaderResult { consumed_bytes: 3, event: Some(Ok(JsonEvent::Number(Cow::Borrowed("1")))) }
/// ));
/// assert!(matches!(
///     reader.read_next_event(b"}".as_slice(), false),
///     LowLevelJsonReaderResult { consumed_bytes: 1, event: Some(Ok(JsonEvent::EndObject)) }
/// ));
/// assert!(matches!(
///     reader.read_next_event(b"".as_slice(), true),
///     LowLevelJsonReaderResult { consumed_bytes: 0, event: Some(Ok(JsonEvent::Eof)) }
/// ));
/// # std::io::Result::Ok(())
/// ```
pub struct LowLevelJsonReader {
    lexer: JsonLexer,
    state_stack: Vec<JsonState>,
    max_state_stack_size: usize,
    element_read: bool,
    buffered_event: Option<JsonEvent<'static>>,
}

impl LowLevelJsonReader {
    pub const fn new() -> Self {
        Self {
            lexer: JsonLexer {
                file_offset: 0,
                file_line: 0,
                file_start_of_last_line: 0,
                file_start_of_last_token: 0,
                is_start: true,
            },
            state_stack: Vec::new(),
            max_state_stack_size: MAX_STATE_STACK_SIZE,
            element_read: false,
            buffered_event: None,
        }
    }

    /// Maximal allowed number of nested object and array openings. Infinite by default.
    pub fn with_max_stack_size(mut self, size: usize) -> Self {
        self.max_state_stack_size = size;
        self
    }

    /// Reads a new event from the data in `input_buffer`.
    ///
    /// `is_ending` must be set to true if all the JSON data have been already consumed or are in `input_buffer`.
    pub fn read_next_event<'a>(
        &mut self,
        input_buffer: &'a [u8],
        is_ending: bool,
    ) -> LowLevelJsonReaderResult<'a> {
        if let Some(event) = self.buffered_event.take() {
            return LowLevelJsonReaderResult {
                consumed_bytes: 0,
                event: Some(Ok(event)),
            };
        }
        let start_file_offset = self.lexer.file_offset;
        while let Some(token) = self.lexer.read_next_token(
            &input_buffer[usize::try_from(self.lexer.file_offset - start_file_offset).unwrap()..],
            is_ending,
        ) {
            let consumed_bytes = (self.lexer.file_offset - start_file_offset)
                .try_into()
                .unwrap();
            match token {
                Ok(token) => {
                    let (event, error) = self.apply_new_token(token);
                    let error = error.map(|e| {
                        self.lexer.syntax_error(
                            self.lexer.file_start_of_last_token..self.lexer.file_offset,
                            e,
                        )
                    });
                    if let Some(error) = error {
                        self.buffered_event = event.map(owned_event);
                        return LowLevelJsonReaderResult {
                            consumed_bytes,
                            event: Some(Err(error)),
                        };
                    }
                    if let Some(event) = event {
                        return LowLevelJsonReaderResult {
                            consumed_bytes,
                            event: Some(Ok(event)),
                        };
                    }
                }
                Err(error) => {
                    return LowLevelJsonReaderResult {
                        consumed_bytes,
                        event: Some(Err(error)),
                    }
                }
            }
        }
        LowLevelJsonReaderResult {
            consumed_bytes: (self.lexer.file_offset - start_file_offset)
                .try_into()
                .unwrap(),
            event: if is_ending {
                self.buffered_event = Some(JsonEvent::Eof);
                Some(Err(self.lexer.syntax_error(
                    self.lexer.file_offset..self.lexer.file_offset + 1,
                    "Unexpected end of file",
                )))
            } else {
                None
            },
        }
    }

    fn apply_new_token<'a>(
        &mut self,
        token: JsonToken<'a>,
    ) -> (Option<JsonEvent<'a>>, Option<String>) {
        match self.state_stack.pop() {
            Some(JsonState::ObjectKeyOrEnd) => {
                if token == JsonToken::ClosingCurlyBracket {
                    (Some(JsonEvent::EndObject), None)
                } else {
                    if let Err(e) = self.push_state_stack(JsonState::ObjectKey) {
                        return (None, Some(e));
                    }
                    self.apply_new_token(token)
                }
            }
            Some(JsonState::ObjectKey) => {
                if token == JsonToken::ClosingCurlyBracket {
                    return (Some(JsonEvent::EndObject), Some("Trailing commas are not allowed".into()));
                }
                if let Err(e) = self.push_state_stack(JsonState::ObjectColon) {
                    return (None, Some(e));
                }
                if let JsonToken::String(key) = token {
                    (Some(JsonEvent::ObjectKey(key)), None)
                } else {
                    (None, Some("Object keys must be strings".into()))
                }
            }
            Some(JsonState::ObjectColon) => {
                if let Err(e) = self.push_state_stack(JsonState::ObjectValue) {
                    return (None, Some(e));
                }
                if token == JsonToken::Colon {
                    (None, None)
                } else {
                    let (event, _) = self.apply_new_token(token);
                    (event, Some("Object keys must be strings".into()))
                }
            }
            Some(JsonState::ObjectValue) => {
                if let Err(e) = self.push_state_stack(JsonState::ObjectCommaOrEnd) {
                    return (None, Some(e));
                }
                self.apply_new_token_for_value(token)
            }
            Some(JsonState::ObjectCommaOrEnd) => match token {
                JsonToken::Comma => {
                    (None, self.push_state_stack(JsonState::ObjectKey).err())
                }
                JsonToken::ClosingCurlyBracket => (Some(JsonEvent::EndObject), None),
                _ => (None, Some("Object values must be followed by a comma to add a new value or a curly bracket to end the object".into())),
            },
            Some(JsonState::ArrayValueOrEnd) =>{
                if token == JsonToken::ClosingSquareBracket {
                    return (Some(JsonEvent::EndArray), None);
                }
                if let Err(e) = self.push_state_stack(JsonState::ArrayValue) {
                    return (None, Some(e));
                }
                self.apply_new_token(token)
            }
            Some(JsonState::ArrayValue) => {
                if token == JsonToken::ClosingSquareBracket {
                    return (Some(JsonEvent::EndArray), Some("Trailing commas are not allowed".into()));
                }
                if let Err(e) = self.push_state_stack(JsonState::ArrayCommaOrEnd) {
                    return (None, Some(e));
                }
                self.apply_new_token_for_value(token)
            }
            Some(JsonState::ArrayCommaOrEnd) => match token {
                JsonToken::Comma => {
                    (None, self.push_state_stack(JsonState::ArrayValue).err())
                }
                JsonToken::ClosingSquareBracket => (Some(JsonEvent::EndArray), None),
                _ => {
                    let _ = self.push_state_stack(JsonState::ArrayValue); // We already have an error
                    let (event, _) = self.apply_new_token(token);
                    (event, Some("Array values must be followed by a comma to add a new value or a squared bracket to end the array".into()))
                }
            }
            None => if self.element_read {
                if token == JsonToken::Eof {
                    (Some(JsonEvent::Eof), None)
                } else {
                    (None, Some("The JSON already contains one root element".into()))
                }
            } else {
                self.element_read = true;
                self.apply_new_token_for_value(token)
            }
        }
    }

    fn apply_new_token_for_value<'a>(
        &mut self,
        token: JsonToken<'a>,
    ) -> (Option<JsonEvent<'a>>, Option<String>) {
        match token {
            JsonToken::OpeningSquareBracket => (
                Some(JsonEvent::StartArray),
                self.push_state_stack(JsonState::ArrayValueOrEnd).err(),
            ),
            JsonToken::ClosingSquareBracket => (
                None,
                Some("Unexpected closing square bracket, no array to close".into()),
            ),
            JsonToken::OpeningCurlyBracket => (
                Some(JsonEvent::StartObject),
                self.push_state_stack(JsonState::ObjectKeyOrEnd).err(),
            ),
            JsonToken::ClosingCurlyBracket => (
                None,
                Some("Unexpected closing curly bracket, no array to close".into()),
            ),
            JsonToken::Comma => (None, Some("Unexpected comma, no values to separate".into())),
            JsonToken::Colon => (None, Some("Unexpected colon, no key to follow".into())),
            JsonToken::String(string) => (Some(JsonEvent::String(string)), None),
            JsonToken::Number(number) => (Some(JsonEvent::Number(number)), None),
            JsonToken::True => (Some(JsonEvent::Boolean(true)), None),
            JsonToken::False => (Some(JsonEvent::Boolean(false)), None),
            JsonToken::Null => (Some(JsonEvent::Null), None),
            JsonToken::Eof => (
                Some(JsonEvent::Eof),
                Some("Unexpected end of file, a value was expected".into()),
            ),
        }
    }

    fn push_state_stack(&mut self, state: JsonState) -> Result<(), String> {
        self.check_stack_size()?;
        self.state_stack.push(state);
        Ok(())
    }

    fn check_stack_size(&self) -> Result<(), String> {
        if self.state_stack.len() > self.max_state_stack_size {
            Err(format!(
                "Max stack size of {} reached on an object opening",
                self.max_state_stack_size
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum JsonState {
    ObjectKey,
    ObjectKeyOrEnd,
    ObjectColon,
    ObjectValue,
    ObjectCommaOrEnd,
    ArrayValue,
    ArrayValueOrEnd,
    ArrayCommaOrEnd,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum JsonToken<'a> {
    OpeningSquareBracket, // [
    ClosingSquareBracket, // ]
    OpeningCurlyBracket,  // {
    ClosingCurlyBracket,  // }
    Comma,                // ,
    Colon,                // :
    String(Cow<'a, str>), // "..."
    Number(Cow<'a, str>), // 1.2e3
    True,                 // true
    False,                // false
    Null,                 // null
    Eof,                  // EOF
}

struct JsonLexer {
    file_offset: u64,
    file_line: u64,
    file_start_of_last_line: u64,
    file_start_of_last_token: u64,
    is_start: bool,
}

impl JsonLexer {
    fn read_next_token<'a>(
        &mut self,
        mut input_buffer: &'a [u8],
        is_ending: bool,
    ) -> Option<Result<JsonToken<'a>, SyntaxError>> {
        // We remove BOM at the beginning
        if self.is_start {
            if input_buffer.len() < 3 && !is_ending {
                return None;
            }
            self.is_start = false;
            if input_buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
                input_buffer = &input_buffer[3..];
                self.file_offset += 3;
            }
        }

        // We skip whitespaces
        let mut i = 0;
        while let Some(c) = input_buffer.get(i) {
            match *c {
                b' ' | b'\t' => {
                    i += 1;
                }
                b'\n' => {
                    i += 1;
                    self.file_line += 1;
                    self.file_start_of_last_line = self.file_offset + u64::try_from(i).unwrap();
                }
                b'\r' => {
                    i += 1;
                    if let Some(c) = input_buffer.get(i) {
                        if *c == b'\n' {
                            i += 1; // \r\n
                        }
                    } else if !is_ending {
                        // We need an extra byte to check if followed by \n
                        i -= 1;
                        self.file_offset += u64::try_from(i).unwrap();
                        return None;
                    }
                    self.file_line += 1;
                    self.file_start_of_last_line = self.file_offset + u64::try_from(i).unwrap();
                }
                _ => {
                    break;
                }
            }
        }
        self.file_offset += u64::try_from(i).unwrap();
        input_buffer = &input_buffer[i..];
        self.file_start_of_last_token = self.file_offset;

        if is_ending && input_buffer.is_empty() {
            return Some(Ok(JsonToken::Eof));
        }

        // we get the first character
        match *input_buffer.first()? {
            b'{' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::OpeningCurlyBracket))
            }
            b'}' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::ClosingCurlyBracket))
            }
            b'[' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::OpeningSquareBracket))
            }
            b']' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::ClosingSquareBracket))
            }
            b',' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::Comma))
            }
            b':' => {
                self.file_offset += 1;
                Some(Ok(JsonToken::Colon))
            }
            b'"' => self.read_string(input_buffer),
            b't' => self.read_constant(input_buffer, is_ending, "true", JsonToken::True),
            b'f' => self.read_constant(input_buffer, is_ending, "false", JsonToken::False),
            b'n' => self.read_constant(input_buffer, is_ending, "null", JsonToken::Null),
            b'-' | b'0'..=b'9' => self.read_number(input_buffer, is_ending),
            c => {
                self.file_offset += 1;
                Some(Err(self.syntax_error(
                    self.file_offset - 1..self.file_offset,
                    if c < 128 {
                        format!("Unexpected char: '{}'", char::from(c))
                    } else {
                        format!("Unexpected byte: \\x{c:X}")
                    },
                )))
            }
        }
    }

    fn read_string<'a>(
        &mut self,
        input_buffer: &'a [u8],
    ) -> Option<Result<JsonToken<'a>, SyntaxError>> {
        let mut error = None;
        let mut string: Option<(String, usize)> = None;
        let mut next_byte_offset = 1;
        loop {
            match *input_buffer.get(next_byte_offset)? {
                b'"' => {
                    // end of string
                    let result = Some(if let Some(error) = error {
                        Err(error)
                    } else if let Some((mut string, read_until)) = string {
                        if read_until < next_byte_offset {
                            let (str, e) = self.decode_utf8(
                                &input_buffer[read_until..next_byte_offset],
                                self.file_offset + u64::try_from(read_until).unwrap(),
                            );
                            error = error.or(e);
                            string.push_str(&str);
                        }
                        if let Some(error) = error {
                            Err(error)
                        } else {
                            Ok(JsonToken::String(Cow::Owned(string)))
                        }
                    } else {
                        let (string, error) = self
                            .decode_utf8(&input_buffer[1..next_byte_offset], self.file_offset + 1);
                        if let Some(error) = error {
                            Err(error)
                        } else {
                            Ok(JsonToken::String(string))
                        }
                    });
                    self.file_offset += u64::try_from(next_byte_offset).unwrap() + 1;
                    return result;
                }
                b'\\' => {
                    // Escape sequences
                    if string.is_none() {
                        string = Some((String::new(), 1))
                    }
                    let (string, read_until) = string.as_mut().unwrap();
                    if *read_until < next_byte_offset {
                        let (str, e) = self.decode_utf8(
                            &input_buffer[*read_until..next_byte_offset],
                            self.file_offset + u64::try_from(*read_until).unwrap(),
                        );
                        error = error.or(e);
                        string.push_str(&str);
                    }
                    next_byte_offset += 1;
                    match *input_buffer.get(next_byte_offset)? {
                        b'"' => {
                            string.push('"');
                            next_byte_offset += 1;
                        }
                        b'\\' => {
                            string.push('\\');
                            next_byte_offset += 1;
                        }
                        b'/' => {
                            string.push('/');
                            next_byte_offset += 1;
                        }
                        b'b' => {
                            string.push('\u{8}');
                            next_byte_offset += 1;
                        }
                        b'f' => {
                            string.push('\u{C}');
                            next_byte_offset += 1;
                        }
                        b'n' => {
                            string.push('\n');
                            next_byte_offset += 1;
                        }
                        b'r' => {
                            string.push('\r');
                            next_byte_offset += 1;
                        }
                        b't' => {
                            string.push('\t');
                            next_byte_offset += 1;
                        }
                        b'u' => {
                            next_byte_offset += 1;
                            let val = input_buffer.get(next_byte_offset..next_byte_offset + 4)?;
                            next_byte_offset += 4;
                            let code_point = match read_hexa_char(val) {
                                Ok(cp) => cp,
                                Err(e) => {
                                    error = error.or_else(|| {
                                        let pos = self.file_offset
                                            + u64::try_from(next_byte_offset).unwrap();
                                        Some(self.syntax_error(pos - 4..pos, e))
                                    });
                                    char::REPLACEMENT_CHARACTER.into()
                                }
                            };
                            if let Some(c) = char::from_u32(code_point) {
                                string.push(c);
                            } else {
                                let high_surrogate = code_point;
                                if !(0xD800..=0xDBFF).contains(&high_surrogate) {
                                    error = error.or_else(|| {
                                        let pos = self.file_offset
                                            + u64::try_from(next_byte_offset).unwrap();
                                        Some(self.syntax_error(
                                            pos - 6..pos,
                                            format!(
                                                "\\u{:X} is not a valid high surrogate",
                                                high_surrogate
                                            ),
                                        ))
                                    });
                                }
                                let val =
                                    input_buffer.get(next_byte_offset..next_byte_offset + 6)?;
                                next_byte_offset += 6;
                                if !val.starts_with(b"\\u") {
                                    error = error.or_else(|| {
                                        let pos = self.file_offset + u64::try_from(next_byte_offset).unwrap();
                                        Some(self.syntax_error(
                                            pos - 6..pos,
                                            format!(
                                                "\\u{:X} is a high surrogate and should be followed by a low surrogate \\uXXXX",
                                                high_surrogate
                                            )
                                        ))
                                    });
                                }
                                let low_surrogate = match read_hexa_char(&val[2..]) {
                                    Ok(cp) => cp,
                                    Err(e) => {
                                        error = error.or_else(|| {
                                            let pos = self.file_offset
                                                + u64::try_from(next_byte_offset).unwrap();
                                            Some(self.syntax_error(pos - 6..pos, e))
                                        });
                                        char::REPLACEMENT_CHARACTER.into()
                                    }
                                };
                                if !(0xDC00..=0xDFFF).contains(&low_surrogate) {
                                    error = error.or_else(|| {
                                        let pos = self.file_offset
                                            + u64::try_from(next_byte_offset).unwrap();
                                        Some(self.syntax_error(
                                            pos - 6..pos,
                                            format!(
                                                "\\u{:X} is not a valid low surrogate",
                                                low_surrogate
                                            ),
                                        ))
                                    });
                                }
                                let code_point = 0x10000
                                    + ((high_surrogate & 0x03FF) << 10)
                                    + (low_surrogate & 0x03FF);
                                if let Some(c) = char::from_u32(code_point) {
                                    string.push(c)
                                } else {
                                    string.push(char::REPLACEMENT_CHARACTER);
                                    error = error.or_else(|| {
                                        let pos = self.file_offset
                                            + u64::try_from(next_byte_offset).unwrap();
                                        Some(self.syntax_error(
                                            pos - 12..pos,
                                            format!(
                                                "\\u{:X}\\u{:X} is an invalid surrogate pair",
                                                high_surrogate, low_surrogate
                                            ),
                                        ))
                                    });
                                }
                            }
                        }
                        c => {
                            next_byte_offset += 1;
                            error = error.or_else(|| {
                                let pos =
                                    self.file_offset + u64::try_from(next_byte_offset).unwrap();
                                Some(self.syntax_error(
                                    pos - 2..pos,
                                    format!("'\\{}' is not a valid escape sequence", char::from(c)),
                                ))
                            });
                            string.push(char::REPLACEMENT_CHARACTER);
                        }
                    }
                    *read_until = next_byte_offset;
                }
                c @ (0..=0x1F) => {
                    error = error.or_else(|| {
                        let pos = self.file_offset + u64::try_from(next_byte_offset).unwrap();
                        Some(self.syntax_error(
                            pos..pos + 1,
                            format!("'{}' is not allowed in JSON strings", char::from(c)),
                        ))
                    });
                    next_byte_offset += 1;
                }
                _ => {
                    next_byte_offset += 1;
                }
            }
        }
    }

    fn read_constant(
        &mut self,
        input_buffer: &[u8],
        is_ending: bool,
        expected: &str,
        value: JsonToken<'static>,
    ) -> Option<Result<JsonToken<'static>, SyntaxError>> {
        if input_buffer.get(..expected.len())? == expected.as_bytes() {
            self.file_offset += u64::try_from(expected.len()).unwrap();
            return Some(Ok(value));
        }
        let ascii_chars = input_buffer
            .iter()
            .take_while(|c| c.is_ascii_alphabetic())
            .count();
        if ascii_chars == input_buffer.len() && !is_ending {
            return None; // We might read a bigger token
        }
        let read = max(1, ascii_chars); // We want to consume at least a byte
        let start_offset = self.file_offset;
        self.file_offset += u64::try_from(read).unwrap();
        Some(Err(self.syntax_error(
            start_offset..self.file_offset,
            format!("{} expected", expected),
        )))
    }

    fn read_number<'a>(
        &mut self,
        input_buffer: &'a [u8],
        is_ending: bool,
    ) -> Option<Result<JsonToken<'a>, SyntaxError>> {
        let mut next_byte_offset = 0;
        if *input_buffer.get(next_byte_offset)? == b'-' {
            next_byte_offset += 1;
        }
        // integer starting with first bytes
        match *input_buffer.get(next_byte_offset)? {
            b'0' => {
                next_byte_offset += 1;
            }
            b'1'..=b'9' => {
                next_byte_offset += 1;
                next_byte_offset += read_digits(&input_buffer[next_byte_offset..], is_ending)?;
            }
            c => {
                next_byte_offset += 1;
                self.file_offset += u64::try_from(next_byte_offset).unwrap();
                return Some(Err(self.syntax_error(
                    self.file_offset - 1..self.file_offset,
                    format!("A number is not allowed to start with '{}'", char::from(c)),
                )));
            }
        }

        // Dot
        if input_buffer.get(next_byte_offset).map_or_else(
            || if is_ending { Some(None) } else { None },
            |c| Some(Some(*c)),
        )? == Some(b'.')
        {
            next_byte_offset += 1;
            let c = *input_buffer.get(next_byte_offset)?;
            next_byte_offset += 1;
            if !c.is_ascii_digit() {
                self.file_offset += u64::try_from(next_byte_offset).unwrap();
                return Some(Err(self.syntax_error(
                    self.file_offset - 1..self.file_offset,
                    format!(
                        "A number fractional part must start with a digit and not '{}'",
                        char::from(c)
                    ),
                )));
            }
            next_byte_offset += read_digits(&input_buffer[next_byte_offset..], is_ending)?;
        }

        // Exp
        let c = input_buffer.get(next_byte_offset).map_or_else(
            || if is_ending { Some(None) } else { None },
            |c| Some(Some(*c)),
        )?;
        if c == Some(b'e') || c == Some(b'E') {
            next_byte_offset += 1;
            match *input_buffer.get(next_byte_offset)? {
                b'-' | b'+' => {
                    next_byte_offset += 1;
                    let c = *input_buffer.get(next_byte_offset)?;
                    next_byte_offset += 1;
                    if !c.is_ascii_digit() {
                        self.file_offset += u64::try_from(next_byte_offset).unwrap();
                        return Some(Err(self.syntax_error(
                            self.file_offset - 1..self.file_offset,
                            format!(
                                "A number exponential part must contain at least a digit, '{}' found",
                                char::from(c)
                            ),
                        )));
                    }
                }
                b'0'..=b'9' => {
                    next_byte_offset += 1;
                }
                c => {
                    next_byte_offset += 1;
                    self.file_offset += u64::try_from(next_byte_offset).unwrap();
                    return Some(Err(self.syntax_error(
                        self.file_offset - 1..self.file_offset,
                        format!(
                            "A number exponential part must start with +, - or a digit, '{}' found",
                            char::from(c)
                        ),
                    )));
                }
            }
            next_byte_offset += read_digits(&input_buffer[next_byte_offset..], is_ending)?;
        }
        self.file_offset += u64::try_from(next_byte_offset).unwrap();
        Some(Ok(JsonToken::Number(Cow::Borrowed(
            str::from_utf8(&input_buffer[..next_byte_offset]).unwrap(),
        ))))
    }

    fn decode_utf8<'a>(
        &self,
        input_buffer: &'a [u8],
        start_position: u64,
    ) -> (Cow<'a, str>, Option<SyntaxError>) {
        match str::from_utf8(input_buffer) {
            Ok(str) => (Cow::Borrowed(str), None),
            Err(e) => (
                String::from_utf8_lossy(input_buffer),
                Some({
                    let pos = start_position + u64::try_from(e.valid_up_to()).unwrap();
                    self.syntax_error(pos..pos + 1, format!("Invalid UTF-8: {e}"))
                }),
            ),
        }
    }

    fn syntax_error(&self, file_offset: Range<u64>, message: impl Into<String>) -> SyntaxError {
        let start_file_offset = max(file_offset.start, self.file_start_of_last_line);
        SyntaxError {
            location: TextPosition {
                line: self.file_line,
                column: start_file_offset - self.file_start_of_last_line, //TODO: unicode
                offset: start_file_offset,
            }..TextPosition {
                line: self.file_line,
                column: file_offset.end - self.file_start_of_last_line, //TODO: unicode
                offset: file_offset.end,
            },
            message: message.into(),
        }
    }
}

fn read_hexa_char(input: &[u8]) -> Result<u32, String> {
    let mut value = 0;
    for c in input.iter().copied() {
        value = value * 16
            + match c {
                b'0'..=b'9' => u32::from(c) - u32::from(b'0'),
                b'a'..=b'f' => u32::from(c) - u32::from(b'a') + 10,
                b'A'..=b'F' => u32::from(c) - u32::from(b'A') + 10,
                _ => {
                    return Err(format!(
                        "Unexpected character in a unicode escape: '{}'",
                        char::from(c)
                    ))
                }
            }
    }
    Ok(value)
}

fn read_digits(input_buffer: &[u8], is_ending: bool) -> Option<usize> {
    let count = input_buffer
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .count();
    if count == input_buffer.len() && !is_ending {
        return None;
    }
    Some(count)
}

fn owned_event(event: JsonEvent<'_>) -> JsonEvent<'static> {
    match event {
        JsonEvent::String(s) => JsonEvent::String(s.into_owned().into()),
        JsonEvent::Number(n) => JsonEvent::Number(n.into_owned().into()),
        JsonEvent::Boolean(b) => JsonEvent::Boolean(b),
        JsonEvent::Null => JsonEvent::Null,
        JsonEvent::StartArray => JsonEvent::StartArray,
        JsonEvent::EndArray => JsonEvent::EndArray,
        JsonEvent::StartObject => JsonEvent::StartObject,
        JsonEvent::EndObject => JsonEvent::EndObject,
        JsonEvent::ObjectKey(k) => JsonEvent::ObjectKey(k.into_owned().into()),
        JsonEvent::Eof => JsonEvent::Eof,
    }
}

/// Result of [`LowLevelJsonReader::read_next_event`].
#[derive(Debug)]
pub struct LowLevelJsonReaderResult<'a> {
    /// How many bytes have been read from `input_buffer` and should be removed from it.
    pub consumed_bytes: usize,
    /// A possible new event
    pub event: Option<Result<JsonEvent<'a>, SyntaxError>>,
}

/// A position in a text i.e. a `line` number starting from 0, a `column` number starting from 0 (in number of code points) and a global file `offset` starting from 0 (in number of bytes).
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct TextPosition {
    pub line: u64,
    pub column: u64,
    pub offset: u64,
}

/// An error in the syntax of the parsed file.
///
/// It is composed of a message and a byte range in the input.
#[derive(Debug)]
pub struct SyntaxError {
    location: Range<TextPosition>,
    message: String,
}

impl SyntaxError {
    /// The location of the error inside of the file.
    #[inline]
    pub fn location(&self) -> Range<TextPosition> {
        self.location.clone()
    }

    /// The error message.
    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SyntaxError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.location.start.offset + 1 >= self.location.end.offset {
            write!(
                f,
                "Parser error at line {} column {}: {}",
                self.location.start.line + 1,
                self.location.start.column + 1,
                self.message
            )
        } else if self.location.start.line == self.location.end.line {
            write!(
                f,
                "Parser error at line {} between columns {} and column {}: {}",
                self.location.start.line + 1,
                self.location.start.column + 1,
                self.location.end.column + 1,
                self.message
            )
        } else {
            write!(
                f,
                "Parser error between line {} column {} and line {} column {}: {}",
                self.location.start.line + 1,
                self.location.start.column + 1,
                self.location.end.line + 1,
                self.location.end.column + 1,
                self.message
            )
        }
    }
}

impl Error for SyntaxError {}

impl From<SyntaxError> for io::Error {
    #[inline]
    fn from(error: SyntaxError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, error)
    }
}

/// A parsing error.
///
/// It is the union of [`SyntaxError`] and [`std::io::Error`].
#[derive(Debug)]
pub enum ParseError {
    /// I/O error during parsing (file not found...).
    Io(io::Error),
    /// An error in the file syntax.
    Syntax(SyntaxError),
}

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Syntax(e) => e.fmt(f),
        }
    }
}

impl Error for ParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Io(e) => e,
            Self::Syntax(e) => e,
        })
    }
}

impl From<SyntaxError> for ParseError {
    #[inline]
    fn from(error: SyntaxError) -> Self {
        Self::Syntax(error)
    }
}

impl From<io::Error> for ParseError {
    #[inline]
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ParseError> for io::Error {
    #[inline]
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::Syntax(e) => e.into(),
            ParseError::Io(e) => e,
        }
    }
}
