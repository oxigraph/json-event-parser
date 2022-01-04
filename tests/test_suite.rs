use json_event_parser::{JsonEvent, JsonReader, JsonWriter};
use std::fs::{read_dir, File};
use std::io::{BufReader, Read, Result};
use std::{fs, str};

const OTHER_VALID_TESTS: [&str; 12] = [
    "i_number_double_huge_neg_exp.json",
    "i_number_huge_exp.json",
    "i_number_neg_int_huge_exp.json",
    "i_number_pos_double_huge_exp.json",
    "i_number_real_neg_overflow.json",
    "i_number_real_pos_overflow.json",
    "i_number_real_underflow.json",
    "i_number_too_big_neg_int.json",
    "i_number_too_big_pos_int.json",
    "i_number_very_big_negative_int.json",
    "i_structure_500_nested_arrays.json",
    "i_structure_UTF-8_BOM_empty_object.json",
];

const OTHER_INVALID_TESTS: [&str; 23] = [
    "i_object_key_lone_2nd_surrogate.json",
    "i_string_1st_surrogate_but_2nd_missing.json",
    "i_string_1st_valid_surrogate_2nd_invalid.json",
    "i_string_incomplete_surrogate_and_escape_valid.json",
    "i_string_incomplete_surrogate_pair.json",
    "i_string_incomplete_surrogates_escape_valid.json",
    "i_string_invalid_lonely_surrogate.json",
    "i_string_invalid_surrogate.json",
    "i_string_invalid_utf-8.json",
    "i_string_inverted_surrogates_U+1D11E.json",
    "i_string_iso_latin_1.json",
    "i_string_lone_second_surrogate.json",
    "i_string_lone_utf8_continuation_byte.json",
    "i_string_not_in_unicode_range.json",
    "i_string_overlong_sequence_2_bytes.json",
    "i_string_overlong_sequence_6_bytes.json",
    "i_string_overlong_sequence_6_bytes_null.json",
    "i_string_truncated-utf-8.json",
    "i_string_UTF8_surrogate_U+D800.json",
    "i_string_utf16BE_no_BOM.json",
    "i_string_utf16LE_no_BOM.json",
    "i_string_UTF-8_invalid_sequence.json",
    "i_string_UTF-16LE_with_BOM.json",
];

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
        if file_name.starts_with("y_") || OTHER_VALID_TESTS.contains(&file_name.as_ref()) {
            match result {
                Ok(serialization) => match parse_result(serialization.as_slice()) {
                    Ok(other_serialization) => assert_eq!(serialization, other_serialization),
                    Err(error) => panic!(
                        "Parsing of {} failed with error {}",
                        str::from_utf8(&serialization).unwrap(),
                        error
                    ),
                },
                Err(error) => panic!(
                    "Parsing of {} failed with error {} on {:x?}",
                    file_name,
                    error,
                    fs::read(file.path())?
                ),
            }
        } else if file_name.starts_with("n_") || OTHER_INVALID_TESTS.contains(&file_name.as_ref()) {
            if let Ok(json) = result {
                panic!(
                    "Parsing of {} wrongly succeeded with json {}",
                    file_name,
                    str::from_utf8(&json).unwrap()
                )
            }
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
