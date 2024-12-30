use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct StopLimitStopLimitGtd {
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub end_time: Option<String>, // RFC3339 Timestamp
    pub stop_direction: Option<String>,
}
