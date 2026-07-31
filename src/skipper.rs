use crate::JsonEvent;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SkipError {
    SkipAlreadyDone,
    StackEmpty,
    NestedObjectKey,
}

impl Display for SkipError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipError::SkipAlreadyDone => write!(f, "Skip already done"),
            SkipError::StackEmpty => write!(f, "Stack is empty (trying to skip after done?)"),
            SkipError::NestedObjectKey => write!(f, "Nested object key"),
        }
    }
}

impl Error for SkipError {}

#[derive(Copy, Clone)]
pub struct Skipper {
    has_skipped_value: bool,
    depth: usize,
}

impl Default for Skipper {
    #[inline]
    fn default() -> Self {
        Skipper::new()
    }
}

impl Skipper {
    #[inline]
    pub const fn new() -> Self {
        Self {
            has_skipped_value: false,
            depth: 0,
        }
    }

    #[inline]
    pub fn skipping(&self) -> bool {
        !self.has_skipped_value || self.depth > 0
    }

    pub fn reset(&mut self) {
        self.has_skipped_value = false;
        self.depth = 0;
    }

    pub fn on_event(&mut self, event: &JsonEvent<'_>) -> Result<bool, SkipError> {
        if !self.skipping() {
            return Err(SkipError::SkipAlreadyDone);
        }

        match event {
            JsonEvent::String(_)
            | JsonEvent::Number(_)
            | JsonEvent::Boolean(_)
            | JsonEvent::Null => {
                self.has_skipped_value = true;
            }
            JsonEvent::StartArray | JsonEvent::StartObject => {
                self.has_skipped_value = true;
                self.depth += 1;
            }
            JsonEvent::EndArray | JsonEvent::EndObject => {
                if self.depth == 0 {
                    return Err(SkipError::StackEmpty);
                }
                self.depth -= 1;
            }
            JsonEvent::ObjectKey(_) => {
                if self.depth == 0 {
                    return Err(SkipError::NestedObjectKey);
                }
            }
        }

        Ok(self.skipping())
    }
}

#[cfg(test)]
mod test {
    use crate::{JsonEvent, ReaderJsonParser, Skipper};

    fn skip_test(json: &str, check_y: &str) {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum State {
            WaitScope,
            InScopeWaitX,
            InScopeSkippingWaitY,
            InScopeWaitYValue,
            WaitEndScope,
            Done,
        }

        let mut reader = ReaderJsonParser::new(json.as_bytes());
        let mut skipper = Skipper::new();
        let mut state = State::WaitScope;
        let mut y: Option<String> = None;

        while let Some(event) = reader.parse_next() {
            let event = event.unwrap();
            match event {
                JsonEvent::StartObject if state == State::WaitScope => {
                    state = State::InScopeWaitX;
                }
                JsonEvent::EndObject if state == State::WaitEndScope => {
                    state = State::Done;
                }
                JsonEvent::ObjectKey(k) if state == State::InScopeWaitX && k == "x" => {
                    state = State::InScopeSkippingWaitY;
                }
                event if state == State::InScopeSkippingWaitY && skipper.skipping() => {
                    skipper.on_event(&event).unwrap();
                }
                JsonEvent::ObjectKey(k) if state == State::InScopeSkippingWaitY && k == "y" => {
                    state = State::InScopeWaitYValue;
                }
                JsonEvent::Number(v) if state == State::InScopeWaitYValue => {
                    state = State::WaitEndScope;
                    y = Some(v.to_string());
                }
                event => {
                    panic!("unexpected event {:?} of state {:?}", event, state);
                }
            }
        }

        assert!(!skipper.skipping());
        assert_eq!(state, State::Done);
        assert_eq!(y.as_deref(), Some(check_y));
    }

    #[test]
    fn skip_single_value() {
        skip_test(
            r#"
                {
                    "x": 1,
                    "y": 2
                }
            "#,
            "2",
        );
    }

    #[test]
    fn skip_object() {
        skip_test(
            r#"
                {
                    "x": {"sub_a": true, "sub_b": "text", "sub_c": []},
                    "y": 2
                }
            "#,
            "2",
        );
    }

    #[test]
    fn skip_array() {
        skip_test(
            r#"
                {
                    "x": [1, true, "text", {}],
                    "y": 2
                }
            "#,
            "2",
        );
    }
}
