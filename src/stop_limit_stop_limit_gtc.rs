use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopLimitStopLimitGtc {
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub stop_direction: Option<String>,
}
