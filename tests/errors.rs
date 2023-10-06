use json_event_parser::{FromBufferJsonReader, JsonEvent, ToWriteJsonWriter};

#[test]
fn test_recovery() {
    let entries = [
        (b"[nonono]".as_slice(), "[]"),
        (b"[a]", "[]"),
        (b"[1,]", "[1]"),
        (b"{\"foo\":1,}", "{\"foo\":1}"),
        (b"{\"foo\" 1}", "{\"foo\":1}"),
        (b"[1 2]", "[1,2]"),
        (b"[\"\x00\"]", "[]"),
        (b"[\"\\uD888\\u1234\"]", "[]"),
    ];

    for (input, expected_output) in entries {
        let mut reader = FromBufferJsonReader::new(input);
        let mut writer = ToWriteJsonWriter::new(Vec::new());
        loop {
            match reader.read_next_event() {
                Ok(JsonEvent::Eof) => break,
                Ok(event) => writer.write_event(event).unwrap(),
                Err(_) => (),
            }
        }
        let actual_output = String::from_utf8(writer.finish().unwrap()).unwrap();
        assert_eq!(
            expected_output,
            actual_output,
            "on {}",
            String::from_utf8_lossy(input)
        );
    }
}

#[test]
fn test_error_messages() {
    let entries = [
        (
            b"".as_slice(),
            "Parser error at line 1 column 1: Unexpected end of file, a value was expected",
        ),
        (
            b"\n}",
            "Parser error at line 2 column 1: Unexpected closing curly bracket, no array to close",
        ),
        (
            b"\r\n}",
            "Parser error at line 2 column 1: Unexpected closing curly bracket, no array to close",
        ),
        (
            b"\"\n\"",
            "Parser error at line 1 column 2: '\n' is not allowed in JSON strings",
        ),
        (
            b"\"\\uDCFF\\u0000\"",
            "Parser error at line 1 between columns 2 and column 8: \\uDCFF is not a valid high surrogate",
        )
    ];
    for (json, error) in entries {
        assert_eq!(
            FromBufferJsonReader::new(json)
                .read_next_event()
                .unwrap_err()
                .to_string(),
            error
        );
    }
}
