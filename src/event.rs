use chrono::{DateTime, Duration, Local, Utc};

pub struct EventBuilder {}
impl EventBuilder {
    pub fn new() -> Event {
        let start_time = Some(Local::now());

        Event {
            start_time,
            stop_time: None,
        }
    }
}
pub struct Event {
    start_time: Option<DateTime<Local>>,
    stop_time: Option<DateTime<Local>>,
}

impl Event {
    fn stop_event(&self) {}

    fn duration(&self) -> Result<Duration, String> {
        match (self.start_time, self.stop_time) {
            (Some(start), Some(stop)) => Ok(stop - start),
            (Some(start), None) => Ok(Local::now() - start),
            _ => Err("The event does not have any set items".to_owned()),
        }
    }
}
impl Default for Event {
    fn default() -> Self {
        Self {
            start_time: None,
            stop_time: None,
        }
    }
}
