use chrono::{ DateTime, Utc };
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Maintenance {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}
