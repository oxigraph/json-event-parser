JSON streaming parser
=================

[![actions status](https://github.com/oxigraph/json-event-parser/workflows/build/badge.svg)](https://github.com/oxigraph/json-event-parser/actions)
[![Latest Version](https://img.shields.io/crates/v/json-event-parser.svg)](https://crates.io/crates/json-event-parser)
[![Released API docs](https://docs.rs/json-event-parser/badge.svg)](https://docs.rs/json-event-parser)

JSON event parser is a simple streaming JSON parser and serializer implementation in Rust.

It does not aims to be the fastest JSON parser possible but to be a simple implementation.

If you want fast and battle-tested code you might prefer to use [json](https://crates.io/crates/json), [serde_json](https://crates.io/crates/serde_json) or [simd-json](https://crates.io/crates/simd-json).

Reader example:

```rust
use json_event_parser::{FromReadJsonReader, JsonEvent};

let json = b"{\"foo\": 1}";
let mut reader = FromReadJsonReader::new(json.as_slice());
assert_eq!(reader.read_next_event()?, JsonEvent::StartObject);
assert_eq!(reader.read_next_event()?, JsonEvent::ObjectKey("foo".into()));
assert_eq!(reader.read_next_event()?, JsonEvent::Number("1".into()));
assert_eq!(reader.read_next_event()?, JsonEvent::EndObject);
assert_eq!(reader.read_next_event()?, JsonEvent::Eof);
# std::io::Result::Ok(())
```

Writer example:

```rust
use json_event_parser::{ToWriteJsonWriter, JsonEvent};

let mut writer = ToWriteJsonWriter::new(Vec::new());
writer.write_event(JsonEvent::StartObject)?;
writer.write_event(JsonEvent::ObjectKey("foo".into()))?;
writer.write_event(JsonEvent::Number("1".into()))?;
writer.write_event(JsonEvent::EndObject)?;

assert_eq!(writer.finish()?.as_slice(), b"{\"foo\":1}");
# std::io::Result::Ok(())
```


## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   `<http://www.apache.org/licenses/LICENSE-2.0>`)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   `<http://opensource.org/licenses/MIT>`)
   
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in json-event-parser by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
