use criterion::{criterion_group, criterion_main, Criterion};
use json_event_parser::{FromReadJsonReader, JsonEvent};
use std::fs::{self, read_dir};

fn bench_json_parse(c: &mut Criterion) {
    let example = load_testsuite_example();

    c.bench_function("JSON test suite", |b| {
        b.iter(|| {
            let mut reader = FromReadJsonReader::new(example.as_slice());
            while reader.read_next_event().unwrap() != JsonEvent::Eof {
                //read more
            }
        })
    });
}

fn bench_serde_json_parse(c: &mut Criterion) {
    let example = load_testsuite_example();

    c.bench_function("Serde JSON test suite", |b| {
        b.iter(|| {
            let _: serde_json::Value = serde_json::from_reader(example.as_slice()).unwrap();
        })
    });
}

fn load_testsuite_example() -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(b"[\n");
    for file in read_dir(format!(
        "{}/JSONTestSuite/test_parsing",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
    {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_owned();
        if file_name.starts_with("y_") && file_name.ends_with(".json") {
            if result.len() > 2 {
                result.extend_from_slice(b",\n");
            }
            result.push(b'\t');
            result.extend_from_slice(&fs::read(file.path()).unwrap());
        }
    }
    result.extend_from_slice(b"\n]");
    result
}

criterion_group!(parser, bench_json_parse, bench_serde_json_parse);

criterion_main!(parser);
