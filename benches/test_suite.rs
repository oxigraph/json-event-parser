use criterion::{criterion_group, criterion_main, Criterion};
use json_event_parser::{JsonEvent, JsonReader};
use std::fs::{read_dir, File};
use std::io::{Cursor, Read};

fn bench_json_parse(c: &mut Criterion) {
    let examples = load_testsuite_examples();

    c.bench_function("JSON test suite", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            for json in examples.iter() {
                let mut reader = JsonReader::from_reader(Cursor::new(json));
                while reader.read_event(&mut buffer).unwrap() != JsonEvent::Eof {
                    //read more
                }
            }
        })
    });
}

fn bench_serde_json_parse(c: &mut Criterion) {
    let examples = load_testsuite_examples();

    c.bench_function("Serde JSON test suite", |b| {
        b.iter(|| {
            for json in examples.iter() {
                let _: serde_json::Value = serde_json::from_reader(Cursor::new(json)).unwrap();
            }
        })
    });
}

fn load_testsuite_examples() -> Vec<Vec<u8>> {
    let blacklist = vec![
        "y_string_accepted_surrogate_pair.json",
        "y_string_accepted_surrogate_pairs.json",
        "y_string_last_surrogates_1_and_2.json",
        "y_string_unicode_U+1FFFE_nonchar.json",
        "y_string_unicode_U+10FFFE_nonchar.json",
        "y_string_surrogates_U+1D11E_MUSICAL_SYMBOL_G_CLEF.json",
    ];
    read_dir(format!(
        "{}/JSONTestSuite/test_parsing",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
    .filter_map(|file| {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_owned();
        if file_name.starts_with("y_")
            && file_name.ends_with(".json")
            && !blacklist.contains(&file_name.as_str())
        {
            let mut buf = Vec::new();
            File::open(file.path())
                .unwrap()
                .read_to_end(&mut buf)
                .unwrap();
            Some(buf)
        } else {
            None
        }
    })
    .collect()
}

criterion_group!(parser, bench_json_parse, bench_serde_json_parse);

criterion_main!(parser);
