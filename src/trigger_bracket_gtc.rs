use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerBracketGtc {
    pub base_size: Option<String>,
    pub limit_price: Option<String>,
    pub stop_trigger_price: Option<String>,
}
