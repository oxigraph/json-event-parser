#![no_main]
use json_event_parser::{JsonEvent, JsonReader};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut buffer = Vec::new();
    let mut reader = JsonReader::from_reader(data);
    while !matches!(reader.read_event(&mut buffer), Ok(JsonEvent::Eof) | Err(_)) {}
});
