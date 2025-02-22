use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use json_event_parser::{JsonEvent, ReaderJsonParser};
use std::fs::{self, read_dir};

fn bench_parse_json_benchmark(c: &mut Criterion) {
    for dataset in ["canada", "citm_catalog", "twitter"] {
        let data = fs::read(format!(
            "{}/benches/json-benchmark/data/{dataset}.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        c.bench_function(dataset, |b| {
            b.iter(|| {
                let mut reader = ReaderJsonParser::new(data.as_slice());
                while reader.parse_next().unwrap() != JsonEvent::Eof {
                    // read more
                }
            })
        });
    }
}

fn bench_parse_testsuite(c: &mut Criterion) {
    let example = load_testsuite_example();

    c.bench_function("JSON test suite", |b| {
        b.iter(|| {
            let mut reader = ReaderJsonParser::new(example.as_slice());
            while reader.parse_next().unwrap() != JsonEvent::Eof {
                // read more
            }
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

criterion_group!(parser, bench_parse_testsuite, bench_parse_json_benchmark);

criterion_main!(parser);
