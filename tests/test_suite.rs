use json_event_parser::{JsonEvent, JsonReader, JsonWriter};
use std::fs::{read_dir, File};
use std::io::{BufReader, Read, Result};
use std::str;

#[test]
fn test_testsuite_parsing() -> Result<()> {
    for file in read_dir(format!(
        "{}/JSONTestSuite/test_parsing",
        env!("CARGO_MANIFEST_DIR")
    ))? {
        let file = file?;
        let file_name = file.file_name().to_str().unwrap().to_owned();
        if !file_name.ends_with(".json") {
            continue;
        }
        let result = parse_result(File::open(file.path())?);
        if file_name.starts_with("y_") {
            match result {
                Ok(serialization) => match parse_result(serialization.as_slice()) {
                    Ok(other_serialization) => assert_eq!(serialization, other_serialization),
                    Err(error) => panic!(
                        "Parsing of {} failed with error {}",
                        str::from_utf8(&serialization).unwrap(),
                        error
                    ),
                },
                Err(error) => panic!("Parsing of {} failed with error {}", file_name, error),
            }
        } else if file_name.starts_with("n_") {
            assert!(
                result.is_err(),
                "Parsing of {} wrongly succeeded",
                file_name
            )
        }
    }
    Ok(())
}

fn parse_result(read: impl Read) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut output_buffer = Vec::new();
    let mut reader = JsonReader::from_reader(BufReader::new(read));
    let mut writer = JsonWriter::from_writer(&mut output_buffer);
    loop {
        match reader.read_event(&mut buffer) {
            Ok(JsonEvent::Eof) => return Ok(output_buffer),
            Ok(e) => writer.write_event(e)?,
            Err(e) => return Err(e),
        }
    }
}
