use chrono::{DateTime, Utc};

#[derive(Debug, Clone,Default)]
pub struct Timeline{
    pub start: DateTime<Utc>,
    pub end:  DateTime<Utc>,
}
