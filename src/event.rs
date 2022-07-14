use chrono::{DateTime, Utc};

pub struct EventStruct {
    start_time: i64,
    duration: Option<f64>,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            start_time: 0i64,
            duration: None,
        }
    }
}

impl EventStruct {
    fn new() -> Self {
        Self {
            start_time: 0i64,
            duration: Some(0f64),
        }
    }
}
