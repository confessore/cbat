use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub start: Option<String>,
    pub low: Option<String>,
    pub high: Option<String>,
    pub open: Option<String>,
    pub close: Option<String>,
    pub volume: Option<String>,
}
