use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LimitLimitGtd {
    pub quote_size: Option<String>,
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub end_time: Option<String>, // RFC3339 Timestamp
    pub post_only: Option<bool>,
}
