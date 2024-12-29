use chrono::{DateTime, Local};
use serde_derive::Deserialize;

use crate::maintenance::Maintenance;

#[derive(Debug, Deserialize)]
pub struct FCMTradingSessionDetails {
    pub is_session_open: Option<bool>,
    pub open_time: Option<DateTime<Local>>,
    pub close_time: Option<DateTime<Local>>,
    pub session_state: Option<String>,
    pub after_hours_order_entry_disabled: Option<bool>,
    pub closed_reason: Option<String>,
    pub maintenance: Option<Maintenance>,
}
