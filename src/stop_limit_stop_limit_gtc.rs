use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct StopLimitStopLimitGtc {
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub stop_direction: Option<String>,
}
