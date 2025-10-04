use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditHistory {
    pub price: Option<String>,
    pub size: Option<String>,
    pub replace_accept_timestamp: Option<DateTime<Utc>>, // RFC3339 Timestamp
}
