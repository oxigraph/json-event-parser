## [0.2.0] - 2024-02-23

No change compared to the alpha releases.

## [0.2.0-alpha.2] - 2023-11-13

## Changed

- Improves error messages.
- Improves ordering of tokens and errors when errors are present.
- Fixes file position in case of errors.

## [0.2.0-alpha.1] - 2023-09-23

### Added

- Support of UTF-8 byte-order-mark (BOM) during parsing.
- Support of Tokio `AsyncRead` and `AsyncWrite` interfaces behind the `async-tokio` feature.

## Changed

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
