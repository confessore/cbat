use chrono::{ DateTime, Utc };
use serde_derive::Deserialize;

use crate::maintenance::Maintenance;

#[derive(Debug, Deserialize)]
pub struct FCMTradingSessionDetails {
    pub is_session_open: Option<bool>,
    pub open_time: Option<DateTime<Utc>>,
    pub close_time: Option<DateTime<Utc>>,
    pub session_state: Option<String>,
    pub after_hours_order_entry_disabled: Option<bool>,
    pub closed_reason: Option<String>,
    pub maintenance: Option<Maintenance>,
}
