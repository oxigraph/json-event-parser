## [0.2.3] - 2026-02-06

### Changed

* Bump MSRV to 1.74
* Use lower case escape sequences in strings serialization to be closer to the JSON Canonicalization Scheme

## [0.2.2] - 2025-03-08

### Changed

* More lenient lifetime bound in `SliceJsonParser::parse_next`

## [0.2.1] - 2025-02-23

### Changed

* Some renaming to get a cleaner API (aliases with the old names are kept):
    - `FromTokioAsyncReadJsonReader` -> `TokioAsyncReaderJsonParser`
    - `FromBufferJsonReader` -> `SliceJsonParser`
    - `FromReadJsonReader` -> `ReaderJsonParser`
    - `LowLevelJsonReader` -> `LowLevelJsonParser`
    - `LowLevelJsonReaderResult` -> `LowLevelJsonParserResult`
    - `ParseError` -> `JsonParseError`
    - `SyntaxError` -> `JsonSyntaxError`
    - `ToTokioAsyncWriteJsonWriter` -> `TokioAsyncWriterJsonSerializer`
    - `ToWriteJsonWriter` -> `WriterJsonSerializer<W>`
    - `LowLevelJsonWriter` -> `LowLevelJsonSerializer`
    - `read_next_event` -> `parse_next`
    - `write_event` -> `serialize_event`

## [0.2.0] - 2024-02-23

No change compared to the alpha releases.

## [0.2.0-alpha.2] - 2023-11-13

### Changed

- Improves error messages.
- Improves ordering of tokens and errors when errors are present.
- Fixes file position in case of errors.

## [0.2.0-alpha.1] - 2023-09-23

### Added

- Support of UTF-8 byte-order-mark (BOM) during parsing.
- Support of Tokio `AsyncRead` and `AsyncWrite` interfaces behind the `async-tokio` feature.

### Changed

- The parser API has been rewritten. The new entry points are `FromBufferJsonReader`, `FromReadJsonReader`,
  and `LowLevelJsonReader`.
- The serializer API has been rewritten. The new entry points are `ToWriteJsonWriter` and `LowLevelJsonWriter`.
- The parser now returns `ParseError` and `SyntaxError` types instead of `std::io::Error`.
- Escaped unicode surrogate pairs are now carefully validated.
- Minimal supported Rust version has been bumped to 1.70.

## [0.1.1] - 2021-07-27

### Added

- Support for encoded UTF-16 surrogate pairs like `"\ud83d\udd25"`.
  The parser now complies with all [JSONTestSuite](https://github.com/nst/JSONTestSuite) positive and negative tests.

## [0.1.0] - 2021-05-30

### Added

- JSON streaming parser.
- JSON streaming serializer.
