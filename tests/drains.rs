#[cfg(test)]
mod tests {

  use json_event_parser::{JsonEvent, ReaderJsonParser};

  #[test]
  fn test_drain_next_value_as_string() -> Result<(), Box<dyn std::error::Error>> {
    let json = br#"
        {
            "skip": 123,
            "target": {
                "nested": [1, 2, {"deep": true}],
                "another": "value"
            },
            "after": false
        }
        "#;

    let mut parser = ReaderJsonParser::new(&json[..]);

    while let Ok(event) = parser.parse_next() {
      match event {
        JsonEvent::ObjectKey(key) => {
          println!("KEY: {:?}", key);
          if key == "target" {
            let raw = parser.drain_next_value_as_string()?;

            println!("brk raw: {:?}", raw);

            let expected = r#"{
                "nested": [1, 2, {"deep": true}],
                "another": "value"
            }"#;

            assert_eq!(raw, expected.to_string());
            return Ok(());
          }
          if key == "nested" || key == "another" {
            panic!("nested or another key found");
          }
        }
        _ => {}
      }
    }

    panic!("target key not found")
  }
}
