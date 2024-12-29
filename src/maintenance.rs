use chrono::{DateTime, Local};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Maintenance {
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
}
