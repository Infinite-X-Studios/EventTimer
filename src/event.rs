use chrono::{DateTime, Utc};

pub struct EventStruct {
    StartTime: DateTime<Utc>,
    Duration: f64,
}

impl Default for EventStruct {
    fn default() -> Self {
        Self {
            StartTime: DateTime::timestamp_millis(&self)
        }
    }
}

impl EventStruct {
    fn new() => Self {

    }
}
